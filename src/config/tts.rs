use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize,Default)]
pub struct TextToSpeech {
    #[serde(default)]
    choice : TextToSpeechService,

    elevenlabs : Option<ElevenlabsConfig>
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