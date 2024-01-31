use std::error::Error;
use roux::submission::SubmissionData;
use unic_langid::LanguageIdentifier;

pub struct Callback {    
    // UI
    pub(in crate::config) invalid_reddit_credentials : fn(&LanguageIdentifier),
    pub(in crate::config) login_successful : fn(&LanguageIdentifier),

    pub(in crate::config) on_new_subreddit : fn(&LanguageIdentifier,&str),
    pub(in crate::config) on_end_subreddit : fn(&LanguageIdentifier),
    
    pub(in crate::config) on_skipping_post_due_to_error : fn(&LanguageIdentifier,&dyn Error),

    pub(in crate::config) on_post_choosen : fn(&LanguageIdentifier,&SubmissionData),
    
    pub(in crate::config) on_video_finished : fn(&LanguageIdentifier,std::io::Result<String>),
    
    pub(in crate::config) failed_to_spawn_task : fn(&LanguageIdentifier,&str,&dyn Error),
    pub(in crate::config) task_with_code : fn(&LanguageIdentifier,&str,&i32),
}

impl Callback {
    pub fn new(
        invalid_reddit_credentials: fn(&LanguageIdentifier), 
        login_successful: fn(&LanguageIdentifier), 
        on_new_subreddit: fn(&LanguageIdentifier,&str), 
        on_end_subreddit: fn(&LanguageIdentifier),
        on_skipping_post_due_to_error: fn(&LanguageIdentifier,&dyn Error),
        on_post_choosen: fn(&LanguageIdentifier,&SubmissionData),
        on_video_finished: fn(&LanguageIdentifier,std::io::Result<String>),
        failed_to_spawn_task : fn(&LanguageIdentifier,&str,&dyn Error),
        task_with_code: fn(&LanguageIdentifier,&str,&i32),
    ) -> Self { 
        Self { 
            invalid_reddit_credentials ,
            login_successful,
            on_new_subreddit , 
            on_end_subreddit ,
            on_skipping_post_due_to_error ,
            on_post_choosen ,
            on_video_finished,
            failed_to_spawn_task,
            task_with_code
        }
    }
}