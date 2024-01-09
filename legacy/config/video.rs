use std::fs::File;

use rand::{Rng, distributions::Alphanumeric};

use crate::{ffmpeg::FFmpeg, config::Dimesions};

pub(in crate::config) fn get_video_dimensions(ffmpeg : &FFmpeg,path : &str) -> std::io::Result<Dimesions> {
    // ffprobe -v error -select_streams v -show_entries stream=width,height -of csv=p=0:s=x input.m4v
    ffmpeg.ffprobe_expect_failure_map(|cmd|{
        cmd.args(["-v", "error", "-select_streams", "v", "-show_entries", "stream=width,height", "-of", "csv=p=0:s=x", path]);
    },|o| {
        let string = String::from_utf8(o.stdout).unwrap();

        let mut split = string.split('x');
        let width = split.next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        // Output contains an extra \n
        let mut _height = split.next().unwrap();
        _height = &_height[0.._height.len() - 2];
        let height = _height    
            .parse::<u32>()
            .unwrap();

        assert!( plit.next().is_none());

        Dimesions { width , height }
    })
}

pub(in crate::config) fn crop_video(ffmpeg: &FFmpeg,dimesions: &Dimesions,input : &str,output : &str) -> std::io::Result<()> {
    // ffmpeg -i input.mp4 -filter:v "crop=w:h:x:y" output.mp4
    ffmpeg.ffmpeg_expect_failure(|cmd|{
        let filter = format!("crop={}:{}",dimesions.width,dimesions.height);
        cmd.args([
            "-i", input, 
            "-vf", &filter, 
            output
        ]);
    })
} 

fn combine_and_repeat_video_till_audio_end(
    ffmpeg: &FFmpeg,
    start : u32,
    mp4_file : &str,
    mp3_file : &str,
    output_mp4 : &str
) -> std::io::Result<()> {
    // ffmpeg -i input.mp4 -i input.mp3 -shortest -async 1 -map 0:v:0 -map 1:a:0 -y out.mp4
    // ffmpeg -i input.mp4 -i input.mp3 -shortest -map 0:v:0 -map 1:a:0 -y out.mp4
    // ffmpeg  -stream_loop -1 -i input.mp4 -i input.mp3 -ss 10 -shortest -map 0:v:0 -map 1:a:0 -y out.mp4
    ffmpeg.ffmpeg_expect_failure(|cmd|{
        cmd.args([
//            "-stream_loop", "-1", 
            "-i", mp4_file, 
            "-i", mp3_file, 
            "-shortest",
            "-map", "0:v:0",
            "-map", "1:a:0",
            "-y",
            //"-ss", &start.to_string() , 
        //     "-c" ,"copy",
            output_mp4
        ]);
    })
}

pub(in crate::config) fn combine_and_repeat_audio_till_video_end(
    ffmpeg : &FFmpeg,
    mp4_file : &str,
    mp3_file : &str,
    output_mp4 : &str
) -> std::io::Result<()> {
    //ffmpeg  -i input.mp4 -stream_loop -1 -i input.mp3 -shortest -map 0:v:0 -map 1:a:0 -y out.mp4
    ffmpeg.ffmpeg_expect_failure(|cmd|{
        cmd.args([
            "-i" , mp4_file ,
            "-stream_loop", "-1", 
            "-i", mp3_file,
            "-shortest", "-map", "0:v:0", 
            "-map", "1:a:0", "-y", 
            output_mp4
        ]);
    })
}

fn center_screenshot_in_mp4(
    ffmpeg : &FFmpeg,
    mp4_file : &str,
    png_file : &str,
    output_mp4 : &str
) -> std::io::Result<()> {
    // ffmpeg -i "C:\Users\Aarav Aditya Shah\Downloads\input.mp4" 
    // -i  "C:\Users\Aarav Aditya Shah\Desktop\short.jpg" 
    // -filter_complex [0][1]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2 "output.mp4"
    ffmpeg.ffmpeg_expect_failure(|cmd|{
        cmd.args([
            "-i", mp4_file,
            "-i", png_file,
            "-filter_complex", "[0][1]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2",
            output_mp4
        ]);
    })
}

#[deprecated]
pub(in crate::config) fn random_file_name() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}



#[deprecated]
pub(in crate::config) fn combine_video_audio_and_png(
    ffmpeg : &FFmpeg,
    id : &str,
    bin_directory : &str,
    video_directory : &str,
    audio_directory : &str,
    png_directory : &str,
) -> std::io::Result<String> {
    let temp_mp4 =  format!("{bin_directory}/temp{id}.mp4");

    super::if_path_exists!(not &temp_mp4,combine_and_repeat_video_till_audio_end(
        ffmpeg,
        0, //start ranodm, TODO MAYBE
        &video_directory,
        &audio_directory,
        &temp_mp4
    )?);

    let output_mp4 = format!("{bin_directory}/{id}.mp4");

    super::if_path_exists!(not &output_mp4,center_screenshot_in_mp4(
        &ffmpeg,
        &temp_mp4,
        &png_directory,
        &output_mp4
    )?);

  //  std::fs::remove_file(temp_mp4)?;

    Ok(output_mp4)
}

pub(in crate::config) fn concat_videos(
    ffmpeg : &FFmpeg,
    dirs : &[String],
    subreddit_directory : String,
    output_mp4 : &str
) -> std::io::Result<()> {
    // create the concat.txt files to concat all the vids
    let name = random_file_name();
    let path = format!("{subreddit_directory}/{name}.txt");

    let mut file = File::create(&path)?;

    for dir in dirs {
        use std::io::Write;
        write!(file,"file '{dir}'")?;
    }
    
    // ffmpeg -f concat -safe 0 -i concat.txt -c copy output.mp4
    ffmpeg.ffmpeg_expect_failure(|cmd|{
        cmd.args([
            "-f", "concat",
            "-safe", "0",
            "-i" , &path,
            "-c","copy",
            output_mp4
        ]);
    })
}