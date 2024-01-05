use std::collections::{HashMap, HashSet};
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

    #[serde_as(as = "HashMap<_,HashSet<DisplayFromStr>>")]
    #[serde(default)]
    proccessed_threads : HashMap<String,HashSet<LanguageIdentifier>>
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


    pub fn unprocessed_threads<'a>(&self,id : &str,langs : &'a [LanguageIdentifier]) -> Vec<&'a LanguageIdentifier> {
        self.proccessed_threads.get(id).and_then(|processed_langs|{
            Some(
                langs.iter()
                    .filter(|lang| !processed_langs.contains(lang))
                    .collect()
            )
        }).unwrap_or(vec![])
    }
}