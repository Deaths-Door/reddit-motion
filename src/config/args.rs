use chromiumoxide::Browser;

use crate::ffmpeg::FFmpeg;
use super::{Config, Callback};

pub struct VideoCreationArguments<'a> {
    pub config : &'a Config,
    pub callback : &'a Callback,
    pub ffmpeg : &'a FFmpeg,
    pub browser : &'a Browser
}

impl<'a> VideoCreationArguments<'a> {
    pub fn new(
        config: &'a Config, 
        callback: &'a Callback, 
        ffmpeg: &'a FFmpeg, 
        browser: &'a Browser
    ) -> Self { Self { config, callback, ffmpeg, browser } }
}