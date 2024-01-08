use chromiumoxide::async_process::Command;
use roux::Subreddit;
use serde::{Deserialize,Serialize};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;
use crate::{config::{StoryMode, TextToSpeechService, VideoCreationArguments, VideoCreationError}, video_generator::VideoGenerationFiles, db::Database, ffmpeg::FFmpeg};

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
    pub async fn exceute(&self,args : &VideoCreationArguments<'_>,db : &mut Database,add_task : impl Fn(VideoGenerationFiles,FFmpeg) + Copy) {
        let subreddit  = Subreddit::new(&self.name);

        let mut count= 0;
        for _ in 0..self.repeat_count {
            if let Err(err) = self.__exceute(&mut count,db, args, &subreddit,add_task).await {
                args.call_on_skipping_post_due_to_error(err)
            }
        }
    }

    pub async fn __exceute(
        &self,
        count : &mut u32,
        db : &mut Database,
        args : &VideoCreationArguments<'_>,
        subreddit : &Subreddit,
        add_task : impl Fn(VideoGenerationFiles,FFmpeg)
    ) -> Result<(),VideoCreationError> {
        // langs that we need to proccess it in
        let (submission,extra_langs) = super::retry_till_new_submission(
            db,
            count,
            &self.extra_langs,
            &subreddit
        ).await?;

        let story_mode = self.story_mode.resolve_mode(&submission)?;

        args.call_on_post_choosen(&submission);

        let page = super::create_new_page(args.browser,&submission).await?;

        {
            let detected_lang = super::detect_post_language(&args.detector,&submission);
            let mut video_generation_files = VideoGenerationFiles::new_and_create_dir(&submission,&detected_lang);
    
            video_generation_files.exceute_data_gathering_no_translation(
                &subreddit,
                &submission,
                &story_mode,
                &page,
                &args
            ).await?;
            
            add_task(video_generation_files,args.ffmpeg.clone());
            db.add_proccessed_thread(&submission, detected_lang);
        }

        for lang in extra_langs {
            let mut video_generation_files = VideoGenerationFiles::new_and_create_dir(&submission,&lang);

            video_generation_files.exceute_data_gathering_with_translation(
                lang,
                &submission,
                &story_mode,
                &page,
                &args
            ).await?;

            add_task(video_generation_files,args.ffmpeg.clone());
            db.add_proccessed_thread(&submission, lang.clone());
        }

        Ok(())
    }
}