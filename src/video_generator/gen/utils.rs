use crate::{ffmpeg::FFmpeg, video_generator::VideoGenerator};

use super::shared::SharedGeneratorLogic;

pub(crate) fn get_duration_str<'a,T>(ffmpeg : &FFmpeg,file_path : &str,map : impl FnOnce(&str) -> std::io::Result<T>) -> std::io::Result<T> {
    // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1 fifa.mp4
    let output = ffmpeg.ffprobe_expect_failure(|cmd|{
        cmd.args([
            "-v" , "error",
            "-show_entries" , "format=duration",
            "-of" , "default=noprint_wrappers=1",
            "-i" , file_path,
        ]);
    })?;

    map(
        String::from_utf8(output.stdout)
            .unwrap()
            .strip_prefix("duration=")
            .unwrap()
            .trim_end()
    )
}

impl VideoGenerator {
    pub(super) fn infinte_video_duration(
        &self,
        bin_directory : &str,
        output_directory : &str
    ) -> std::io::Result<()> {
        let mut shared_generator = SharedGeneratorLogic::new(self,bin_directory)?;

        let iter= || self.video_gen_files.files.iter();
        for (audio_file,_) in iter() {
            shared_generator.append_audio(&audio_file)?;

            get_duration_str(
                &self.ffmpeg, 
                &audio_file,
                |duration| shared_generator.append_image(duration.parse().unwrap())
            )?;
        }

        let concated_audio = shared_generator.concat_audio_files(bin_directory)?;
        
        super::utils::get_duration_str(
            &self.ffmpeg,
            &concated_audio,
            |concated_audio_length| {
                let background_audio = shared_generator.prepare_background_music(bin_directory,concated_audio_length)?;
                let video_path = shared_generator.prepare_background_video(bin_directory,concated_audio_length,|cmd|{
                    for (_,image_file) in iter() {
                        cmd.args(["-i" , image_file]);
                    }
                })?;

                // TODO : Use this https://filmora.wondershare.com/video-editor/ffmpeg-merge-audio-and-video.html 
                // to fix this :  Have to do this as can't find a way to successfully create video with audio in a single command
                let audio_path = shared_generator.combine_background_and_concated_audio(bin_directory,&background_audio,&concated_audio)?;
                let final_video = shared_generator.concat_video_with_audio(
                    output_directory, 
                    &video_path, 
                    &audio_path
                )?;

                // TODO : CALL SOME external script eg to publish and split it for long videos

                Ok(())
            }
        )
    }
}