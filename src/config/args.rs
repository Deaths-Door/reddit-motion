use std::error::Error;

use chromiumoxide::Browser;

use crate::ffmpeg::FFmpeg;
use super::{Config, Callback};

pub struct VideoCreationArguments<'a> {
    pub config : &'a Config,
    pub ffmpeg : &'a FFmpeg,
    pub browser : &'a Browser,

    callback : &'a Callback,
}

impl<'a> VideoCreationArguments<'a> {
    pub fn new(
        config: &'a Config, 
        callback: &'a Callback, 
        ffmpeg: &'a FFmpeg, 
        browser: &'a Browser
    ) -> Self { Self { config, callback, ffmpeg, browser } }

    pub fn call_invalid_reddit_credentials(&self) {
        (self.callback.invalid_reddit_credentials)(&self.config.lang);
    }

    pub fn call_login_successful(&self) {
        (self.callback.login_successful)(&self.config.lang);
    }
    
    pub fn call_on_new_subreddit(&self, subreddit_name: &str) {
        (self.callback.on_new_subreddit)(&self.config.lang, subreddit_name);
    }
    
    pub fn call_on_end_subreddit(&self) {
        (self.callback.on_end_subreddit)(&self.config.lang);
    }

    pub fn call_on_skipping_due_to_error<E : Error>(&self,err : E) {
        (self.callback.on_skipping_due_to_error)(&self.config.lang,&err);
    }
}