use std::{borrow::Cow, ops::DerefMut};

use diesel::prelude::*;

use reddit_motion_core::{
    roux::{submission::SubmissionData, Subreddit},
    SubredditGenerationTask, SubredditTask, SubredditTaskEntry,
};

use crate::{
    models::{EntryWithLanguages, SubredditEntry},
    schema::subreddit_entries,
    DatabaseConnection, DatabaseError, RedditTaskError, RetrievingCachedEntriesError,
    SubmissionDataConversionError, SubredditIndividualTask, ToSubredditTaskEntryConversionError,
};

struct PrivateError(RedditTaskError);

impl SubredditIndividualTask<'_> {
    #[allow(unused_doc_comments)]
    pub(super) async fn internal_get_cached_entries<'a, 'l>(
        &'a mut self,
        subreddit: &Subreddit,
        wanted_count: u8,
    ) -> Result<Vec<SubredditTaskEntry<'l>>, RedditTaskError> {
        let mut connection = self.resources.database_connection.lock().await;

        let entries: Result<Vec<SubredditTaskEntry<'_>>, PrivateError> =
            connection.deref_mut().transaction(|connection| {
                let subreddit_name = subreddit.name.as_str();

                let iter = DatabaseConnection::retrieve_cache_entries_query(
                    subreddit_name,
                    wanted_count.into(),
                    connection,
                )?;

                // **NOTE**
                //   It's important to note that individual errors from the iterator
                //   should **never terminate the enclosing function**. Instead, all successful
                //   task creations should be reported and collected, allowing the function to continue
                //   processing subsequent elements even if some fail.
                let entries = iter.fold(Vec::new(), |mut tasks, result| {
                    match result
                        .map_err(|e| {
                            RedditTaskError::DatabaseError(DatabaseError::RetrievingCachedEntries(
                                RetrievingCachedEntriesError(e),
                            ))
                        })
                        .and_then(|element| {
                            SubredditTaskEntry::try_from(element).map_err(|e| {
                                RedditTaskError::DatabaseError(DatabaseError::Conversion(
                                    crate::DataConversionError::SubredditTaskEntry(e),
                                ))
                            })
                        }) {
                        Ok(element) => tasks.push(element),
                        Err(error) => self.report_creating_task_failed(error),
                    }

                    tasks
                });

                let post_ids = entries.iter().map(|task| task.submission().id.as_str());

                diesel::delete(subreddit_entries::table)
                    .filter(subreddit_entries::post_id.eq_any(post_ids))
                    .execute(connection)
                    .map_err(|e| {
                        RedditTaskError::DatabaseError(DatabaseError::RetrievingCachedEntries(
                            RetrievingCachedEntriesError::from(e),
                        ))
                    })?;

                Ok(entries)
            });

        entries.map_err(|e| e.0)
    }
}

impl DatabaseConnection {
    fn retrieve_cache_entries_query<'a>(
        subreddit_name: &'a str,
        limit: i32,
        conn: &'a mut SqliteConnection,
    ) -> Result<impl Iterator<Item = QueryResult<EntryWithLanguages>> + 'a, diesel::result::Error>
    {
        const QUERY: &str = r#"    
SELECT e.post_id,
       e.subreddit_name,
        e.submission_data,
       json_group_array(l.lang_code) AS languages
FROM subreddit_entries e
JOIN subreddit_entry_languages l ON e.post_id = l.post_id
WHERE e.subreddit_name = ?
GROUP BY e.post_id, e.subreddit_name
LIMIT ?
        "#;

        diesel::sql_query(QUERY)
            .bind::<diesel::sql_types::Text, _>(subreddit_name)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load_iter::<EntryWithLanguages, _>(conn)
    }
}

impl TryFrom<EntryWithLanguages> for SubredditTaskEntry<'_> {
    type Error = ToSubredditTaskEntryConversionError;
    fn try_from(value: EntryWithLanguages) -> Result<Self, Self::Error> {
        let entry = SubredditEntry {
            post_id: value.post_id,
            submission_data: value.submission_data,
            subreddit_name: value.subreddit_name,
        };

        let submission_data = SubmissionData::try_from(entry)
            .map_err(|e| ToSubredditTaskEntryConversionError::from(e.0))?;
        let extra_languages = Cow::Owned(serde_json::from_str(&value.languages)?);

        Ok(Self::new(submission_data, extra_languages))
    }
}

impl TryFrom<SubredditEntry> for SubmissionData {
    type Error = SubmissionDataConversionError;
    fn try_from(entry: SubredditEntry) -> Result<Self, Self::Error> {
        let mut value =
            serde_json::from_slice::<serde_json::Value>(entry.submission_data.as_slice())?;
        let object_map = value
            .as_object_mut()
            .expect("Expected a JSON object in the database entry");

        object_map
            .insert("subreddit".into(), entry.subreddit_name.into())
            .unwrap_or_else(|| {
                panic!("The 'subreddit' field is already present, when it should not be")
            });

        object_map
            .insert("id".into(), entry.post_id.into())
            .unwrap_or_else(|| panic!("The 'id' field is already present, when it should not be"));

        Ok(serde_json::from_value(value)?)
    }
}

impl From<RedditTaskError> for PrivateError {
    fn from(value: RedditTaskError) -> Self {
        Self(value)
    }
}

impl From<diesel::result::Error> for PrivateError {
    fn from(value: diesel::result::Error) -> Self {
        Self::from(RedditTaskError::from(RetrievingCachedEntriesError::from(
            value,
        )))
    }
}
