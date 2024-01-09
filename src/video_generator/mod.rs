mod data;
mod gen;

use roux::submission::SubmissionData;
use unic_langid::LanguageIdentifier;
use std::path::PathBuf;

use crate::{ffmpeg::FFmpeg, config::{VideoCreationArguments, Dimensions}};

#[derive(Debug)]
pub struct VideoGenerationFiles {
    // Gen
    pub(in crate::video_generator) storage_directory : PathBuf,

    // audio + png dirs
    files : Vec<(String,String)>
}

pub struct VideoGenerator {
    video_gen_files : VideoGenerationFiles,
    ffmpeg : FFmpeg,
    dimensions: Dimensions,
    video_asset_directory : String,
    audio_asset_directory : String
}

impl VideoGenerationFiles {
    pub fn new_and_create_dir(submission : &SubmissionData,lang : &LanguageIdentifier) -> Self {
        let storage_directory = format!("bin/{name}/{id}/{lang}",name=submission.subreddit,id=submission.id);
        Self::new(storage_directory)
    }

    fn new(storage_directory: impl Into<PathBuf>) -> Self {
        Self { 
            storage_directory : storage_directory.into() , 
            files : Default::default() 
        }
    }
}

impl VideoGenerator {
    pub fn new(video_gen_files: VideoGenerationFiles, args : &VideoCreationArguments<'_> ) -> Self {
        let ffmpeg = args.ffmpeg.clone();
        let config = &args.config;
        let dimensions = config.dimensions.clone();    
        let video_asset_directory = config.assets.random_video_directory().to_owned();
        let audio_asset_directory = config.assets.random_audio_directory().to_owned();

        Self { video_gen_files, ffmpeg , dimensions , video_asset_directory , audio_asset_directory} 
    }
}