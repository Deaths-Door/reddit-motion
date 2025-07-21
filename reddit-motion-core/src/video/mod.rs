mod impls;
mod pool;
mod source;

use std::{fmt::Debug, path::PathBuf};

pub use impls::*;
pub use source::*;

use crate::Dimensions;

/// A struct representing a pool of subreddit tasks.
#[derive(Debug)]
pub struct SubredditTaskPool<T: TextToSpeech, L: Translate> {
    groups: Vec<SubredditTaskGroup>,
    resources: SubredditTaskPoolResources<T>,
}

pub(crate) type SubredditTaskPoolHidden<T> = SubredditTaskPool<
    <T as SubredditGenerationTask>::Tts,
    <T as SubredditGenerationTask>::Translator,
>;

#[derive(Debug)]
pub(crate) struct SubredditTaskPoolResources<T: TextToSpeech, L: Translate> {
    tts: T,
    translate: L,

    // TODO - Make this also a ref??
    dimensions: Dimensions,
}

/// `VideoSourceFiles` manages the collection of file associations
/// between audio files and image (PNG) files, stored in the order
/// they were provided. This struct serves as the central container
/// for all the file sources used in the video generation process.
#[derive(Debug)]
pub struct VideoSourceFiles {
    /// A collection of file associations between audio and PNG files.
    /// This collection stores the associations in the order they were
    /// added, ensuring that both Audio-to-PNG and PNG-to-Audio mappings
    /// are preserved.
    sources: Vec<FileAssociation>,

    base_directory: PathBuf,
}

impl VideoSourceFiles {
    /// Const version
    pub const fn new(base_directory: PathBuf) -> Self {
        Self {
            base_directory,
            sources: Vec::new(),
        }
    }
}

/// `AudioSource` represents the path to an audio file to be used in
/// the video generation process.
#[derive(Debug)]

pub struct AudioSource(std::path::PathBuf);

/// `ImageSource` represents the path to an image (PNG) file that is
/// associated with one or more audio files.
#[derive(Debug)]

pub struct ImageSource(std::path::PathBuf);

///
#[derive(Debug)]

pub enum FileAssociation {
    Single {
        audio: AudioSource,
        image: ImageSource,
    },

    /// for whitenoise/silence other music etc shit
    AudioOnly(AudioSource),
}
