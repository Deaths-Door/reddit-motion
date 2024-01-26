mod crop;
mod media;
mod utils;
mod shared;

use crate::config::VideoDuration;

use super::VideoGenerator;

impl VideoGenerator {
    // returns output video path
    pub async fn exceute(self) -> std::io::Result<String> {   
        // So basically group together a bunch of audio files till video_len_limit 
        // extend background video to audio_len aka video_limit
        // and then add pngs , audio etc 
        let bin_directory = self.video_gen_files.storage_directory.display().to_string();

        // So instead of bin/.. do to generated_videos/..
        let output_directory = bin_directory.replace("bin", "generated_videos");
        std::fs::create_dir_all(&output_directory)?;

        self.infinte_video_duration(&bin_directory,&output_directory)?;

        // TODO : ALLOW CUSTOMIZABLE VOICE IN BACKGROUND MUSIC + MAIN AUDIO
        // TODO : ENABLE THIS
        /*match self.video_duration {
            VideoDuration::Infinite => todo!(),
            VideoDuration::Short { limit } => todo!(),
            VideoDuration::Both { limit } => todo!(),
        }*/

        // TODO : ADD CLEANUP
        Ok(output_directory)
    }
}