mod login;
mod theme;
mod subreddit;
mod utils;

pub(in crate::config::reddit) use utils::*;

use serde::{Deserialize,Serialize};
use super::{VideoCreationArguments, VideoCreationError};

#[derive(Serialize, Deserialize)]
pub struct RedditConfig {
    #[serde(flatten)]
    user : Option<RedditUser>,
    subreddits : Vec<subreddit::SubredditConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct RedditUser {
    username : String,
    password : String,
    #[serde(default)]
    use_dark_mode : bool
}

impl RedditConfig {
    pub async fn exceute(&self,args : &VideoCreationArguments<'_>) -> Result<(),VideoCreationError> {
        if let Some(user) = &self.user {
            match !user.login_and_set_theme(args.browser).await? {
                true => args.call_invalid_reddit_credentials(),
                false => args.call_login_successful(),
            }
        }

        // TODO : MAYBE CHECK THE BIN DIR FOR THREADS NOT FINISHED?
        for subreddit in &self.subreddits {
            args.call_on_new_subreddit(&subreddit.name);

            subreddit.exceute(args).await?;

            args.call_on_end_subreddit();
        }

        Ok(())
    }
}