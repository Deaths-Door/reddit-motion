mod data;
mod gen;

use roux::submission::SubmissionData;
use unic_langid::LanguageIdentifier;
use std::path::PathBuf;
#[derive(Debug)]
pub struct VideoGenerationFiles {
    // Gen
    storage_directory : PathBuf,

    // audio + png dirs
    files : Vec<(String,String)>
}

impl VideoGenerationFiles {
    pub fn new_and_create_dir(submission : &SubmissionData,lang : &LanguageIdentifier) -> Self {
        let storage_directory = format!("bin/{name}/{id}/{lang}",name=submission.subreddit,id=submission.id);
        Self::new(storage_directory)
    }

    fn new(storage_directory: impl Into<PathBuf>) -> Self {
        Self { 
            storage_directory : storage_directory.into() , 
            files : Default::default() 
        }
    }
}