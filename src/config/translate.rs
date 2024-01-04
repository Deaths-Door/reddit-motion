use serde::{Deserialize,Serialize};


// TODO : Support for translators
#[derive(Serialize, Deserialize,Default)]
pub struct TranslationServices {
    deepl_api_key : Option<String>,
}