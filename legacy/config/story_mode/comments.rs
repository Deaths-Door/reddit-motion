use crate::config::VideoCreationError;
use super::{StoryMode,StoryModeParmeters,StoryModeError};

impl StoryMode {
    pub(super) async fn read_comments(parms : &StoryModeParmeters<'_>) -> Result<(),VideoCreationError> {
        // TODO : FINSIH THIS
        Self::read_mode(
            parms,
            |submission|(submission.num_comments == 0,StoryModeError::comments())
        ).await
    }
}