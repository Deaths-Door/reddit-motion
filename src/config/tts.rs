use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
pub struct TextToSpeech {
    choice : TextToSpeechService,
    elevenlabs : ElevenlabsConfig
}

#[derive(Serialize, Deserialize)]
pub struct ElevenlabsConfig {
    model : String,
    voice_ids : Vec<String>
}

#[derive(Default,Serialize, Deserialize)]
pub enum TextToSpeechService {
    #[default]
    Google,
    Elevenlabs
}