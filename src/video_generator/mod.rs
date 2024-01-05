mod title;

use std::{path::PathBuf, collections::HashMap};

use unic_langid::LanguageIdentifier;

use crate::config::{VideoCreationError, TextToSpeechService};

// root storage dir => png dirs + audio dirs && Organized by lang
pub struct VideoGenerationArguments {
    // Gen
    storage_directory : PathBuf,

    // langs => audio + png dirs
    files : HashMap<LanguageIdentifier,Vec<(String,String)>>
}

impl VideoGenerationArguments {
    pub fn new(storage_directory: impl Into<PathBuf>) -> Self {
        Self { 
            storage_directory : storage_directory.into() , 
            files : Default::default() 
        }
    }

    pub async fn exceute_no_translate(&mut self,lang : LanguageIdentifier) -> Result<(),VideoCreationError> {
        // text + take screenshot -> dir => translate
        Ok(())
    }
}
