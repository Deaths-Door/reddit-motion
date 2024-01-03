mod assets;
mod dimensions;
mod reddit;
mod tts;
mod story_mode;
mod translate;

pub use assets::*;
pub use dimensions::*;
pub use reddit::*;
pub use tts::*;
pub use story_mode::*;
pub use translate::*;

use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "DisplayFromStr")]
    lang: LanguageIdentifier,
    dimensions: Dimensions,
    reddit: RedditConfig,
    assets: Assets,
    tts : TextToSpeech,
    translate : TranslationServices
}