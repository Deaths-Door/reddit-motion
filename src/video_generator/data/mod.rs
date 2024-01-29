mod utils;
mod title;
mod post;
mod comments;

pub(in crate::video_generator) use utils::*;
pub(in crate::video_generator::data) use super::VideoGenerationFiles;

use chromiumoxide::Page;
use unic_langid::LanguageIdentifier;

use roux::{submission::SubmissionData, Subreddit};
use crate::config::{StoryMode, Translator, VideoCreationArguments, VideoCreationError};

impl VideoGenerationFiles {
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
            StoryMode::ReadComments { max_comments } => self.exceute_comments_no_translation(
                *max_comments, 
                subreddit,
                submission, 
                page, 
                args
            ).await,
            StoryMode::ReadPost => self.exceute_post_no_translation(submission,page,args).await,
            _ => unreachable!()
        }
    }

    
    pub async fn exceute_data_gathering_with_translation(
        &mut self,
        subreddit : &Subreddit,
        submission : &SubmissionData,
        story_mode : &StoryMode, // can not be AUTO
        target_lang : LanguageIdentifier,
        translater_client : &Translator,
        page : &Page,
        args : &VideoCreationArguments<'_>
    ) -> Result<(),VideoCreationError> {
        let target_lang = utils::unic_langid_to_deepl_lang(target_lang);

        self.exceute_title_with_translation(submission,target_lang.clone(), translater_client,page, args).await?;
   
        match story_mode {
            StoryMode::ReadComments { max_comments } => self.exceute_comments_with_translation(
                *max_comments, 
                subreddit, 
                submission,
                target_lang,
                translater_client,
                page, 
                args
            ).await,
            StoryMode::ReadPost => self.exceute_post_with_translation(submission, target_lang, translater_client, page, args).await,
            _ => unreachable!()
        }
    }
}