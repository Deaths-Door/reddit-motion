mod general;
mod post;
mod comments;

pub use general::*;
use std::future::Future;

use chromiumoxide::Page;
use roux::submission::SubmissionData;
use crate::config::{VideoCreationError,ParameterArgs};

#[derive(serde::Deserialize,Default,strum::Display)]
pub enum StoryMode {
    #[default]
    #[serde(rename="auto")]
    #[strum(serialize="auto")]
    Auto,
    #[serde(rename="comments")]
    #[strum(serialize="comments")]
    ReadComments,
    #[serde(rename="post")]
    #[strum(serialize="comments")]
    ReadPost
}

// if some process ahead fails it keeps on to the file
macro_rules! if_path_exists {
    ($path : expr,return $return : expr) => {
        if std::path::Path::new($path).exists() {
            return Ok($return)
        }
    };
    (not $path : expr,$code : expr) => {
        if !std::path::Path::new($path).exists() {
           $code
        }
    };
}

pub(crate) use if_path_exists;

#[derive(thiserror::Error)]
#[error("Failed to reach {} , as {} ",.mode,.cause)]
pub struct StoryModeError {
    mode : StoryMode,
    cause : String
}

impl StoryModeError {
    fn new(mode : StoryMode,cause : impl Into<String>) -> Self {
        Self { mode ,cause :  cause.into() }
    }
    fn comments() -> Self {
        Self::new(StoryMode::ReadComments, "there are 0 comments on this post")
    }

    fn post() -> Self {
        Self::new(StoryMode::ReadPost, "the post body is empty")
    }

    fn auto() -> Self {
        Self::new(StoryMode::Auto, "neither modes were possible")
    }
}

impl std::fmt::Debug for StoryModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{self}",)
    }
}

impl StoryMode {
    #[async_recursion::async_recursion]
    pub async fn handle(
        &self,
        parms : &general::StoryModeParmeters<'_>,
    ) -> Result<(),VideoCreationError> {
        match self {
            Self::ReadComments => Self::read_comments(parms).await,
            Self::ReadPost => Self::read_post(parms).await,
            Self::Auto => Self::ReadComments.auto_handler(
                parms,
                || Self::ReadPost.auto_handler(parms,|| async { Err(VideoCreationError::from(StoryModeError::auto())) }
                )
            ).await,
        }
    }

    async fn auto_handler<O>(
        &self,
        parms : &general::StoryModeParmeters<'_>,
        on_story_mode_error : impl FnOnce() -> O
    ) -> Result<(),VideoCreationError> where O : Future<Output=Result<(),VideoCreationError>> {
        match self.handle(parms).await {
            #[allow(unused_variables)]
            Err(err) if matches!(VideoCreationError::StoryMode,err) => (on_story_mode_error)().await,
            result @ _ => result
        }
    }
}