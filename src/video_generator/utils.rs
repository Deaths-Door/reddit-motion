use std::path::{PathBuf, Path};

use chromiumoxide::{Page, Browser};
use roux::submission::SubmissionData;

use crate::config::{VideoCreationError, StoryMode, VideoCreationArguments};

use super::VideoGenerationArguments;

impl VideoGenerationArguments {
    pub(super) fn push_files(&mut self,audio : PathBuf,png : PathBuf) {
        self.files.push((audio.display().to_string(),png.display().to_string()))
    }

    pub(super) fn audio_file(&self,name : &str) -> PathBuf {
        let mut pathbuf = self.storage_directory.clone();
        pathbuf.set_file_name(format!("{}.mp3",name));
        pathbuf
    }
    
    pub(super) fn png_file(&self,name : &str) -> PathBuf {
        let mut pathbuf = self.storage_directory.clone();
        pathbuf.set_file_name(format!("{}.png",name));
        pathbuf
    }
}

macro_rules! exceute {
    (
        $story_mode:expr,
        $title : expr,
        $comments : expr,
        $post :expr,
        $auto : expr
    ) => {{
        $title;

        match $story_mode {
            StoryMode::ReadComments => $comments,
            StoryMode::ReadPost =>$post,
            _ => $auto
        }
    }};
}

// if some process ahead fails it keeps on to the file
macro_rules! if_path_exists {
    (not $path : expr,$code : expr) => {
        if !std::path::Path::new($path).exists() {
           $code
        }
    };
}

pub(super) use {exceute,if_path_exists};

