use serde::{Deserialize,Serialize};


// TODO : Support for translators : USE https://docs.rs/deep-translator/0.8.0/deeptrans
#[derive(Serialize, Deserialize,Default)]
pub struct TranslationServices {
    deepl_api_key : Option<String>,
}