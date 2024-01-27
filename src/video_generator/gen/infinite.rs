use std::ops::Deref;

use crate::video_generator::VideoGenerator;

use super::{shared::SharedGeneratorLogic, utils::get_duration_str};

pub struct InfiniteVideoLength<'a> {
    shared_generator : SharedGeneratorLogic<'a>
}

impl Deref for InfiniteVideoLength<'_> {
    type Target = VideoGenerator;
    fn deref(&self) -> &Self::Target {
        self.shared_generator.video_generator
    }
}

impl<'a> InfiniteVideoLength<'a> {
    pub const fn new(shared_generator: SharedGeneratorLogic<'a>) -> Self { Self { shared_generator } }

    pub fn exceute(mut self,bin_directory : &str,output_directory : &str) -> std::io::Result<String> {
        let iter= || self.video_gen_files.files.iter();

        for (audio_file,_) in iter() {
            self.shared_generator.append_audio(&audio_file)?;

            get_duration_str(
                &self.ffmpeg, 
                &audio_file,
                |duration| self.shared_generator.append_image(duration.parse().unwrap())
            )?;
        }

        self.shared_generator.finalize_video(bin_directory, output_directory, |cmd|{
            for (_,image_file) in iter() {
                cmd.args(["-i" , image_file]);
            }
        })
    }
}