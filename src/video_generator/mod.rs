use std::{path::PathBuf, collections::HashMap};

use unic_langid::LanguageIdentifier;

// root storage dir => png dirs + audio dirs && Organized by lang
pub struct VideoGenerationArguments {
    storage_directory : PathBuf,
    // langs => audio + png dirs
    files : HashMap<LanguageIdentifier,Vec<(String,String)>>
}

impl VideoGenerationArguments {
    pub fn new(storage_directory: impl Into<PathBuf>) -> Self {
        Self { storage_directory : storage_directory.into() , files : Default::default() }
    }
}
