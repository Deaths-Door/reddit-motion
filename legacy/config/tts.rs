use thiserror::Error;
use regex::Regex;

#[derive(Error, Debug)]
#[error("Error converting text to speech")]
pub struct TextToSpeechError;

pub(in crate::config) fn save_to_file(
    __dir : &str,
    file_name : &str,
    input_text : &str
) -> Result<String,TextToSpeechError> {
    let dir = format!("{__dir}/{file_name}");

    super::if_path_exists!(&dir, return dir);

    let text = preprocess_text(input_text);

    match gtts::save_to_file(&text,&dir) {
        true => Ok(dir),
        false => Err(TextToSpeechError)
    }
}

lazy_static::lazy_static! {
    static ref URL_REGEX : Regex = Regex::new(r"((http|https)\:\/\/)?[a-zA-Z0-9\.\/\?\:@\-_=#]+\.([a-zA-Z]){2,6}([a-zA-Z0-9\.\&\/\?\:@\-_=#])").unwrap();
}

fn preprocess_text(text : &str) -> String {
    URL_REGEX.replace_all(text, " ")
        .replace("\n", ". ")
        .replace(r"\bAGI\b", "A.G.I")
        .replace(". . .", ".")
        .replace(".. . ", ".")
        .replace(". . ", ".")
}