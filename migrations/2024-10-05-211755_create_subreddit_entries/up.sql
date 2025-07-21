CREATE TABLE IF NOT EXISTS  subreddit_entries (
  post_id TEXT NOT NULL PRIMARY KEY,
  subreddit_name TEXT NOT NULL,
  submission_data BLOB NOT NULL
);