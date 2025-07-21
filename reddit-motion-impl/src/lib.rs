//! implementation for reddit-motion
mod external_impl;
mod models;
mod schema;
mod utils;

use std::{future::Future, pin::pin};

use futures::{
    lock::Mutex,
    stream::{FuturesUnordered, SelectAll}, Stream,
};

use diesel::Connection;
use futures::FutureExt;
use reddit_motion_core::{
    chromiumoxide::Handler, Dimensions, ExceuteTask, RedditBrowser, RedditError, RedditResult,
    RedditUser, StoryModeError, SubredditGenerationTask, SubredditGenerator, SubredditTask,
    VideoDuration,
};
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

/// Represents a wrapper around a Diesel SQLite connection.
#[allow(missing_debug_implementations)]
pub struct DatabaseConnection(diesel::prelude::SqliteConnection);

/// Manages Reddit tasks, encapsulating the configuration, a Reddit browser instance, and access to the database.
///
/// This struct is responsible for handling tasks related to Reddit, including interaction with the Reddit API and
/// managing subreddit entries in the database.
#[allow(missing_debug_implementations)]
pub struct RedditTaskInducer {
    /// Stores user-specific Reddit task configurations, which includes subreddits, dimensions, and user-related information.
    configuration: UserRedditTaskConfiguration,

    /// To have 'partial' borrows
    resources: RedditTaskResources,
}

pub(crate) struct RedditTaskResources {
    /// The `RedditBrowser` handles interaction with Reddit (e.g., making requests to the API, retrieving posts).
    browser: RedditBrowser,
    /// A thread-safe (`Mutex`) wrapper around the `DatabaseConnection`, which is used to interact with the underlying SQLite database.
    database_connection: Mutex<DatabaseConnection>,
}

/// Configuration data for a user’s Reddit tasks, including subreddits to process, display dimensions, and optional user information.
/// This configuration guides the execution of Reddit tasks for a given user.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRedditTaskConfiguration {
    /// Optional Reddit user information that may be associated with the task configuration.
    user: Option<RedditUser>,

    /// A collection of subreddit-specific configurations that define how each subreddit should be processed.
    subreddits: Vec<UserSubredditConfig>,

    /// The dimensions associated with the task, potentially relating to the display or processing characteristics.
    dimensions: Dimensions,
}

/// Represents errors that may occur during the Reddit task execution.
///
/// The enum variants encapsulate different types of errors, including issues with Reddit API interaction,
/// resolving story modes, and database-related errors.
#[derive(thiserror::Error, Debug)]
pub enum RedditTaskError {
    /// Represents an error that occurs while interacting with Reddit.
    #[error("{0}")]
    Reddit(#[from] RedditError),

    /// Represents an error that occurs when resolving story modes (likely related to specific task modes).
    #[error("{0}")]
    ResolvingStoryMode(#[from] StoryModeError),

    /// Represents an error that occurs when interacting with the database (e.g., retrieving entries).
    #[error("{0}")]
    DatabaseError(#[from] DatabaseError),
}

/// An error that occurs when converting from/to [`roux::SubmissionData`].
///
/// This error is encountered when attempting to serialize or deserialize [`roux::SubmissionData`]
/// and encounter issues during the conversion.
#[derive(thiserror::Error, Debug)]
#[error("Failed to convert roux::SubmissionData : {0}")]
pub struct SubmissionDataConversionError(#[from] serde_json::Error);

///
#[derive(thiserror::Error, Debug)]
#[error("Failed to convert to SubredditTaskEntry : {0}")]
pub struct ToSubredditTaskEntryConversionError(#[from] serde_json::Error);

/// Enum representing errors that can occur while interacting with the database.
///
/// Variants include errors related to retrieving cached subreddit entries and issues with data conversion.
#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    /// Represents an error when retrieving cached entries from the database.
    #[error("{0}")]
    RetrievingCachedEntries(#[from] RetrievingCachedEntriesError),

    ///
    #[error("{0}")]
    CachingEntries(#[from] CachingEntriesError),

    ///
    #[error("{0}")]
    Conversion(#[from] DataConversionError),
}

///
#[derive(thiserror::Error, Debug)]
pub enum DataConversionError {
    ///
    #[error("{0}")]
    SubredditEntry(#[from] SubmissionDataConversionError),
    ///
    #[error("{0}")]
    SubredditTaskEntry(#[from] ToSubredditTaskEntryConversionError),
}

/// Represents an error that occurs while retrieving cached entries from the database.
/// This wraps Diesel's result error and exists so that the there is little chance the original error
/// is categorized wrongly
#[derive(thiserror::Error, Debug)]
#[error("Failed to retrieve cached entries from the database: {0}")]

pub struct RetrievingCachedEntriesError(#[from] diesel::result::Error);

/// Represents an error that occurs whiel cachin entries to the database.
/// This wraps Diesel's result error and exists so that the there is little chance the original error
/// is categorized wrongly
#[derive(thiserror::Error, Debug)]
#[error("Failed caching entries to the database: {0}")]

pub struct CachingEntriesError(#[from] diesel::result::Error);

/// Represents configuration for a specific subreddit task for a user.
///
/// The struct encapsulates various details, including subreddit-specific configurations, extra languages,
/// and video duration.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct UserSubredditConfig {
    /// External subreddit task configuration, inherited from a broader task configuration system.
    #[serde(flatten)]
    external: reddit_motion_core::SubredditTaskConfiguration,

    /// A list of additional languages that may be processed for this subreddit task.
    #[serde(default)]
    extra_languages: Vec<LanguageIdentifier>,

    /// The duration of videos to be processed or generated for this task.
    #[serde(default)]
    video_duration: VideoDuration,
}

/// Represents a task for a specific subreddit, which is linked to a user.
/// The task interacts with the `RedditTaskInducer` to carry out its operations.
///
/// This struct encapsulates the configuration for the subreddit, as well as the offset for submissions,
/// and a reference to the `RedditTaskInducer` that manages the task.
pub(crate) struct SubredditIndividualTask<'a> {
    /// Configuration for the specific subreddit task.
    configuration: UserSubredditConfig,
    resources: &'a RedditTaskResources,
}

impl DatabaseConnection {
    /// Creates a new `DatabaseConnection` by establishing a connection to the SQLite database.
    ///
    /// This function uses environment variables (via dotenv) to fetch the database URL and establish
    /// the connection using Diesel. It returns a `DatabaseConnection` if successful.
    ///
    /// # Errors
    /// Returns an error if the database URL is not set or if the connection cannot be established.
    pub fn new() -> diesel::ConnectionResult<Self> {
        assert!(dotenvy::dotenv().is_ok());
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = diesel::prelude::SqliteConnection::establish(&database_url)?;
        Ok(Self(connection))
    }
}

impl RedditTaskInducer {
    /// Asynchronously creates a new `RedditTaskInducer` with the specified configuration and database connection.
    ///
    /// This function initializes the Reddit task inducer, which manages Reddit-related tasks,
    /// database operations, and the Reddit browser.
    ///
    /// # Arguments
    /// * `configuration` - User-specific Reddit task configurations.
    /// * `database_connection` - The `DatabaseConnection` used to interact with the SQLite database.
    ///
    /// # Returns
    /// A tuple containing the newly created `RedditTaskInducer` and a task handler.
    pub async fn new(
        configuration: UserRedditTaskConfiguration,
        database_connection: DatabaseConnection,
    ) -> RedditResult<(Self, Handler)> {
        let (browser, handler) = Self::create_browser(&configuration.dimensions).await?;
        let browser = RedditBrowser::new(configuration.user.as_ref(), browser).await?;

        Ok((
            Self {
                configuration,
                resources: RedditTaskResources {
                    database_connection: Mutex::new(database_connection),
                    browser,
                },
            },
            handler,
        ))
    }

    ///
    pub async fn start_tasks(self) ->impl Stream<Item = ()> {
        let resources = &self.resources;
        let futures = FuturesUnordered::new();

        for subreddit in self.configuration.subreddits {
            let task = SubredditIndividualTask {
                configuration: subreddit,
                resources: resources,
            };

            if let Some(pool) = task.create_pool().await {
                futures.extend(pool.into_iter().map(|group| async move { group.exceute() }));
            }
        }
    }


    futures
}
