use serde::{Deserialize,Serialize};

#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub enum TextToSpeechService {
    #[default]
    Google,
    Elevenlabs {
        api_key : String,
        model : String,
        voice_name : String,
    }
}

impl TextToSpeechService {
    /// Sets ENV_VAR for elevenlabs_rs 
    pub fn setup(&self) {
        match self {
            Self::Elevenlabs { api_key, ..} => std::env::set_var("ELEVEN_API_KEY", api_key),
            _ => ()
        }
    }

    // TODO : IMPLMENT THIS 
    pub async fn save_speech_to_file(&self,text : &str) {
        match self {
            TextToSpeechService::Google => todo!(),
            TextToSpeechService::Elevenlabs { api_key, model, voice_name } => todo!(),
        }
    }
}