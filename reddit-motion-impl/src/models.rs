//! https://stackoverflow.com/a/66413710

#![allow(unused)]

use crate::schema::languages;
use crate::schema::subreddit_entries;
use crate::schema::subreddit_entry_languages;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(lang_code))]
#[diesel(table_name = languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Language {
    pub lang_code: String,
}

#[derive(Queryable, Debug, Identifiable, Selectable)]
#[diesel(primary_key(post_id))]
#[diesel(table_name = subreddit_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct SubredditEntry {
    pub post_id: String,
    pub subreddit_name: String,
    pub submission_data: Vec<u8>,
}

#[derive(Queryable, Debug, Identifiable, Selectable, Associations)]
#[diesel(primary_key(post_id, lang_code))]
#[diesel(belongs_to(SubredditEntry,foreign_key = post_id))]
#[diesel(table_name = subreddit_entry_languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct SubredditEntryLanguage {
    pub post_id: String,
    pub lang_code: String,
}

use diesel::sql_types::{Blob, Text};

// https://github.com/diesel-rs/diesel/issues/860#issuecomment-633910627
#[derive(QueryableByName, Queryable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct EntryWithLanguages {
    #[sql_type = "Text"]
    pub post_id: String,

    #[sql_type = "Text"]
    pub subreddit_name: String,

    #[sql_type = "Blob"]
    pub submission_data: Vec<u8>, // Assuming BLOB is used for binary data

    #[sql_type = "Text"]
    pub languages: String, // The result of `json_group_array` as text
}

// Struct for the bulk insert into subreddit_entry_languages
#[derive(Insertable)]
#[diesel(table_name = subreddit_entry_languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct NewSubredditEntryLanguage<'a> {
    pub post_id: &'a str,
    pub lang_code: std::borrow::Cow<'a, str>,
}

#[derive(Insertable)]
#[diesel(table_name = subreddit_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct NewSubredditEntry<'a> {
    pub post_id: &'a str,
    pub subreddit_name: &'a str,
    pub submission_data: Vec<u8>,
}
