use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;

use super::{StoryMode, TextToSpeechService};

#[derive(Serialize, Deserialize)]
pub struct RedditConfig {
    credentials : Option<RedditUser>,
    subreddits : Vec<SubredditConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct RedditUser {
    username : String,
    password : String,
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