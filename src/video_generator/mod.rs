mod data;
mod gen;

use roux::submission::SubmissionData;
use unic_langid::LanguageIdentifier;
use std::path::PathBuf;
use crate::{ffmpeg::FFmpeg, config::{Dimensions, VideoCreationArguments, VideoDuration}};

#[derive(Debug)]
pub struct VideoGenerationFiles {
    // TODO : MAYBE USE A StrIng intead
    pub(in crate::video_generator) storage_directory : PathBuf,

    // TODO : Create new struct for it??
    // audio + png dirs
    files : Vec<(String,String)>
}

pub struct VideoGenerator<'a> {
    video_gen_files : VideoGenerationFiles,
    arguments: &'a VideoCreationArguments<'a>,
    video_duration : VideoDuration,

    /// Reference from [VideoGenerator.arguments.config.assets.random_video_directory]
    video_asset_directory : &'a str,

    /// Reference from [VideoGenerator.arguments.config.assets.random_audio_directory]
    audio_asset_directory : &'a str
}

impl VideoGenerationFiles {
    pub fn new_and_create_dir(submission : &SubmissionData,lang : &LanguageIdentifier) -> std::io::Result<Self> {
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

impl<'a> VideoGenerator<'a> {
    pub fn new(video_gen_files: VideoGenerationFiles, arguments : &'a VideoCreationArguments<'a>,video_duration : VideoDuration) -> Self {
        let config = &arguments.config;

        let video_asset_directory = config.assets.random_video_directory();
        let audio_asset_directory = config.assets.random_audio_directory();

        Self { video_gen_files ,arguments , video_duration , video_asset_directory , audio_asset_directory } 
    }

    pub const fn ffmpeg(&self) -> &FFmpeg {
        &self.arguments.ffmpeg
    }

    pub const fn dimensions(&self) -> &Dimensions {
        &self.arguments.config.dimensions
    }
}