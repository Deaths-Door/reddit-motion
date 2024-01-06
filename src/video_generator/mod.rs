mod utils;
mod title;
mod post;

use chromiumoxide::Page;
use unic_langid::LanguageIdentifier;
pub(in crate::video_generator) use utils::*;
use std::path::PathBuf;

use roux::submission::SubmissionData;

use crate::config::{VideoCreationError, VideoCreationArguments, StoryMode};

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
        submission : &SubmissionData,
        story_mode : &StoryMode, // can not be AUTO
        page : &Page,
        args : &VideoCreationArguments<'_>
    ) -> Result<(),VideoCreationError> {
        // TODO UPDATE THIS
        exceute!(
            story_mode,
            self.exceute_title_no_translation(submission,page,args).await?,
            self.exceute_post_no_translation(submission,page,args).await,
            Ok(()),
            Ok(())
        )
    }

    
    pub async fn exceute_with_translation(
        &mut self,
        langs : &LanguageIdentifier,
        submission : &SubmissionData,
        story_mode : &StoryMode, // can not be AUTO
        page : &Page,
        args : &VideoCreationArguments<'_>
    ) -> Result<(),VideoCreationError> {
        exceute!(
            story_mode,
            { },
            Ok(()),
            Ok(()),
            Ok(())
        )
    }
}