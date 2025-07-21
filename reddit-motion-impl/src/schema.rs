// @generated automatically by Diesel CLI.

diesel::table! {
    languages (lang_code) {
        lang_code -> Text,
    }
}

diesel::table! {
    subreddit_entries (post_id) {
        post_id -> Text,
        subreddit_name -> Text,
        submission_data -> Binary,
    }
}

diesel::table! {
    subreddit_entry_languages (post_id, lang_code) {
        post_id -> Text,
        lang_code -> Text,
    }
}

diesel::joinable!(subreddit_entry_languages -> languages (lang_code));
diesel::joinable!(subreddit_entry_languages -> subreddit_entries (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    languages,
    subreddit_entries,
    subreddit_entry_languages,
);
