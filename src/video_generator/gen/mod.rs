mod crop;
mod media;
mod utils;
mod shared;
mod infinite;
mod limited;

use crate::config::VideoDuration;

use self::{infinite::InfiniteVideoLength, limited::LimitedVideoLength, shared::SharedGeneratorLogic};

use super::VideoGenerator;

impl VideoGenerator<'_> {
    /// returns output video path
    pub async fn exceute(self) -> std::io::Result<String> {    
        let bin_directory = self.video_gen_files.storage_directory.display().to_string();

        // So instead of bin/.. do to generated_videos/..
        let output_directory = bin_directory.replace("bin", "generated_videos");
        std::fs::create_dir_all(&output_directory)?;

        let scripts = self.scripts();
        let mut infinite_process = None;
        let mut limited_process = None;
        
        if let VideoDuration::Infinite = self.video_duration {
            let shared_generator = SharedGeneratorLogic::new(&bin_directory)?;

            let file_path = InfiniteVideoLength::new(shared_generator)
                .exceute(&self,&bin_directory,&output_directory)?;
            
            infinite_process = scripts.call_infinite_script(&file_path)
        }

        if let VideoDuration::Limited { limit } | VideoDuration::Both { limit } = self.video_duration {
            let shared_generator = SharedGeneratorLogic::new(&bin_directory)?;
            
            let file_paths = LimitedVideoLength::new(shared_generator,limit)
                .exceute(&self,&bin_directory, &output_directory)?;

            limited_process = scripts.call_limited_script(&file_paths)
        }

        // TODO : ALLOW CUSTOMIZABLE VOICE IN BACKGROUND MUSIC + MAIN AUDIO
        self.cleanup(bin_directory)?;

        let callback = self.arguments();
        let infinite_script = scripts.infinite_script();
        let limited_script = scripts.limited_script();

        // Don't show success if both fail
        match utils::handle_child_proccess(infinite_script,infinite_process,callback) 
            && utils::handle_child_proccess(limited_script,limited_process,callback) {
            true => Ok(output_directory),
            false => Err(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "The execution of both external scripts failed"
            ))
        }
    }
}