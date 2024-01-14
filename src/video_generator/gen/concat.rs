use crate::{ffmpeg::FFmpeg, video_generator::data::if_path_exists};

/// Returns Created MP4 File || Path && Duration ||
pub(super) fn concat_media_files(
    index : usize,
    current_position : &f64,
    ffmpeg : &FFmpeg,
    mut temp_index_file : String,
    video_directory : &str,
    audio_directory : &str,
    png_directory : &str
) -> std::io::Result<(String,f64)> {
    // Firstly combine the audio with the video (max len == audiolen)
    // Later indefinitely add a png in the center of the created video 
    // return the output directory

    temp_index_file.push_str(&format!("/temp_{index}.mp4"));

    let video_asset_duration = super::utils::get_duration(ffmpeg,video_directory)?;
    let audio_duration = super::utils::get_duration(ffmpeg, audio_directory)?;


    // TODO : REMOVE THIS , temp
    if_path_exists!(not &temp_index_file,{
        std::fs::copy(video_directory, &temp_index_file)?; 
    });

    /*// So if video.len > audio.len then just extract that subvideo , as it results in an infinite loop
    if_path_exists!(not &temp_index_file,ffmpeg.ffmpeg_expect_failure(|cmd| {
        match video_asset_duration < audio_duration {
            // loop video till audio end
            // ffmpeg -stream_loop -1 -i video.mp4 -i audio.mp3 -c copy -shortest output.mp4
            true => cmd.args([
                "-stream_loop", "-1",
                "-i" , video_directory,
                "-i" , audio_directory,
                "-ss" , &current_position.to_string(),
                "-t" , &video_asset_duration.to_string(),
                "-shortest" , 
                "-c" , "copy",
                &temp_index_file
            ]),
            false => {
                cmd.args(
                    "-i" , video_directory,
                    "-ss" ,  &current_position.to_string(),
                    "-t" , &video_asset_duration.to_string(),
                )
            }
        }; 
    })?);

    todo!()*/
   // let out_index_file = temp_index_file.replace("temp_","");
    
    //center_screenshot_in_mp4(ffmpeg, &temp_index_file, png_directory, &out_index_file)?;

    let duration = super::utils::get_duration(ffmpeg, &temp_index_file)?;
    //let duration = super::utils::get_duration(ffmpeg, &out_index_file)?;
    Ok((temp_index_file,video_asset_duration))
    //Ok((out_index_file,video_asset_duration))
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