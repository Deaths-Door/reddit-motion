mod crop;
mod media;
mod utils;
mod shared;
mod infinite;
mod limited;

use crate::config::VideoDuration;

use self::{infinite::InfiniteVideoLength, limited::LimitedVideoLength, shared::SharedGeneratorLogic};

use super::VideoGenerator;

impl VideoGenerator {
    /// returns output video path
    pub async fn exceute(self) -> std::io::Result<String> {    
        let bin_directory = self.video_gen_files.storage_directory.display().to_string();

        // So instead of bin/.. do to generated_videos/..
        let output_directory = bin_directory.replace("bin", "generated_videos");
        std::fs::create_dir_all(&output_directory)?;

        if let VideoDuration::Infinite = self.video_duration {
            let shared_generator = SharedGeneratorLogic::new(&bin_directory)?;

            // TODO : CALL EXTERNAL SCRIPT
            InfiniteVideoLength::new(shared_generator)
                .exceute(&self,&bin_directory,&output_directory)?;
        }

        if let VideoDuration::Limited { limit } | VideoDuration::Both { limit } = self.video_duration {
            let shared_generator = SharedGeneratorLogic::new(&bin_directory)?;
            
            // TODO : CALL EXTERNAL SCRIPT
            LimitedVideoLength::new(shared_generator,limit)
                .exceute(&self,&bin_directory, &output_directory)?;
        }

        // TODO : CALL SOME external script eg to publish and split it for long videos
        // TODO : ALLOW CUSTOMIZABLE VOICE IN BACKGROUND MUSIC + MAIN AUDIO
        
        self.cleanup(bin_directory)?;

        Ok(output_directory)
    }
}