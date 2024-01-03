use serde::{Deserialize,Serialize};

#[derive(Default,strum::Display)]
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
