
use crate::{video_generator::{data::if_path_exists, VideoGenerator}, ffmpeg::FFmpeg, config::Dimensions};

use super::shared::SharedGeneratorLogic;

impl SharedGeneratorLogic {
    pub(super) fn crop_and_move(&self,video_generator : &VideoGenerator,directory : &str) -> std::io::Result<String> {
        let video_file = format!("{directory}/video.mp4");

        if_path_exists!(not &video_file,{
            let video_dir = &video_generator.video_asset_directory;

            let dimensions = get_video_dimensions(
                &video_generator.ffmpeg,
                &video_dir
            )?;
        
            match video_generator.dimensions.width > dimensions.width || video_generator.dimensions.height > dimensions.height {
                true => { std::fs::copy(video_dir, &video_file)?; },
                false => { self.crop_video(video_generator,video_dir, &video_file)?; }
            }
        });

        Ok(video_file)
    }

    fn crop_video(
        &self,
        video_generator : &VideoGenerator,
        input : &str,
        file_path : &str
    ) -> std::io::Result<()> {
        if_path_exists!(file_path,return ok);

        // ffmpeg -i input.mp4 -filter:v "crop=w:h:x:y" output.mp4
        video_generator.ffmpeg.ffmpeg_expect_failure(|cmd|{
            let filter = format!("crop={}:{}",video_generator.dimensions.width,video_generator.dimensions.height);
            cmd.args([
                "-i", input, 
                "-vf", &filter, 
                file_path
            ]);
        })
    } 
}

fn get_video_dimensions(ffmpeg: &FFmpeg,file_path : &str) -> std::io::Result<Dimensions> {
    // ffprobe -v error -select_streams v -show_entries stream=width,height -of csv=p=0:s=x input.m4v
    let output = ffmpeg.ffprobe_expect_failure(|cmd|{
        cmd.args([
            "-v", "error", 
            "-select_streams", "v", 
            "-show_entries", "stream=width,height", 
            "-of", "csv=p=0:s=x", 
            file_path
        ]);
    })?;

    let string = String::from_utf8(output.stdout).unwrap();

    let mut split = string.split('x');
    let width = split.next()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    // Output contains an extra \n
    let height = split.next()
        .unwrap()    
        .trim_end()
        .parse::<u32>()
        .unwrap();

    assert!(split.next().is_none());

    Ok(Dimensions { width , height })
}