mod assets;
mod reddit;
mod subreddit;
mod story_mode;
mod tts;
mod screenshot;
mod args;
mod video;

use serde::{Deserialize, Deserializer};
use serde_with::{serde_as,DisplayFromStr};
pub use story_mode::*;
pub use tts::*;
pub use subreddit::*;
use unic_langid::LanguageIdentifier;

#[serde_as]
#[derive(Deserialize)]
pub struct Config {
    pub(crate) assets : assets::Assets,

    #[serde_as(as = "DisplayFromStr")]
    pub(crate) lang : LanguageIdentifier,

    dimensions : Dimesions,
    reddit : reddit::RedditConfig
}


#[derive(serde::Deserialize)]
pub struct Dimesions {
    #[serde(default = "dwidth")]
    pub(crate) width : u32,
    #[serde(default = "dheight")]
    pub(crate) height : u32,
}

fn dwidth() -> u32 { 800 }
fn dheight() -> u32 { 600 }

#[derive(thiserror::Error, Debug)]
pub enum LoadingConfigError {
    #[error("{}",.0)]
    File(#[from] std::io::Error),

    
    #[error("{}",.0.message())]
    Toml(#[from] toml::de::Error)
}

use std::{path::Path, str::FromStr};
use chromiumoxide::{Browser,BrowserConfig,handler::viewport::Viewport};
use futures::StreamExt;
use anyhow::anyhow;

use crate::ffmpeg::FFmpeg;

use self::args::ParameterArgs;

impl Config {
    pub fn from_file<P>(path : P) -> Result<Self,LoadingConfigError> where P : AsRef<Path> {
        let toml = std::fs::read_to_string(path)?;
        let config = toml::from_str::<Config>(&toml)?;
        Ok(config)
    }

    pub async fn create_videos(&self,db : &mut crate::db::Database,ffmpeg : FFmpeg,callback : &crate::callback::Callback) -> anyhow::Result<tokio::task::JoinHandle<()>> {
        let (browser,handler) = create_browser(&self).await?;

        let mut parms = ParameterArgs::new(
            &self.assets,
            &self.dimensions,
            db,
            &browser,
            callback,
            ffmpeg
        );

        self.reddit.handle(&mut parms).await?;
        
        Ok(handler)
    }
}

pub async fn create_browser(config : &Config) -> anyhow::Result<(Browser,tokio::task::JoinHandle<()>)> {
    let (width,height) = (config.dimensions.width,config.dimensions.height);

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
        .with_head()
        .build()
        .map_err(|e| anyhow!("Failed to open browser due to error. ({e})"))?;

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