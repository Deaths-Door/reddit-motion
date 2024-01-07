mod utils;
mod title;
mod post;
mod comments;

use chromiumoxide::Page;
use unic_langid::LanguageIdentifier;
pub(in crate::video_generator) use utils::*;
use std::path::PathBuf;

use roux::{submission::SubmissionData, Subreddit};

use crate::config::{VideoCreationError, VideoCreationArguments, StoryMode};

#[derive(Debug)]
pub struct VideoGenerationArguments {
    // Gen
    storage_directory : PathBuf,

    // audio + png dirs
    files : Vec<(String,String)>
}

impl VideoGenerationArguments {
    pub fn new(storage_directory: impl Into<PathBuf>) -> Self {
        Self { 
            storage_directory : storage_directory.into() , 
            files : Default::default() 
        }
    }

    pub async fn exceute_no_translation(
        &mut self,
        subreddit : &Subreddit,
        submission : &SubmissionData,
        story_mode : &StoryMode, // can not be AUTO
        page : &Page,
        args : &VideoCreationArguments<'_>
    ) -> Result<(),VideoCreationError> {
        self.exceute_title_no_translation(submission,page,args).await?;
        match story_mode {
            StoryMode::ReadComments { max_comments } => self.exceute_comments_no_translation(*max_comments, subreddit, submission, page, args).await,
            StoryMode::ReadPost => self.exceute_post_no_translation(submission,page,args).await,
            _ => Ok(())
        }
    }

    
    pub async fn exceute_with_translation(
        &mut self,
        langs : &LanguageIdentifier,
        submission : &SubmissionData,
        story_mode : &StoryMode, // can not be AUTO
        page : &Page,
        args : &VideoCreationArguments<'_>
    ) -> Result<(),VideoCreationError> {
        // TODO()
        todo!()
    }
}