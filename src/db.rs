use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use roux::submission::SubmissionData;
use serde_with::{serde_as,DisplayFromStr};
use unic_langid::LanguageIdentifier;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

#[serde_as]
#[derive(Default,serde::Deserialize,serde::Serialize)]
pub struct Database {
    #[serde_as(as = "DisplayFromStr")]
    pub last_version_check : DateTime<Utc>,

    #[serde_as(as = "HashMap<_,HashSet<DisplayFromStr>>")]
    #[serde(default)]
    proccessed_threads : HashMap<String,HashSet<LanguageIdentifier>>
}

const NAME : &str = "db.toml";
impl Database {
    pub fn try_create() -> anyhow::Result<Database> {
        let path : &Path = NAME.as_ref();
        Ok(match path.exists() {
            true => {
                let toml = std::fs::read_to_string(path)?;
                toml::from_str::<Database>(&toml)?
            },
            false => Self::default()
        })
    }

    pub fn update_database(self) -> anyhow::Result<()> {
        let toml = toml::to_string(&self).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(NAME)
            .unwrap();

        Ok(write!(file,"{}",toml)?)
    }


    pub fn unprocessed_threads<'a>(&self,id : &str,langs : &'a [LanguageIdentifier]) -> Vec<&'a LanguageIdentifier> {
        self.proccessed_threads.get(id).map(|processed_langs|  langs.iter()
            .filter(|lang| !processed_langs.contains(lang))
            .collect()
        ).unwrap_or(vec![])
    }

    pub fn add_proccessed_thread(&mut self,submission : &SubmissionData,lang : LanguageIdentifier) {
        match self.proccessed_threads.get_mut(&submission.id) {
            None => { self.proccessed_threads.insert(submission.id.clone(), HashSet::from([lang])); },
            Some(values) => { values.insert(lang); },
        };
    }
}