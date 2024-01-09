use std::path::Path;
use rand::Rng;
use rusty_ytdl::{get_video_id, Video};
use serde::Deserialize;

use crate::ffmpeg::FFmpeg;

#[derive(Deserialize)]
pub struct Assets {
    videos : Vec<String>,
    audio : Vec<String>
}

impl Assets {
    pub fn count(&self) -> usize {
        self.audio.len() + self.videos.len()
    }

}
impl Assets {
    pub async fn download<F>(&mut self,ffmpeg : &FFmpeg,on_each_download_finished : F) -> anyhow::Result<()> where F : Fn() + Copy {
        async fn inner<F : Fn(), I : Fn(String) -> anyhow::Result<String>>(
            vec : &mut [String],
            on_each_download_finished : F,
            map : I,
        ) -> anyhow::Result<()>  {
            for item in vec {
                let id = get_video_id(item);
    
                if id.is_none() {
                    let path : &Path = item.as_ref();
                    assert!(path.exists(),"Given path {item} , does not exists on local machine");
                    continue;
                }
               
                let id = id.unwrap();
    
                let file_name = format!(r"assets/{id}.mp4");
                let path : &Path = file_name.as_ref();
    
                if path.exists() {
                    // Exists don't brother downloading
                    continue;
                }
    
                let video = Video::new(id)?;
                video.download(&file_name).await?;
    
                // Update assets to valid local paths
                *item = map(file_name)?;
    
                (on_each_download_finished)()
            }

            Ok(())
        }

        inner(&mut self.videos,on_each_download_finished,|mp4_directory| Ok(mp4_directory)).await?;
        inner(&mut self.audio, on_each_download_finished,|mp4_directory| {
            let mp3_directory = mp4_directory.replace(".mp4", ".mp3");
            extract_audio_from_mp4_and_delete(ffmpeg,&mp4_directory,&mp3_directory)?;
            Ok(mp3_directory)
         }).await?;

        Ok(())
    }   
}

fn extract_audio_from_mp4_and_delete(ffmpeg : &FFmpeg,mp4_directory : &str,mp3_directory : &str) -> anyhow::Result<()> {
    //  ffmpeg -i "inputmp4" -q:a 0 -map a "outputmp3"";
    ffmpeg.ffmpeg_expect_failure(|cmd| { 
        cmd.args(&["-i", mp4_directory, "-q:a", "0", "-map", "a", mp3_directory]); 
    })?;

    std::fs::remove_file(mp4_directory)?;
    Ok(())
}