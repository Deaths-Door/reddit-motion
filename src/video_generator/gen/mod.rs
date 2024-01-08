use crate::{config::{VideoCreationArguments, Config, VideoCreationError}, ffmpeg::FFmpeg};

use super::VideoGenerationFiles;



impl VideoGenerationFiles {
    // returns output video path
    pub async fn exceute_generation(self,ffmpeg : FFmpeg) -> std::io::Result<String> {
        // TODO
        todo!()
    }
}