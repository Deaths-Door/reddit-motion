use std::path::{Path, PathBuf};

use mime::Name;
use mime_guess::MimeGuess;
use rusty_ytdl::{Video, get_video_id};
use serde::{Deserialize,Serialize};
use rand::Rng;

#[derive(Serialize,Deserialize)]
pub struct Assets {
    videos : Vec<String>,
    audio : Vec<String>
}

impl Assets {
    pub fn count(&self) -> usize {
        self.audio.len() + self.videos.len()
    }

    fn get_random_index(slice : &[String]) -> Option<&String> {
        slice.get(rand::thread_rng().gen_range(0..slice.len()))
    }

    pub fn random_video_directory(&self) -> Option<&String> {
        Self::get_random_index(&self.videos)
    }

    pub fn random_audio_directory(&self) -> Option<&String> {
        Self::get_random_index(&self.audio)
    }
}

impl Assets {
    pub fn on_empty_assets<F>(&self,on_empty : F) -> anyhow::Result<()> where F : Fn() -> anyhow::Result<()>{
        if self.videos.is_empty() || self.audio.is_empty() {
            on_empty()
        } else { Ok(()) }
    }

    pub async fn process_and_download<F,D>(
        &mut self,
        warn_wrong_mime : F,
        on_each_download : D,
    ) -> anyhow::Result<()> where F : Fn(PathBuf,&Name<'_>,&Name<'_>) + Copy , D : Fn() + Copy {
        Self::__proccess(&mut self.videos,warn_wrong_mime,on_each_download,"mp4",mime::VIDEO).await?;
        Self::__proccess(&mut self.audio,warn_wrong_mime,on_each_download,"mp3",mime::AUDIO).await
    }

    async fn __proccess<F,D>(
        slice : &mut Vec<String>,
        warn_wrong_mime : F,
        on_each_download : D,
        extension : &str,
        expected_mime : Name<'_>
    ) -> anyhow::Result<()> where F : Fn(PathBuf,&Name<'_>,&Name<'_>) , D : Fn() + Copy {
        let mut vec = vec![];
        for item in &mut *slice {
            // Download  
            if item.contains("https") {
                let id = get_video_id(item).unwrap();
                let file_name = format!(r"assets/{id}.{extension}");
                let path : &Path = file_name.as_ref();
        
                if path.exists() {
                    // Exists don't brother downloading
                    continue;
                }
        
                let video = Video::new(id)?;
                video.download(&file_name).await?;
                on_each_download();

                *item = file_name;
                continue
            }

            let entries = glob::glob(item)?;
            
            for entry in entries.filter_map(Result::ok) {
                let mime = MimeGuess::from_path(&entry).first_or_octet_stream();
                let display = entry.display().to_string();
                
                if mime.type_() != expected_mime {
                    warn_wrong_mime(entry,&mime.type_(),&expected_mime);
                }

                vec.push(display)
            }
        }

        slice.extend(vec);

        Ok(())
    }
}