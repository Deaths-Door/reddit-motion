mod crop;

use crate::{config::{VideoCreationArguments, Config, VideoCreationError}, ffmpeg::FFmpeg};

use super::{VideoGenerationFiles, VideoGenerator};

impl VideoGenerator {
    // returns output video path
    pub async fn exceute(self) -> std::io::Result<String> {
        let storage_directory = self.video_gen_files.storage_directory.display().to_string();

        let video_directory = self.crop_and_move(storage_directory)?;
        // TODO
        todo!()
    }
}