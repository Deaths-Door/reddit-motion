mod assets;
mod dimensions;
mod reddit;
mod tts;
mod story_mode;
mod translate;
mod callback;

pub use assets::*;
pub use dimensions::*;
pub use reddit::*;
pub use tts::*;
pub use story_mode::*;
pub use translate::*;
pub use callback::*;

use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "DisplayFromStr")]
    pub lang: LanguageIdentifier,
    pub(crate) assets: Assets,
    dimensions: Dimensions,
    reddit: RedditConfig,

    #[serde(default)]
    tts : TextToSpeech,

    #[serde(default)]
    translate : TranslationServices
}

use std::path::Path;

impl Config {
    pub fn from_file<P>(path : P) -> anyhow::Result<Self> where P : AsRef<Path> {
        let toml = std::fs::read_to_string(path)?;
        let config = toml::from_str::<Config>(&toml)?;
        Ok(config)
    }
}