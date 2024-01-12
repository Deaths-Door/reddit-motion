use crate::{ffmpeg::FFmpeg, video_generator::data::if_path_exists};

/// Returns Created MP4 File || Path && Duration ||
pub(super) fn concat_media_files(
    index : usize,
    current_position : &f64,
    ffmpeg : &FFmpeg,
    video_directory : &str,
    audio_directory : &str,
    png_directory : &str
) -> std::io::Result<(String,f64)> {
    // Firstly combine the audio with the video (max len == audiolen)
    // Later indefinitely add a png in the center of the created video 
    // return the output directory

    // ffmpeg  -stream_loop -1 -i input.mp4 -i input.mp3 -ss 10 -shortest -map 0:v:0 -map 1:a:0 -y out.mp4
    // + t to restrict max duration
    let vid_duration = super::utils::get_duration(ffmpeg,video_directory)?;

    let mut out_index_directory = format!("temp_{index}.mp4");

    if_path_exists!(not &out_index_directory,ffmpeg.ffmpeg_expect_failure(|cmd|{
        cmd.args([
            "-stream_loop", "-1",
            "-i" , video_directory,
            "-i" , audio_directory,
            "-ss" , &current_position.to_string(),
            "-t" , &vid_duration.to_string(),
            "-shortest" , 
            "-map", "0:v:0" , 
            "-map" , "1:a:0",
            "-y" , &out_index_directory
        ]);
    })?);

    // 0..=5 as its the len of temp_
    out_index_directory.replace_range(0..=5,"");
    
    center_screenshot_in_mp4(ffmpeg, &out_index_directory, png_directory, &out_index_directory)?;

    let duration = super::utils::get_duration(ffmpeg, &out_index_directory)?;

    Ok((out_index_directory,duration))
}


fn center_screenshot_in_mp4(
    ffmpeg : &FFmpeg,
    mp4_file : &str,
    png_file : &str,
    output_mp4 : &str
) -> std::io::Result<()> {
    if_path_exists!(output_mp4,return ok);

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


pub(super) fn concat_for_mp4s(
    ffmpeg : &FFmpeg,
    txt_path : &str,
    output_path : &str
) -> std::io::Result<()> {
    if_path_exists!(output_path,return ok);

    // ffmpeg -f concat -safe 0 -i concat.txt -c copy output.mp4

    ffmpeg.ffmpeg_expect_failure(|cmd|{
        cmd.args([
            "-f" , "concat",
            "-safe", "0",
            "-i" , txt_path,
            "-c" , "copy",
            output_path
        ]);
    })
}