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

pub(in crate::video_generator::data) use super::VideoGenerationArguments;

impl VideoGenerationArguments {
    pub async fn exceute_data_gathering_no_translation(
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

    
    pub async fn exceute_data_gathering_with_translation(
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