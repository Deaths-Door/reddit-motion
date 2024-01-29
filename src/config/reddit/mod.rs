mod login;
mod theme;
mod subreddit;
mod utils;

use futures::{stream::FuturesUnordered, StreamExt};
pub(in crate::config::reddit) use utils::*;

use serde::{Deserialize,Serialize};
use crate::{db::Database, video_generator::VideoGenerator};

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
    pub async fn exceute(&self,args : &VideoCreationArguments<'_>,db : &mut Database) -> Result<(),VideoCreationError> {
        if let Some(user) = &self.user {
            match !user.login_and_set_theme(args.browser).await? {
                true => args.call_login_successful(),
                false => args.call_invalid_reddit_credentials(),
            }
        }

        let mut tasks = FuturesUnordered::new();
    
        for subreddit in &self.subreddits {
            args.call_on_new_subreddit(&subreddit.name);

            subreddit.exceute(args,db,|files,video_length_limit| {
                let gen = VideoGenerator::new(files,args,video_length_limit);
                tasks.push(gen.exceute())
            }).await;

            args.call_on_end_subreddit();
        }

        while let Some(task_result) = tasks.next().await {
            args.call_on_video_finished(task_result);
        }
        
        Ok(())
    }
}