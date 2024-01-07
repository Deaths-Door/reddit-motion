use roux::submission::SubmissionData;
use serde::{Deserialize,Serialize};

#[derive(Clone)]
#[derive(Default,Debug,strum::Display)]
#[derive(Serialize,Deserialize)]
pub enum StoryMode {
    #[default]
    #[serde(rename="auto")]
    #[strum(serialize="auto")]
    Auto,
    #[serde(rename="comments")]
    #[strum(serialize="comments")]
    ReadComments{ max_comments : u32 },
    #[serde(rename="post")]
    #[strum(serialize="comments")]
    ReadPost
}

impl StoryMode {
    pub const fn default_read_comments() -> Self {
        Self::ReadComments { max_comments: 20 }
    }
}


#[derive(thiserror::Error,Debug)]
pub struct StoryModeError(StoryMode);

impl std::fmt::Display for StoryModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Unable to procceed in {}",match self.0 {
            StoryMode::Auto => "there are 0 comments on this post",
            StoryMode::ReadComments {..}  => "the post body is empty",
            StoryMode::ReadPost => "neither modes were possible",
        })
    }
}

impl StoryMode {
    pub fn resolve_mode(&self,submission : &SubmissionData) -> Result<Self,StoryModeError> {
        match self {
            Self::Auto => match Self::ReadPost.resolve_mode(submission) {
                Err(_) => Self::default_read_comments().resolve_mode(submission),
                Ok(value) => Ok(value)
            },
            Self::ReadComments {..} if submission.num_comments == 0 => Err(StoryModeError(Self::default_read_comments())),
            Self::ReadPost if submission.selftext.is_empty() =>  Err(StoryModeError(Self::ReadPost)),
            _ => Ok(self.clone())
        }
    }
}