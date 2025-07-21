
CREATE TABLE IF NOT EXISTS  subreddit_entry_languages (
  post_id TEXT NOT NULL,
  lang_code TEXT NOT NULL,
  PRIMARY KEY (post_id, lang_code),
  FOREIGN KEY (post_id) REFERENCES subreddit_entries(post_id) ON DELETE CASCADE,
  FOREIGN KEY (lang_code) REFERENCES languages(lang_code) ON DELETE CASCADE
);