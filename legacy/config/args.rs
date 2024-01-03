use chromiumoxide::Browser;
use roux::Subreddit;

use crate::{db::Database, callback::Callback, ffmpeg::FFmpeg};

use super::{assets::Assets, Dimesions};

pub struct ParameterArgs<'a> {
    pub assets : &'a Assets,
    pub dimensions : &'a Dimesions,
    pub db : &'a mut Database,
    pub browser : &'a Browser,
    pub callback : &'a Callback,
    pub ffmpeg : FFmpeg,
}

impl<'a> ParameterArgs<'a> {
    pub fn new(
        assets: &'a Assets,
        dimensions: &'a Dimesions, 
        db: &'a mut Database, 
        browser: &'a Browser, 
        callback: &'a Callback,
        ffmpeg : FFmpeg
    ) -> Self { Self { assets, dimensions, db, browser, callback ,ffmpeg} }
}