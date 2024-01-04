mod login;
mod theme;

use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;

use super::{StoryMode, TextToSpeechService, VideoCreationArguments, VideoCreationError};

#[derive(Serialize, Deserialize)]
pub struct RedditConfig {
    #[serde(flatten)]
    user : Option<RedditUser>,
    subreddits : Vec<SubredditConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct RedditUser {
    username : String,
    password : String,
    #[serde(default)]
    use_dark_mode : bool
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SubredditConfig {
    name : String,

    #[serde(default = "drepeat")]
    repeat_count : u8,

    #[serde(default,rename = "mode")]
    story_mode : StoryMode,

    #[serde(default)]
    for_tts_use : TextToSpeechService,

    #[serde_as(as = "Vec<DisplayFromStr>")]
    extra_langs: Vec<LanguageIdentifier>,
}

fn drepeat() -> u8 { 1 }

impl RedditConfig {
    pub async fn exceute(&self,args : &VideoCreationArguments<'_>) -> Result<(),VideoCreationError> {
        let callback = args.callback;

        if let Some(user) = &self.user {
            match !user.login_and_set_theme(args.browser).await? {
                true => args.call_invalid_reddit_credentials(),
                false => args.call_login_successful(),
            }
        }

        for subreddit in &self.subreddits {
            args.call_on_new_subreddit(&subreddit.name);

            //TODO

            args.call_on_end_subreddit();
        }

        Ok(())
    }
}

pub(in crate::config::reddit) async fn wait_for(secs : u64) {
    tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
}