use roux::Subreddit;
use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::{LanguageIdentifier, langid};
use whatlang::Lang;

use crate::{config::{StoryMode, TextToSpeechService, VideoCreationArguments, VideoCreationError}, video_generator::VideoGenerationArguments};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SubredditConfig {
    pub(super) name : String,

    #[serde(default = "drepeat")]
    repeat_count : u8,

    #[serde(default,rename = "mode")]
    story_mode : StoryMode,

    #[serde(default)]
    for_tts_use : TextToSpeechService,

    #[serde_as(as = "Vec<DisplayFromStr>")]
    extra_langs: Vec<LanguageIdentifier>,
}

fn drepeat() -> u8 { 1 }

impl SubredditConfig {
    pub async fn exceute(&self,args : &VideoCreationArguments<'_>) -> Result<(),VideoCreationError> {
        let subreddit  = Subreddit::new(&self.name);

        let mut count= 0;
        for _ in 0..self.repeat_count {
            if let Err(err) = self.__exceute(&mut count, args, &subreddit).await {
                args.call_on_skipping_due_to_error(err)
            }
        }

        Ok(())
    }

    pub async fn __exceute(
        &self,
        count : &mut u32,
        args : &VideoCreationArguments<'_>,
        subreddit : &Subreddit
    ) -> Result<(),VideoCreationError> {
        // langs that we need to proccess it in
        let (submission,extra_langs) = super::retry_till_new_submission(
            count,
            &self.extra_langs,
            &args, 
            &subreddit
        ).await?;

        let story_mode = self.story_mode.resolve_mode(&submission)?;

        args.call_on_post_choosen(&submission);

        let detected_lang = super::detect_post_language(&args.detector,&submission);
        // AND EVERY OTHER LANG
       // for lang in langs {
            let storage_directory = format!("bin/{name}/{id}/{detected_lang}",name=subreddit.name,id=submission.id);
            std::fs::create_dir_all(&storage_directory)?;
            
            let mut video_generation_arguments = VideoGenerationArguments::new(storage_directory);

            video_generation_arguments.exceute_no_translate(
                &submission,
                &story_mode,
                &args
            ).await?;
            // TODO : ADD IT TO THE TASKMANAGER
    //    }

        Ok(())
    }
}