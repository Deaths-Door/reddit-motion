use serde::{Deserialize,Serialize};


// TODO : Support for translators
#[derive(Serialize, Deserialize)]
pub struct TranslationServices {
    deepl_api_key : Option<String>,
}