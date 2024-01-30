use std::error::Error;

use chromiumoxide::Browser;
use roux::submission::SubmissionData;
use whatlang::Detector;
use crate::ffmpeg::FFmpeg;
use super::{Config, Callback};

// TODO : MAKE IT ALL A REFERENCE??
pub struct VideoCreationArguments<'a> {
    pub config : Config,
    pub ffmpeg : FFmpeg,
    pub browser : &'a Browser,
    pub detector : Detector,
    callback : &'a Callback,
}

impl<'a> VideoCreationArguments<'a> {
    pub fn new(
        callback: &'a Callback, 
        browser: &'a Browser,
        config: Config, 
        ffmpeg: FFmpeg,
    ) -> Self { Self { config , ffmpeg, callback, browser , detector : Detector::new()} }

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

    pub fn call_on_skipping_post_due_to_error<E : Error>(&self,err : E) {
        (self.callback.on_skipping_post_due_to_error)(&self.config.lang,&err)
    }

    pub fn call_on_post_choosen(&self,submission : &SubmissionData) {
        (self.callback.on_post_choosen)(&self.config.lang,submission)
    }

    pub fn call_on_video_finished(&self,result : std::io::Result<String>) {
        (self.callback.on_video_finished)(&self.config.lang,result)
    }
}