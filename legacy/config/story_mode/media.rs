use std::collections::HashMap;

use unic_langid::LanguageIdentifier;

#[derive(Default)]
pub(super) struct MediaFiles {
    files : HashMap<LanguageIdentifier,Vec<(String,String)>>
}