use crate::video_generator::VideoGenerator;

use super::{shared::SharedGeneratorLogic, utils::get_duration};

pub struct InfiniteVideoLength {
    shared_generator : SharedGeneratorLogic,
}

impl InfiniteVideoLength {
    pub const fn new(shared_generator: SharedGeneratorLogic) -> Self { Self { shared_generator } }

    pub fn exceute(
        mut self,
        video_generator : &VideoGenerator,
        bin_directory : &str,
        output_directory : &str
    ) -> std::io::Result<String> {
        let iter= || video_generator.video_gen_files.files.iter();

        for (audio_file,_) in iter() {
            self.shared_generator.append_audio(&audio_file)?;

            let duration = get_duration(&video_generator.ffmpeg,&audio_file)?;
            self.shared_generator.append_image(duration)
        }

        // So it does not clash with other generated files 
        let bin_directory = format!("{bin_directory}/infinite");
        std::fs::create_dir_all(&bin_directory)?;

        self.shared_generator.finalize_video(&video_generator,&bin_directory, output_directory, |cmd|{
            for (_,image_file) in iter() {
                cmd.args(["-i" , image_file]);
            }
        })
    }
}