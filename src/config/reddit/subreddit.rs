use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;

use crate::config::{StoryMode, TextToSpeechService, VideoCreationArguments, VideoCreationError};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SubredditConfig {
    pub(super) name : String,

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

impl SubredditConfig {
    pub async fn exceute(&self,args : &VideoCreationArguments<'_>) -> Result<(),VideoCreationError> {

    }
}
