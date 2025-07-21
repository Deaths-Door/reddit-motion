mod post;
mod title;
mod utils;

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use chromiumoxide::error::CdpError;
use roux::{submission::SubmissionData, Subreddit};
use unic_langid::LanguageIdentifier;

use crate::{RedditBrowser, RedditPage, StoryMode, SubredditTaskConfiguration};

use super::VideoSourceFiles;

/// A struct representing an entry in the subreddit task pool.
///
/// This struct holds data for a specific submission, along with any
/// additional languages that the submission should be translated in.
#[derive(Debug)]

pub struct SubredditTaskEntry<'a> {
    pub(in crate::video) submission: SubmissionData,
    pub(in crate::video) extra_languages: Cow<'a, [unic_langid::LanguageIdentifier]>,
}

/// A struct representing a task related to a subreddit.
#[derive(Debug)]
pub struct SubredditTask {
    pub(in crate::video) story_mode: StoryMode,

    // TODO - Try to change this into a reference
    pub(in crate::video) language: Option<unic_langid::LanguageIdentifier>,
}

/// A struct representing a group of subreddit tasks.
///
/// This struct organizes multiple `SubredditTask` instances under a
/// specific Reddit page,
pub struct SubredditTaskGroup {
    pub(in crate::video) tasks: Vec<SubredditTask>,
    pub(in crate::video) resources: SubredditTaskGroupResources,
}

pub(crate) struct SubredditTaskGroupResources {
    pub(in crate::video) page: RedditPage,
    pub(in crate::video) submission: roux::submission::SubmissionData,
}

/// An error type representing failures related to story modes.
///
/// This error type is constructed when there are issues resolving story modes.
#[derive(thiserror::Error, Debug)]
pub struct StoryModeError(pub(in crate::video) StoryMode);

/// A struct representing a subreddit generator.
///
/// This struct manages the configuration for generating tasks related
/// to a specific subreddit and the associated task implementation.
#[derive(Debug)]
#[deprecated(note = "This has been replaced with traits. Remove this")]
pub struct SubredditGenerator<T>
where
    T: SubredditGenerationTask,
{
    pub(in crate::video) subreddit: SubredditTaskConfiguration,
    pub(in crate::video) task: T,
}

pub(crate) struct SourceAggregator<'a> {
    entry: &'a SubredditTask,
    resources: &'a SubredditTaskGroupResources,
    source_files: VideoSourceFiles,
}

/// A trait representing a generation task for subreddit entries.
///
/// This trait provides methods for creating new entries, handling task
/// creation failures, accessing the Reddit browser, and detecting the
/// language of submissions.
pub trait SubredditGenerationTask {
    /// The associated error type for this trait
    type Error: std::error::Error + From<StoryModeError> + From<CdpError>;

    /// Returns a reference to the Reddit browser associated with this task.
    fn reddit_browser(&self) -> &RedditBrowser;

    /// Returns a reference to the configuration
    fn subreddit_configuration(&self) -> &SubredditTaskConfiguration;

    type Tts: TextToSpeech;
    fn text_to_speech(&self) -> Self::Tts;

    type Translator: Translate;
    fn translator(&self) -> Self::Translator;

    /// Asynchronously creates a new entry for the specified subreddit.
    ///
    /// This method takes a mutable reference to `self` and a reference
    /// to a `Subreddit`. It returns a `Result` containing a new
    /// `SubredditTaskEntry` or an error.
    async fn new_entries<'a, 'l>(
        &'a mut self,
        subreddit: &Subreddit,
        count: u8,
    ) -> Result<Vec<SubredditTaskEntry<'l>>, Self::Error>;

    /// Retrieves cached entries for a specific subreddit.
    ///
    /// # Arguments
    /// - `subreddit`: A reference to the `Subreddit` for which the entries are being requested.
    /// - `wanted_count`: The number of entries that are needed from the cache.
    ///
    /// # Notes
    /// - The function returns a vector of `SubredditTaskEntry<'l>`, but may return fewer entries than `wanted_count` depending on the cache's availability.
    async fn get_cached_entries<'a, 'l>(
        &'a mut self,
        subreddit: &Subreddit,
        wanted_count: u8,
    ) -> Result<Vec<SubredditTaskEntry<'l>>, Self::Error>;

    /// Caches a list of subreddit task entries for later retrieval.
    ///
    /// # Arguments
    /// - `entries`: A vector of `SubredditTaskEntry<'l>` that represents the entries to be stored in the cache for future use.
    ///
    /// # Notes
    /// - This function is designed to cache task entries if they are not used so that subsequent calls to `get_cached_entries` can retrieve them.
    /// - The cache may persist the entries across multiple task runs, allowing for efficient reuse of entries.
    async fn cache_entries<'a, 'l>(
        &'a mut self,
        entries: Vec<SubredditTaskEntry<'l>>,
    ) -> Result<(), Self::Error>;

    /// Reports a failure encountered while creating a task.
    ///
    /// This method is called to handle any errors that occur during task
    /// creation, allowing the implementer to define custom error
    /// handling.
    fn report_creating_task_failed(&mut self, error: Self::Error);
}

/// A trait that defines an interface for converting text to speech and saving the audio output to a specified file path.

pub trait TextToSpeech {
    type Error: std::error::Error;

    /// Asynchronously converts the given text into speech and saves it to the specified file path.
    async fn save_speech_at(
        &self,
        text: &str,
        base_dir: &Path,
        // No extension
        file_name: &str,
    ) -> Result<PathBuf, Self::Error>;

    /// Adds noise to the beginning and end of a audio file
    /// to improve separation between segments (e.g., blockquotes).
    ///
    /// # Arguments
    /// * `audio_path` - Path to the audio file where white noise will be added.
    ///
    /// # Returns
    /// * A `Result` containing the updated audio path on success, or an error on failure.
    async fn add_noise(&self) -> Result<PathBuf, Self::Error>;

    /// Adds a silent gap (space) between consecutive audio segments to improve clarity.
    /// This is useful for separating paragraphs or distinct sections of text.
    ///
    /// # Arguments
    /// * `audio_path` - Path to the audio file where silence will be added.
    /// * `duration` - The duration of the silence in seconds.
    ///
    /// # Returns
    /// * A `Result` containing the updated audio path on success, or an error on failure.
    async fn add_silence(&self) -> Result<PathBuf, Self::Error>;
}

/// A trait for translating text asynchronously.
pub trait Translate {
    type Error: std::error::Error;
    /// Translates the given text asynchronously.
    async fn translate(&self, text: &str) -> Result<String, Self::Error>;
}

impl<'a> SourceAggregator<'a> {
    pub(crate) const fn new(
        base_directory: PathBuf,
        entry: &'a SubredditTask,
        resources: &'a SubredditTaskGroupResources,
    ) -> Self {
        Self {
            entry,
            resources,
            source_files: VideoSourceFiles::new(base_directory),
        }
    }
}

impl<'a> SubredditTaskEntry<'a> {
    /// Create new instance
    pub const fn new(
        submission: SubmissionData,
        extra_languages: Cow<'a, [unic_langid::LanguageIdentifier]>,
    ) -> Self {
        Self {
            submission,
            extra_languages,
        }
    }

    /// Get reference to [SubmissionData]
    pub const fn submission(&self) -> &SubmissionData {
        &self.submission
    }

    /// Get reference to a slice of [LanguageIdentifier]
    pub fn extra_languages(&self) -> &[LanguageIdentifier] {
        &self.extra_languages
    }
}
