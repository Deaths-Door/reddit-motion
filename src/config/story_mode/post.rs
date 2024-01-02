use crate::config::VideoCreationError;
use super::{StoryMode,StoryModeParmeters,StoryModeError};

impl StoryMode {
    pub(super) async fn read_post(parms : &StoryModeParmeters<'_>) -> Result<(),VideoCreationError> {
        // TODO : FINISH THIS
        Self::read_mode(
            parms,
            |submission|(submission.selftext.is_empty(),StoryModeError::post())
        ).await
    }
}