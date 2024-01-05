use std::error::Error;

use chromiumoxide::Browser;
use roux::submission::SubmissionData;
use whatlang::Detector;
use crate::{ffmpeg::FFmpeg, db::Database};
use super::{Config, Callback};

pub struct VideoCreationArguments<'a> {
    pub config : &'a Config,
    pub ffmpeg : &'a FFmpeg,
    pub browser : &'a Browser,
    pub db : &'a Database,
    pub detector : Detector,
    callback : &'a Callback,
}

impl<'a> VideoCreationArguments<'a> {
    pub fn new(
        config: &'a Config, 
        callback: &'a Callback, 
        ffmpeg: &'a FFmpeg,
        db : &'a Database, 
        browser: &'a Browser
    ) -> Self { Self { config, callback, ffmpeg, browser ,db , detector : Detector::new()} }

    pub fn call_invalid_reddit_credentials(&self) {
        (self.callback.invalid_reddit_credentials)(&self.config.lang)
    }

    pub fn call_login_successful(&self) {
        (self.callback.login_successful)(&self.config.lang)
    }
    
    pub fn call_on_new_subreddit(&self, subreddit_name: &str) {
        (self.callback.on_new_subreddit)(&self.config.lang, subreddit_name)
    }
    
    pub fn call_on_end_subreddit(&self) {
        (self.callback.on_end_subreddit)(&self.config.lang)
    }

    pub fn call_on_skipping_due_to_error<E : Error>(&self,err : E) {
        (self.callback.on_skipping_due_to_error)(&self.config.lang,&err)
    }

    pub fn call_on_post_choosen(&self,submission : &SubmissionData) {
        (self.callback.on_post_choosen)(&self.config.lang,submission)
    }
}