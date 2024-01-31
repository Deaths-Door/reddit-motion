use std::process::Command;
use rand::Rng;

use crate::video_generator::{data::if_path_exists, VideoGenerator};
use super::{shared::SharedGeneratorLogic, utils::get_duration};

impl SharedGeneratorLogic {
    pub fn concat_audio_files(
        &self,
        video_generator : &VideoGenerator,
        bin_directory : &str
    ) -> std::io::Result<String> {
        let path = format!("{bin_directory}/concated_audio.mp3");

        if_path_exists!(not &path,{
            // ffmpeg -f concat -safe 0 -i concat.txt -c copy output.mp4
            video_generator.ffmpeg().ffmpeg_expect_failure(|cmd|{
                cmd.args([
                    "-f" , "concat",
                    "-i" , &self.audio_concat_file_path,
                    "-c" , "copy",
                    &path
                ]);
            })?
        });

        Ok(path)
    }

    pub fn prepare_background_music(
        &self,
        video_generator : &VideoGenerator,
        bin_directory : &str,
        concated_audio_length: &str
    ) -> std::io::Result<String> {
        // extend_background_music_to_concated_audio_length
        let output_file_path = format!("{bin_directory}/extended_music.mp3");

        // TODO : Add option to change volume of background music and audio files
        const FILTER_COMPLEX : &str = "volume=0.6";

        self.extend_media_to_duration(
            video_generator,
            &video_generator.audio_asset_directory, 
            &output_file_path,
            "0", 
            &concated_audio_length,
            true,
            None
          //  Some(FILTER_COMPLEX)
        )?;
        
        Ok(output_file_path)
    }

    pub fn prepare_background_video(
        &self,
        video_generator : &VideoGenerator,
        bin_directory : &str,
        concated_audio_length: &str,
        image_inputs : impl FnOnce(&mut Command)
    ) -> std::io::Result<String> {
        // crop_and_extend_video_to_concated_audio_length_andaddpngs
        let video_path = self.crop_and_move(video_generator,&bin_directory)?;

        // Start from random position within video length
        let duration = get_duration(&video_generator.ffmpeg(), &video_path)?;

        // Ensure start_duration is not a value near/at the end of the video
        // Hence the * 0.5
        let start_duration = rand::thread_rng().gen_range(0f64..duration * 0.5f64);
        let start_duration = start_duration.to_string();

        let temp_file_path = format!("{bin_directory}/temp_ext_video.mp4");

        self.extend_media_to_duration(
            video_generator,
            &video_path,
            &temp_file_path, 
            &start_duration, 
            &concated_audio_length,
            false,
            None
        )?;

        let output_file_path = format!("{bin_directory}/extended_video.mp4");

        // ffmpeg -i video -i image1 -i image2 -i image3
        //  -filter_complex
        //  "[0][1]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,23,27)'[v1];
        //   [v1][2]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,44,61)'[v2];
        //   [v2][3]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='gt(t,112)'[v3]"
        // -map "[v3]" -map 0:a  out.mp4
        video_generator.ffmpeg().ffmpeg_expect_failure(|cmd|{
            cmd.args(["-i", &temp_file_path]);

            image_inputs(cmd);
        
            cmd.args([
                "-filter_complex", &self.image_filter_complex,
                "-map", &format!("[v{index}]",index = self.image_index),
                &output_file_path
            ]);
        })?;


        Ok(output_file_path)
    }

    // TODO : REMOVE -y Flag and make it so it is in a new dir
    fn extend_media_to_duration(
        &self,
        video_generator : &VideoGenerator,
        input_file : &str,
        output_file: &str,
        start_duration : &str,
        end_duration : &str,
        is_audio : bool,
        // TODO : remove this??
        filter_complex : Option<&str>
    ) -> std::io::Result<()> {
        // TODO : ADD THIS BACK
        //if_path_exists!(input_file,return ok);

        let map_impl = match is_audio {
            true => "0:a:0",
            false => "0:v:0",
        };

        // ffmpeg-6.0\bin\ffmpeg.exe 
        // -stream_loop -1 
        // -i "C:\Users\Aarav Aditya Shah\Desktop\input.mp4" 
        // -ss 5 -t 60 
        // -map 0:v:0 out.mp4
        video_generator.ffmpeg().ffmpeg_expect_failure(|cmd|{
            cmd.args([
                "-stream_loop" , "-1",
                "-i" , input_file,
                "-ss" , start_duration,
                "-t", end_duration,
                "-map" , map_impl,
                output_file
            ]);

            if let Some(filter_complex) = filter_complex {
                cmd.args(["-filter_complex",filter_complex]);
            }
        }) 
    }
}

impl SharedGeneratorLogic {
    pub fn concat_video_with_audio(
        &self,
        video_generator : &VideoGenerator,
        output_directory : &str,
        video_path : &str,
        audio_path : &str
    ) -> std::io::Result<String> {
        let output_file_path = format!("{output_directory}/final_video.mp4");

        // ffmpeg -i video.mp4 -i audio.wav -c:v copy -c:a aac output.mp4
        video_generator.ffmpeg().ffmpeg_expect_failure(|cmd|{
            cmd.args([
                "-i" , video_path,
                "-i" , audio_path,
                "-c" , "copy",
                &output_file_path
            ]);
        })?;

        Ok(output_file_path)
    }

    pub fn combine_background_and_concated_audio(
        &self,
        video_generator : &VideoGenerator,
        bin_directory : &str,
        concated_audio : &str,
        background_audio: &str
    ) -> std::io::Result<String> {
        let output_file_path = format!("{bin_directory}/background_and_concated.mp3");

        if_path_exists!(not &output_file_path,{
            video_generator.ffmpeg().ffmpeg_expect_failure(|cmd|{
                cmd.args([
                    "-i" , concated_audio,
                    "-i" , background_audio,
                    "-filter_complex",
                    "[0:a]aformat=fltp:44100:stereo,apad[0a];[1]aformat=fltp:44100:stereo,volume=0.6[1a];[0a][1a]amerge[a]",
                    "-map" , "[a]",
                    "-ac" , "2",
                    &output_file_path
                ]);
            })?;
        });

        Ok(output_file_path)
    }
}