use roux::{Subreddit, util::FeedOption, submission::SubmissionData};
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
            // langs that we need to proccess it in
            let (submission,langs) = retry_till_new_submission(
                &mut count,
                &self.extra_langs,
                &args, 
                &subreddit
            ).await?;

            // TODO PROCCESS IT FOR THE CURRENT LANG As WELL
            for lang in langs {
                let storage_directory = format!("bin/{name}/{id}/{lang}",name=subreddit.name,id=submission.id);
                let vid_gen_args = VideoGenerationArguments::new(storage_directory);


                // TODO : PUSH THIS TO SOME SORT OF TASK MANAGER
            }
        }

        Ok(())
    }
}

async fn retry_till_new_submission<'a>(
    count : &mut u32,
    extra_langs: &'a [LanguageIdentifier],
    args : &VideoCreationArguments<'_>,
    subreddit: &Subreddit,
) -> Result<(SubmissionData,Vec<&'a LanguageIdentifier>),VideoCreationError> {
    if extra_langs.is_empty() {
        let submission = submission(&subreddit,*count).await?;
        return Ok((submission,vec![]));
    }

    loop {
        let submission = submission(&subreddit,*count).await?;
        let langs = args.db.unprocessed_threads(&submission.id,extra_langs);

        if !langs.is_empty() {
            return Ok((submission,langs));
        }
        *count += 1;
    }
}

async fn submission(subreddit : &Subreddit,count : u32)  -> Result<SubmissionData,VideoCreationError> {
    let options = FeedOption::new()
        .limit(1)
        .count(count);
    
    let v = subreddit.top(1, Some(options))
        .await?
        .data
        .children
        .pop()
        .unwrap()
        .data;

    Ok(v)
}