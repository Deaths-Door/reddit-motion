use std::collections::HashSet;
use chrono::{DateTime, Utc};
use serde_with::{serde_as,DisplayFromStr};
#[serde_as]
#[derive(Default,serde::Deserialize,serde::Serialize)]
pub struct Database {
    #[serde(skip)]
    path : String,

    // TODO : Include more metadata eg language doneit 
    subreddit_ids : HashSet<String>,

    #[serde_as(as = "DisplayFromStr")]
    pub(crate) last_version_check : DateTime<Utc>
}
use std::path::Path;
use std::fs::{File,OpenOptions};
use std::io::Write;

use roux::submission::SubmissionData;

impl Database {

    pub fn from_file_or_create(_path : &str) -> Result<Self,crate::config::LoadingConfigError> {
        let path : &Path = _path.as_ref();
        if !path.exists() {
            let mut db = Self::default();
            db.path = _path.to_string();

            let mut file = File::create(path)?;
            write!(file,"{}",toml::to_string(&db).unwrap())?;
            return Ok(db)
        }

        let toml = std::fs::read_to_string(path)?;
        let mut db = toml::from_str::<Database>(&toml)?;
        db.path = _path.to_string();
        Ok(db)
    }

    pub fn update_database(self) -> Result<(),crate::config::LoadingConfigError> {
        let toml = toml::to_string(&self).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .open(self.path)
            .unwrap();

        Ok(write!(file,"{}",toml)?)
    }

    pub fn retain(&mut self,submission : &SubmissionData) -> bool {
        // TODO : Check against more data once it has that
        self.subreddit_ids.remove(&submission.id)
    }

    pub fn add(&mut self,submission : SubmissionData) {
        // TODO : Insert more data once done
        self.subreddit_ids.insert(submission.id);
    }
}