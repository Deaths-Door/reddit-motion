mod assets;
mod dimensions;
mod reddit;
mod tts;
mod story_mode;
mod translate;
mod callback;
mod args;
mod error;
mod duration;

pub use assets::*;
pub use dimensions::*;
pub use reddit::*;
pub use tts::*;
pub use story_mode::*;
pub use translate::*;
pub use callback::*;
pub use args::*;
pub use error::*;
pub use duration::*;

use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;
use chromiumoxide::{Browser, handler::viewport::Viewport, BrowserConfig};
use futures::StreamExt;
use tokio::task::JoinHandle;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "DisplayFromStr")]
    pub lang: LanguageIdentifier,
    pub assets: Assets,
    pub dimensions: Dimensions,
    
    #[serde(default)]
    pub tts : TextToSpeechService,

    reddit: RedditConfig,

    #[serde(default)]
    translate : TranslationServices
}

use std::path::Path;

use crate::{ffmpeg::FFmpeg, db::Database};

impl Config {
    pub fn from_file<P>(path : P) -> anyhow::Result<Self> where P : AsRef<Path> {
        let toml = std::fs::read_to_string(path)?;
        let config = toml::from_str::<Config>(&toml)?;

        config.tts.setup();
        
        Ok(config)
    }

    pub async fn exceute_create_videos(
        self,
        ffmpeg : FFmpeg,
        db : &mut Database,
        callback : &Callback
    ) -> anyhow::Result<JoinHandle<()>> {
        let (browser,handler) = self.create_browser().await?;

        let args = VideoCreationArguments::new(callback, &browser, self, ffmpeg);

        args.config.reddit.exceute(&args,db).await?;

        Ok(handler)
    }

    async fn create_browser(&self) -> anyhow::Result<(Browser,JoinHandle<()>)> {
        let (width,height) = self.dimensions.width_height();
    
        // Device scale factor (or dsf for short) allows us to increase the resolution of the screenshots
        // When the dsf is 1, the width of the screenshot is 600 pixels
        // so we need a dsf such that the width of the screenshot is greater than the final resolution of the video
        let device_scale_factor = (width / 600) + 1;
        
        let viewport = Viewport {
            width, height,
            device_scale_factor :  Some(device_scale_factor as f64),
            ..Default::default()
        };
    
        let browser_config = BrowserConfig::builder()
            .viewport(viewport)
            .with_head() // Just for debuggin purposes
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to open browser due to error. ({e})"))?;
    
        let (browser,mut handler) = Browser::launch(browser_config).await?;
    
        let handle = tokio::task::spawn(async move {
            loop {
                match handler.next().await {
                    Some(h) => match h {
                        Ok(_) => continue,
                        Err(_) => break,
                    },
                    None => break,
                }
            }
        });    
    
        Ok((browser,handle))
    }
}