use roux::submission::SubmissionData;

use crate::config::{StoryModeError,SubredditConfig, Dimesions};

pub struct Callback {
    pub on_new_subreddit : fn (&SubredditConfig),
    pub on_end_subreddit : fn (),
    pub info : fn(&SubmissionData),
    pub skipping_post : fn(&StoryModeError),
    pub dimesions_out_of_bounds : fn(&Dimesions,&Dimesions)
}