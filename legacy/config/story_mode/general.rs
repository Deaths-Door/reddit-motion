use std::collections::HashMap;
use std::path::Path;

use chromiumoxide::Page;
use roux::submission::SubmissionData;
use unic_langid::LanguageIdentifier;
use crate::config::{video, tts, screenshot};
use crate::config::{VideoCreationError,ParameterArgs};
use crate::ffmpeg::FFmpeg;
use super::{StoryMode,StoryModeError, MediaFiles};

pub struct StoryModeParmeters<'a> {
    pub parms : &'a ParameterArgs<'a>,
    pub bin_directory : &'a str,
    pub submission : &'a SubmissionData,
    pub page : &'a Page,
}

impl StoryMode {
    pub(super) async fn read_mode(
        parms : &StoryModeParmeters<'_>,
        cond : impl FnOnce(&SubmissionData) -> (bool,StoryModeError)
    ) -> Result<(),VideoCreationError> {
        let submission = parms.submission;
        let bin_directory = parms.bin_directory;

        let (is_error,future_error) = cond(submission);
        if is_error { return Err(VideoCreationError::from(future_error)) }

        std::fs::create_dir_all(bin_directory)?;

        (parms.parms.callback.info)(submission);

        let video_directory = random_video_crop_and_move_to_directory(parms.parms, &bin_directory)?;

        let media_files = MediaFiles::default();

        
       /*generate_and_concentate_media(
            parms,
            &submission.id,
            &submission.title,
            &video_directory,
            |file_name| async move {
                let s = screenshot::screenshot_post_title(parms.page, submission,parms.bin_directory, &file_name).await?;
                Ok(s)
            }
        ).await?;*/

       // parms.bin_directory;


        // TODO: FINSIH THIS
        Ok(())
    }
}


fn random_video_crop_and_move_to_directory(
    parms : &ParameterArgs,
    bin_directory : &str
) -> std::io::Result<String> {
    let video_directory = parms.assets.random_video_directory();

    let video_dimesions = video::get_video_dimensions(
        &parms.ffmpeg,
        video_directory
    )?;

    let new_vdir  = format!("{bin_directory}/video.mp4");

    super::if_path_exists!(&new_vdir, return new_vdir);

    if parms.dimensions.width > video_dimesions.width || parms.dimensions.height > video_dimesions.height {
        std::fs::copy(video_directory, &new_vdir)?; 
    } else { video::crop_video(&parms.ffmpeg, parms.dimensions, video_directory, &new_vdir)?; } 

    Ok(new_vdir)
}


#[deprecated]
async fn generate_and_concentate_media<O>(
    parms : &StoryModeParmeters<'_>,
    id : &str,
    text : &str,
    video_directory : &str,
    take_screen_shot : impl FnOnce(String) -> O
) -> Result<String,VideoCreationError> where O : std::future::Future<Output = Result<String,VideoCreationError>>  {
    let audio_directory = tts::save_to_file(
        &parms.bin_directory,
         &format!("{id}.mp3"),
         text
    )?;    

    let png_directory = take_screen_shot(format!("{id}.png")).await?;

    let final_dir = video::combine_video_audio_and_png(
        &parms.parms.ffmpeg, 
        id,
        parms.bin_directory, 
        &video_directory, 
        &audio_directory, 
        &png_directory
    )?;

    Ok(final_dir)
}

