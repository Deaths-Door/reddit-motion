use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

#[serde_as]
#[derive(Default,serde::Deserialize,serde::Serialize)]
pub struct Database {
    #[serde(skip)]
    path : String,

    #[serde_as(as = "DisplayFromStr")]
    pub last_version_check : DateTime<Utc>,

    // TODO : Include more metadata eg language doneit 
    #[serde_as(as = "HashMap<_,Vec<DisplayFromStr>>")]
    subreddit_ids : HashMap<String,Vec<LanguageIdentifier>>
}

impl Database {
    pub fn from_file_or_create(_path : &str) -> anyhow::Result<Database> {
        let path : &Path = _path.as_ref();
        let mut db = match path.exists() {
            true => {
                let toml = std::fs::read_to_string(path)?;
                toml::from_str::<Database>(&toml)?
            },
            false => Self::default()
        };
        db.path = _path.to_string();
        Ok(db)
    }

    pub fn update_database(self) -> anyhow::Result<()> {
        let toml = toml::to_string(&self).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.path)
            .unwrap();

        Ok(write!(file,"{}",toml)?)
    }

    /*pub fn retain(&mut self,submission : &SubmissionData) -> bool {
        // TODO : Check against more data once it has that
        self.subreddit_ids.remove(&submission.id)
    }

    pub fn add(&mut self,submission : SubmissionData) {
        // TODO : Insert more data once done
        self.subreddit_ids.insert(submission.id);
    }*/
}