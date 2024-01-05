use chromiumoxide::{Browser, Page};
use roux::{Subreddit, util::{RouxError, FeedOption}, submission::SubmissionData};

use crate::{config::StoryMode, db::Database};
use super::{reddit::RedditConfig, args::ParameterArgs};

// TODO : Add language support , so translate to multiple langs
#[derive(serde::Deserialize)]
pub struct SubredditConfig {
    pub(crate) name : String,

    #[serde(default = "drepeat")]
    repeat : u8,

    #[serde(default,rename = "mode")]
    story_mode : StoryMode
}

fn drepeat() -> u8 { 1 }

impl SubredditConfig {
    pub async fn handle(
        &self,
        config : &RedditConfig,
        parms : &mut ParameterArgs<'_>
    ) -> Result<(),VideoCreationError> {
        let subreddit = Subreddit::new(&self.name);
        let directory = format!("bin/{}",self.name);

        std::fs::create_dir_all(&directory)?;

        let mut count= 0;

        for _ in 0..self.repeat {
            let submission = retry_till_new_submission(
                &subreddit, parms.db, &mut count
            ).await?;

            let page = create_new_page(parms.browser,config, &submission).await?;

            let bin_directory = format!("{directory}/{id}",id = submission.id);

            let story_mode_parms = crate::config::story_mode::StoryModeParmeters {
                parms : parms, bin_directory : &bin_directory, 
                submission : &submission, page : &page
            };

            let result = self.story_mode.handle(&story_mode_parms).await;

            match result {
                // Finished making the video , add it to the list
                Ok(_) => parms.db.add(submission),
                Err(error) => {
                    if let VideoCreationError::StoryMode(ref story_error) = error {
                        (parms.callback.skipping_post)(story_error);
                        continue;
                    }

                    return Err(error)
                }
            }

            page.close().await?;
        
            count += 1;
        }

        Ok(())
    }

