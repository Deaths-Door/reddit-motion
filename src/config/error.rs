use roux::util::RouxError;

#[derive(thiserror::Error, Debug)]
pub enum VideoCreationError {
    #[error(transparent)]
    Reddit(#[from] RouxError),

   // #[error(transparent)]
  //  File(#[from] std::io::Error),
    
    //#[error(transparent)]
  //  TextToSpeech(#[from] super::TextToSpeechError),

    #[error(transparent)]
    StoryMode(#[from] crate::config::story_mode::StoryModeError),   
    #[error("Failed to navigate browser, {}",.0)]
    Browser(#[from] chromiumoxide::error::CdpError),
}