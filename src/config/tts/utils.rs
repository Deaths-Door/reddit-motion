use std::path::Path;

use regex::Regex;
use elevenlabs_rs::Speech;
use super::TextToSpeechError;

lazy_static::lazy_static! {
    static ref URL_REGEX : Regex = Regex::new(r"((http|https)\:\/\/)?[a-zA-Z0-9\.\/\?\:@\-_=#]+\.([a-zA-Z]){2,6}([a-zA-Z0-9\.\&\/\?\:@\-_=#])").unwrap();
}

pub(super) fn preprocess_text(text : &str) -> String {
    URL_REGEX.replace_all(text, " ")
        .replace("\n", ". ")
        .replace(r"\bAGI\b", "A.G.I")
        .replace(". . .", ".")
        .replace(".. . ", ".")
        .replace(". . ", ".")
}

pub(super) fn google(directory : &Path,text : &str) -> Result<(),TextToSpeechError> {
    match gtts::save_to_file(&text,&directory.display().to_string()) {
        true => Ok(()),
        false => Err(TextToSpeechError(anyhow::anyhow!("Unknown Cause").into()))
    }
}

pub(super) async fn elevenlabs(directory : &Path,text : &str, model : &str, voice_name : &str ) -> Result<(),TextToSpeechError> {
    let speech = Speech::new(text,voice_name,model,0).await?;
    Ok(speech.save(Some(directory.display().to_string()))?)
}