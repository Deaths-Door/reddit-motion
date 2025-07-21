use std::num::NonZero;

use serde::{Deserialize, Serialize};

/// `RedditUser` represents a user's credentials and preferences on Reddit.
/// It stores the username, password, and the theme preference for the user.
#[derive(Serialize, Deserialize, Debug)]

pub struct RedditUser {
    /// The Reddit username associated with the user account.
    pub(in crate::reddit) username: String,
    /// The Reddit password associated with the user account.
    pub(in crate::reddit) password: String,
    /// The preferred theme for the Reddit interface (e.g., Dark or Light mode).
    pub(in crate::reddit) theme: Theme,
}

/// `Theme` represents the user's preferred visual theme for the Reddit interface.
#[derive(Serialize, Deserialize, Debug)]
pub enum Theme {
    /// Dark theme: Reddit's interface is displayed with darker colors, often preferred in low-light settings.
    Dark,
    /// Light theme: Reddit's interface is displayed with lighter colors, suitable for bright environments.
    Light,
}

/// `VideoDuration` represents the duration of a video. It can either be infinite
/// (for videos that don't have a predefined end) or limited to a specific time length.
#[derive(Clone, Copy, Default, Serialize, Deserialize, Debug)]
pub enum VideoDuration {
    /// Infinite duration: The video has no predefined end and will continue indefinitely.
    #[default]
    Infinite,
    /// Limited duration: The video has a specific time limit.
    Limited(VideoDurationLimit),
    ///  In this mode, the tool will generate two versions of the video: one with infinite duration and one with the specified `limit` in seconds.
    Both(VideoDurationLimit),
}

/// A struct representing a limit on video duration.

#[derive(Clone, Copy, Default, Serialize, Deserialize, Debug)]

pub struct VideoDurationLimit(f64);

/// `SubredditTaskConfiguration` stores the configuration for reading a specific subreddit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubredditTaskConfiguration {
    /// The name of the subreddit to be read.
    pub(crate) name: String,
    /// The number of times the subreddit should be processed/read.
    #[serde(default = "default_repeat")]
    pub(crate) repeat_count: NonZero<u8>,
    /// The mode in which the subreddit content will be read, based on the chosen story mode.
    #[serde(flatten)]
    pub(crate) story_mode: StoryMode,
}

fn default_repeat() -> NonZero<u8> {
    #[allow(unsafe_code)]
    unsafe {
        NonZero::new_unchecked(1)
    }
}

/// `StoryMode` defines the mode in which the content from the subreddit is read.
/// It supports different modes like automatically choosing between reading the post or comments,
/// reading a certain number of comments, or just reading the post itself.
#[derive(Clone, Default, Debug, strum::Display, Serialize, Deserialize, PartialEq, Eq)]
pub enum StoryMode {
    /// Default mode: Automatically chooses whether to read the post or comments, based on the content.
    #[default]
    #[strum(serialize = "auto")]
    Auto,
    /// Mode where a specific number of comments will be read from the post.
    #[strum(serialize = "comments")]
    ReadComments {
        ///  The maximum number of comments to read from the post.
        max_comments: Option<u8>,
    },
    /// Mode where only the post itself will be read, without reading any comments.
    #[strum(serialize = "post")]
    ReadPost,
}

impl Theme {
    /// Checks if the current theme is dark.
    pub const fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Checks if the current theme is light.
    pub const fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }
}

/// Dimensions of the generated Video
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dimensions {
    #[serde(default = "dwidth")]
    pub(crate) width: u32,
    #[serde(default = "dheight")]
    pub(crate) height: u32,
}

fn dwidth() -> u32 {
    800
}
fn dheight() -> u32 {
    600
}

impl Dimensions {
    /// Returns a tuple containing the width and height of the video as `(width, height)`.
    ///
    /// # Returns
    /// - A tuple `(u32, u32)` where the first value is the width and the second value is the height.
    ///
    /// # Example
    /// ```
    /// let dimensions = Dimensions { width: 1920, height: 1080 };
    /// assert_eq!(dimensions.width_height(), (1920, 1080));
    /// ```
    pub const fn width_height(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
