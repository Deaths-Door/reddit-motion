mod data;
mod gen;

use roux::submission::SubmissionData;
use unic_langid::LanguageIdentifier;
use std::path::PathBuf;

use crate::{ffmpeg::FFmpeg, config::{Dimensions, VideoCreationArguments, VideoDuration}};

#[derive(Debug)]
pub struct VideoGenerationFiles {
    // TODO : MAYBE USE A StrIng intead
    // Gen
    pub(in crate::video_generator) storage_directory : PathBuf,

    // audio + png dirs
    files : Vec<(String,String)>
}

// TODO : Figure out a way to use lifetimes to avoid cloning all the time
pub struct VideoGenerator {
    video_gen_files : VideoGenerationFiles,
    ffmpeg : FFmpeg,
    dimensions: Dimensions,
    video_duration : VideoDuration,
    video_asset_directory : String,
    audio_asset_directory : String
}

impl VideoGenerationFiles {
    pub fn new_and_create_dir(submission : &SubmissionData,lang : &LanguageIdentifier) -> std::io::Result<Self> {
        // TODO : why the fuck arent the media files being created in this directory
        let storage_directory = format!("bin/{name}/{id}/{lang}",name=submission.subreddit,id=submission.id);
        std::fs::create_dir_all(&storage_directory)?;
        Ok(
            Self { 
                storage_directory : storage_directory.into() , 
                files : Default::default() 
            }
        )
    }
}

impl VideoGenerator {
    pub fn new(video_gen_files: VideoGenerationFiles, args : &VideoCreationArguments<'_>,video_duration : &VideoDuration) -> Self {
        let ffmpeg = args.ffmpeg.clone();
        let config = &args.config;
        let dimensions = config.dimensions.clone();    
        let video_asset_directory = config.assets.random_video_directory().to_owned();
        let audio_asset_directory = config.assets.random_audio_directory().to_owned();
        let video_duration = (*video_duration).clone();

        Self { video_gen_files, ffmpeg , dimensions , video_asset_directory , audio_asset_directory , video_duration } 
    }
}