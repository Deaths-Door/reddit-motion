use serde::{Deserialize,Serialize};
use std::process::{Command,Child};

#[derive(Clone,Copy,Default,Deserialize,Serialize)]
pub enum VideoDuration {
    #[default]
    Infinite,
    Limited { limit: f64 },
    Both { limit: f64 }
}

#[derive(Default,Deserialize,Serialize)]
pub struct ExternalScripts { 
    #[serde(skip_serializing_if = "Option::is_none")]
    limited : Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    infinite : Option<String>
}

impl ExternalScripts {
    pub fn infinite_script(&self) -> Option<&String> {
        self.infinite.as_ref()
    }

    pub fn limited_script(&self) -> Option<&String> {
        self.limited.as_ref()
    }

    pub fn call_infinite_script(&self,file_directory : &str) -> Option<std::io::Result<Child>> {
        self.infinite.as_ref().map(|s| Command::new(s).args(["--infinite",file_directory]).spawn() )
    }

    pub fn call_limited_script(&self,file_directories : &[String]) -> Option<std::io::Result<Child>> {
        self.limited.as_ref().map(|s| Command::new(s).arg("--limited").args(file_directories).spawn() )
    }
}