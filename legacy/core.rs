use std::{fs::File, io::Write, path::Path};

use crate::{ffmpeg::{FFmpeg, self}, video_generator::{VideoGenerator, data::if_path_exists}};



/// The goal is to create this command 
/// ffmpeg -i video -i image1 -i image2 -i image3
///  -filter_complex
///  "[0][1]overlay=x=X:y=Y:enable='between(t,23,27)'[v1];
///   [v1][2]overlay=x=X:y=Y:enable='between(t,44,61)'[v2];
///   [v2][3]overlay=x=X:y=Y:enable='gt(t,112)'[v3]"
/// -map "[v3]" -map 0:a  out.mp4
pub(super) struct CoreCreationCommand<'a> {
    filter_complex : String,
    png_input : Vec<&'a String>,
    current_index : u16,
    current_position : f64,
    start_position : f64,
    audio_concat_file : File,
    audio_concat_file_path : String
}


impl CoreCreationCommand<'_> {
    pub fn new(directory : &str,start_position : f64) -> std::io::Result<Self> {
        let file_path = format!("{directory}/concat.txt");

        let file = File::create(&file_path)?;

        Ok(Self {
            start_position,
            audio_concat_file : file,
            audio_concat_file_path : file_path,
            current_position : 0.0,
            current_index : 0,
            filter_complex : Default::default(),
            png_input : Default::default(),
        })
    }

    pub const fn current_position(&self) -> f64 {
        self.current_position
    }

    #[inline(always)]
    pub fn expected_new_position(&self,duration : f64) -> f64 {
       self.current_position() + duration
    }
}

impl<'a> CoreCreationCommand<'a> {
    pub fn extend_command(
        &mut self,
        png_file: &'a String,
        audio_file : &str,
        audio_file_duration : f64,
    ) -> std::io::Result<()> {
        // Update File
        write!(self.audio_concat_file,"file ")?;
        let name_bytes = Path::new(audio_file).file_name().unwrap().as_encoded_bytes();
        self.audio_concat_file.write_all(name_bytes)?;
        self.audio_concat_file.write_all("\n".as_bytes())?;

        // Extend Inputs 
        // -i {png_file}
       // self.png_input.push_str(" -i ");
        //self.png_input.push_str(png_file);
        self.png_input.push(png_file);

        // Extend Filter Complex
        let end = self.expected_new_position(audio_file_duration);
        let index = &mut self.current_index;
        let current_position_mut = &mut self.current_position;
        let plus_one = *index + 1;

        println!("index={index};current_position={current_position_mut};end={end}");

        // Update values
        *index = plus_one;
        *current_position_mut = end;

        println!("index={index};current_position={current_position_mut}");

        // for this part [v1][2]
        let prefix = match index == &0 {
            true => "",
            false => "v",
        };

        //[v1][2]overlay=x=X:y=Y:enable='between(t,23,27)'[v1];
        self.filter_complex.push_str(
            &format!("[{prefix}{index}][{plus_one}]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,{current_position_mut},{end})'[v{plus_one}];")
        );

        Ok(())
    }
}

impl CoreCreationCommand<'_> {
    pub fn merge_into_video(
        self,
        generator : &VideoGenerator,
        bin_directory : &str,
        output_directory : &str,
    ) -> std::io::Result<()> {
        // Extend Video
        // Combine Video + png
        // Add Background Music to it
        let extended_video_path = self.extend_video_to_limit(generator,bin_directory)?;
        let temperory_video_with_images = self.update_video_with_images(
            generator,
            bin_directory,
            &extended_video_path
        )?;


        let concated_audio_path = self.concat_audio_files(generator,bin_directory)?;
        let video_and_audio_merged = todo!();

        let final_video = self.add_background_music(
            generator, 
            output_directory,
            &video_and_audio_merged, 
        )?;

        Ok(())
    }

    fn extend_video_to_limit(&self,generator : &VideoGenerator,bin_directory : &str) -> std::io::Result<String> {
        let path = format!("{bin_directory}/extended_video.mp4");
        
        if_path_exists!(not &path,{
            // ffmpeg-6.0\bin\ffmpeg.exe 
            // -stream_loop -1 
            // -i "C:\Users\Aarav Aditya Shah\Desktop\input.mp4" 
            // -ss 5 -t 60 
            // -map 0:v:0 out.mp4
            generator.ffmpeg.ffmpeg_expect_failure(|cmd|{
                cmd.args([
                    "-stream_loop" , "-1",
                    "-i" , &generator.video_asset_directory,
                    // Start Video from start pos
                    "-ss" , &self.start_position.to_string(),
                    // Current position as the audiolength may not be the video_limit (rn)
                    "-t", &self.current_position.to_string(),
                    "-map" , "0:v:0",
                    &path
                ]);

                println!("extend_video_command={:?}",cmd);
            })?;  
        });

        Ok(path)
    }


    fn concat_audio_files(
        &self,
        generator : &VideoGenerator,
        bin_directory : &str
    ) -> std::io::Result<String> {
        let path = format!("{bin_directory}/concated_audio.mp3");

        if_path_exists!(not &path,{
            // ffmpeg -f concat -safe 0 -i concat.txt -c copy output.mp4
            generator.ffmpeg.ffmpeg_expect_failure(|cmd|{
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

    fn update_video_with_images(
        &self,
        generator : &VideoGenerator,
        bin_directory : &str,
        extended_video : &str
    ) -> std::io::Result<String> {
        let path = format!("{bin_directory}/video_with_images.mp3");

        if_path_exists!(not &path,{
            generator.ffmpeg.ffmpeg_expect_failure(|cmd|{
                let vindex = format!("[v{}]",self.current_index);

                cmd.args(["-i" , extended_video]);

                for input in &self.png_input {
                    cmd.args(["-i",&input]);
                }

                cmd.args([
                    "-filter_complex" , &self.filter_complex,
                    "-map" , &vindex,
                    "-map" , "0:a",
                    &path
                ]);
            })?;
        });

        Ok(path)
    }

    fn add_background_music(
        &self,
        generator : &VideoGenerator,
        video : &str,
        video_count : &u16
    ) -> std::io::Result<String> {
        let output_directory = format!("video_{}",self.current_index);
        /*
        ffmpeg 
        -i "C:\Users\Aarav Aditya Shah\Downloads\input.mp4" 
        -i "C:\Users\Aarav Aditya Shah\Music\Alesso - When I’m Gone (with Katy Perry).mp3" 
        -c:v copy  
        -filter_complex "[0:a]aformat=fltp:44100:stereo,apad[0a];[1]aformat=fltp:44100:stereo,volume=1.5[1a];[0a][1a]amerge[a]" 
        -map 0:v -map "[a]" 
        -ac 2 
        "output.mp4" */
        generator.ffmpeg.ffmpeg_expect_failure(|cmd|{
            cmd.args([
                "-i" , video,
                "-i" , &generator.audio_asset_directory,
                "-c:v" , "copy",
                "-filter_complex", 
                "[0:a]aformat=fltp:44100:stereo,apad[0a];[1]aformat=fltp:44100:stereo,volume=0.6[1a];[0a][1a]amerge[a]",
                "-map" , "0:v",
                "-map" , "[a]",
                "-ac" , "2",
                &output_directory
            ]);
        })?;

        Ok(output_directory)
    }
}