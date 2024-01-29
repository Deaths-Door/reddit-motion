mod utils;

use std::path::Path;

use serde::{Deserialize,Serialize};

#[derive(Default)]

#[derive(Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum TextToSpeechService {
    #[default]
    Google,
    Elevenlabs {
        api_key : String,

        #[serde(default="default_model")]
        model : String,

        #[serde(default="default_voice")]
        voice_name : String,
    }
}

fn default_voice() -> String { "Adam".into() }

fn default_model() -> String { "eleven_multilingual_v2".into() }

#[derive(thiserror::Error,Debug)]
#[error("Failed to convert text to speech as, {}",.0.to_string())]
pub struct TextToSpeechError(#[from] pub(in crate::config::tts) Box<dyn std::error::Error + Send + Sync>);

impl TextToSpeechService {
    /// Sets ENV_VAR for elevenlabs_rs 
    pub fn setup(&self) {
        match self {
            Self::Elevenlabs { api_key, ..} => std::env::set_var("ELEVEN_API_KEY", api_key),
            _ => ()
        }
    }

    pub async fn save_speech_to_file(&self,directory : &Path,unproccessed_text : &str) -> Result<(),TextToSpeechError> {
        let text = utils::preprocess_text(unproccessed_text);
        match self {
            TextToSpeechService::Google => utils::google(directory,&text),
            TextToSpeechService::Elevenlabs { model, voice_name , .. } => utils::elevenlabs(directory,&text,&model,&voice_name).await,
        }
    }
}