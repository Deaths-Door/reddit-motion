use roux::submission::SubmissionData;
use serde::{Deserialize,Serialize};

#[derive(Default,Debug,strum::Display)]
#[derive(Serialize,Deserialize)]
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
#[derive(thiserror::Error,Debug)]
pub struct StoryModeError(StoryMode);

impl std::fmt::Display for StoryModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Unable to procceed in {}",match self.0 {
            StoryMode::Auto => "there are 0 comments on this post",
            StoryMode::ReadComments => "the post body is empty",
            StoryMode::ReadPost => "neither modes were possible",
        })
    }
}

impl StoryMode {
    pub fn can_proceed(&self,submission : &SubmissionData) -> Result<(),StoryModeError> {
        match self {
            StoryMode::Auto => match StoryMode::ReadPost.can_proceed(submission) {
                Err(_) => StoryMode::ReadComments.can_proceed(submission),
                _ => Ok(())
            },
            StoryMode::ReadComments if submission.num_comments == 0 => Err(StoryModeError(StoryMode::ReadComments)),
            StoryMode::ReadPost if submission.selftext.is_empty() =>  Err(StoryModeError(StoryMode::ReadPost)),
            _ => Ok(())
        }
    }
}