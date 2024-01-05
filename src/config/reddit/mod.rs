mod login;
mod theme;
mod subreddit;

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

        for subreddit in &self.subreddits {
            args.call_on_new_subreddit(&subreddit.name);

            match subreddit.exceute(args).await {
                Ok(_) => args.call_on_end_subreddit(),
                Err(err)=> args.call_on_skipping_due_to_error(err),
            }
        }

        Ok(())
    }
}

pub(in crate::config::reddit) async fn wait_for(secs : u64) {
    tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
}