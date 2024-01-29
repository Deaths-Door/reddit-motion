use deepl::Lang;
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize,Default)]
pub struct TranslationServices {
    deepl_api_key : String,
}

impl TranslationServices {
    pub fn translator_client(&self) -> Translator {
        Translator(deepl::DeepLApi::with(&self.deepl_api_key).new())
    }
}

pub struct Translator(deepl::DeepLApi);

pub type TranslatorError = deepl::Error;

pub type TranslatorResult = Result<String, TranslatorError>;

impl Translator {    
    pub async fn translate(
        &self,
        text : &str,
        target_lang : Lang
    ) -> TranslatorResult {
        self.0.translate_text(text, target_lang)
            .await
            .map(|response| {
                response.translations.into_iter()
                    .fold(String::new(),|mut string,sentence|{
                        string.push_str(&sentence.text);
                        string
                    })
                    .into()
            })
    }
}
