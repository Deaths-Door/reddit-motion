mod title;

use std::{path::PathBuf, collections::HashMap};

use roux::submission::SubmissionData;
use unic_langid::LanguageIdentifier;

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
        /*args.config.tts.save_speech_to_file(&submission.title).await?;

        story_mode.exceute(submission,args).await?;*/
        // text + take screenshot -> dir => translate
        Ok(())
    }
}
