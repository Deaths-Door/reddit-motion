use diesel::prelude::*;
use reddit_motion_core::{roux::submission::SubmissionData, SubredditTaskEntry};

use crate::{
    models::{NewSubredditEntry, NewSubredditEntryLanguage},
    schema::{
        subreddit_entries::{self, post_id as post_id_dsl, submission_data, subreddit_name},
        subreddit_entry_languages::{
            self, dsl::subreddit_entry_languages as subreddit_entry_languages_table,
        },
    },
    CachingEntriesError, DatabaseError, RedditTaskError, SubmissionDataConversionError,
    SubredditIndividualTask,
};

impl SubredditIndividualTask<'_> {
    pub(super) async fn internal_cache_entries<'a, 'l>(
        &'a mut self,
        entries: Vec<SubredditTaskEntry<'l>>,
    ) -> Result<(), RedditTaskError> {
        let mut connection = self.resources.database_connection.lock().await;

        let output = connection.transaction(|connection| {
            let transformed_iter =
                entries.iter().filter_map(|entry| {
                    match submission_entry_to_blob(entry.submission()) {
                        Ok(submission) => {
                            let post_id: &str = entry.submission().id.as_str();
                            let new_entry = NewSubredditEntry {
                                post_id: post_id,
                                subreddit_name: entry.submission().name.as_str(),
                                submission_data: submission,
                            };

                            let associated_languages: Vec<_> = entry
                                .extra_languages()
                                .iter()
                                .map(|language_identifier| NewSubredditEntryLanguage {
                                    post_id,
                                    lang_code: language_identifier.to_string().into(),
                                })
                                .collect();

                            Some((new_entry, associated_languages))
                        }
                        Err(error) => None,
                    }
                });

            let (entry_values, entry_language_values): (
                Vec<NewSubredditEntry<'_>>,
                Vec<Vec<NewSubredditEntryLanguage<'_>>>,
            ) = transformed_iter.unzip();

            let entry_language_values: Vec<_> = entry_language_values
                .into_iter()
                .flat_map(|ele| ele)
                .collect();

            // Bulk insert into subreddit_entry_languages
            let _ = diesel::insert_or_ignore_into(subreddit_entry_languages_table)
                .values(entry_language_values)
                .execute(connection)?;

            let _ = diesel::insert_into(subreddit_entries::table)
                .values(entry_values)
                .execute(connection)?;

            Ok::<_, diesel::result::Error>(())
        });

        output.map_err(|e| {
            RedditTaskError::DatabaseError(DatabaseError::from(CachingEntriesError::from(e)))
        })
    }
}

fn submission_entry_to_blob(
    submission: &SubmissionData,
) -> Result<Vec<u8>, SubmissionDataConversionError> {
    let mut value = serde_json::to_value(submission)?;

    let object_map = value
        .as_object_mut()
        .expect("Expected a JSON object in the database entry");

    object_map.remove("subreddit").unwrap_or_else(|| {
        panic!("The 'subreddit' field is not already present, when it should be")
    });

    object_map
        .remove("id")
        .unwrap_or_else(|| panic!("The 'id' field is not salready present, when it should be"));

    Ok(serde_json::from_value(value)?)
}
