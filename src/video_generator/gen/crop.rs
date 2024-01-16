
use crate::{video_generator::{VideoGenerator, data::if_path_exists}, ffmpeg::FFmpeg, config::Dimensions};

impl VideoGenerator {
    pub(super) fn crop_and_move(&self,directory : &str) -> std::io::Result<String> {
        let video_file = format!("{directory}/video.mp4");

        if_path_exists!(not &video_file,{
            let video_dir = &self.video_asset_directory;

            let video_dimesions = get_video_dimensions(
                &self.ffmpeg,
                &video_dir
            )?;
        
            match self.dimensions.width > video_dimesions.width || self.dimensions.height > video_dimesions.height {
                true => { std::fs::copy(video_dir, &video_file)?; },
                false => { crop_video(&self.ffmpeg, &self.dimensions, video_dir, &video_file)?; }
            }
        });

        Ok(video_file)
    }
}

fn get_video_dimensions(ffmpeg: &FFmpeg,file_path : &str) -> std::io::Result<Dimensions> {
    // ffprobe -v error -select_streams v -show_entries stream=width,height -of csv=p=0:s=x input.m4v
    ffmpeg.ffprobe_expect_failure_map(|cmd|{
        cmd.args([
            "-v", "error", 
            "-select_streams", "v", 
            "-show_entries", "stream=width,height", 
            "-of", "csv=p=0:s=x", 
            file_path
        ]);
    },|o| {
        let string = String::from_utf8(o.stdout).unwrap();

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

        Dimensions { width , height }
    })
}

fn crop_video(ffmpeg: &FFmpeg,dimesions: &Dimensions,input : &str,file_path : &str) -> std::io::Result<()> {
    if_path_exists!(file_path,return ok);

    // ffmpeg -i input.mp4 -filter:v "crop=w:h:x:y" output.mp4
    ffmpeg.ffmpeg_expect_failure(|cmd|{
        let filter = format!("crop={}:{}",dimesions.width,dimesions.height);
        cmd.args([
            "-i", input, 
            "-vf", &filter, 
            file_path
        ]);
    })
} 