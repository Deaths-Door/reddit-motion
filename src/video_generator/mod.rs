mod utils;
mod title;

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

    pub async fn exceute_no_translate(
        &mut self,
        submission : &SubmissionData,
        story_mode : &StoryMode, // can not be AUTO
        args : &VideoCreationArguments<'_>
    ) -> Result<(),VideoCreationError> {
        let page = create_new_page(args.browser,submission).await?;

        self.exceute_title(submission,&page,args).await?;

        /*story_mode.exceute(submission,args).await?;*/
        Ok(())
    }
}