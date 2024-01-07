use roux::Subreddit;
use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;

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

        let page = super::create_new_page(args.browser,&submission).await?;

        let detected_lang = super::detect_post_language(&args.detector,&submission);
        let storage_directory = format!("bin/{name}/{id}/{detected_lang}",name=subreddit.name,id=submission.id);
        std::fs::create_dir_all(&storage_directory)?;
        let mut video_generation_arguments = VideoGenerationArguments::new(storage_directory);


        // TODO : ADD IT TO THE TASKMANAGER
        // TODO : FINSIH IT
        video_generation_arguments.exceute_no_translation(
            &subreddit,
            &submission,
            &story_mode,
            &page,
            &args
        ).await?;

        println!("{:?}",video_generation_arguments);

        /*for lang in extra_langs {
            video_generation_arguments.exceute_with_translation(
                lang,
                &submission,
                &story_mode,
                &page,
                &args
            ).await?;
        }*/

        Ok(())
    }
}