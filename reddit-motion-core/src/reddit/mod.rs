mod config;
mod user;

use std::ops::Deref;

use chromiumoxide::{handler::viewport::Viewport, Browser, BrowserConfig, Handler};
pub use config::*;
pub use user::*;

/// A specialized `Result` type for Reddit operations, where the success type is `T`
/// and the error type is `RedditError`.
pub type RedditResult<T> = core::result::Result<T, RedditError>;

/// An enumeration representing possible errors that can occur while interacting
/// with Reddit, using the `thiserror` crate for simplified error handling.
#[derive(thiserror::Error, Debug)]
pub enum RedditError {
    /// Error variant representing a login failure. It wraps a `RedditLoginError`.
    #[error("{0}")]
    Login(#[from] RedditLoginError),

    /// Error variant representing a theme-related failure. It wraps a `RedditThemeError`.
    #[error("{0}")]
    Theme(#[from] RedditThemeError),

    /// A generic error variant that can wrap any error from the `chromiumoxide` library,
    /// specifically a `CdpError`.
    #[error("{0}")]
    Chromiumoxide(#[from] chromiumoxide::error::CdpError),

    /// Error returned when browser detection fails.

    #[error("{0}")]
    ChromeDetectionFailed(#[from] BrowserDetectionFailed),
}

/// Error returned when browser detection fails.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct BrowserDetectionFailed(String);

/// A struct representing a browser instance for Reddit, utilizing the `chromiumoxide`
/// library to manage the browser lifecycle and interactions.
#[derive(Debug)]

pub struct RedditBrowser(chromiumoxide::Browser);

/// A struct representing a page instance in a Reddit browser, allowing for
/// interactions with the page content and navigation, leveraging the
/// `chromiumoxide` library.
#[derive(Debug)]
pub struct RedditPage(chromiumoxide::Page);

impl RedditBrowser {
    /// Launches a Reddit browser and returns a tuple containing the browser and a handler.
    ///
    /// This function is responsible for setting up the browser environment using the provided
    /// user information and screen dimensions. It calculates the appropriate device scale factor
    /// for rendering, builds the browser configuration, and then launches the browser asynchronously.
    ///
    /// # Arguments
    ///
    /// * `user` - An optional reference to a `RedditUser` object.
    /// * `dimensions` - A `Dimensions` struct used to set the width and height for the browser viewport.
    ///
    /// # Returns
    ///
    /// Returns a `RedditResult` containing a tuple of the `RedditBrowser` and its `Handler` for
    /// managing browser events.
    ///
    /// # Errors
    ///
    /// Returns a `BrowserDetectionFailed` error if the browser configuration could not be built.
    ///
    /// ** NOTE **
    /// Example of how to spawn an asynchronous task for handling browser events.
    ///
    /// Spawns a new asynchronous task using `async_std::task::spawn` to continuously
    /// await the next browser event from the handler in a loop.
    ///
    /// After spawning, the handle is awaited to allow the event handling task to complete.
    ///
    /// ```rust, no-run
    /// let handler = RedditBrowser::from_user(None,..).await?;
    /// let handle = async_std::task::spawn(async move {
    ///     loop {
    ///         let _event = handler.next().await.unwrap();
    ///     }
    /// });
    /// handle.await;
    /// ```
    ///
    /// More details on Chromium browser automation can be found in the
    /// [chromiumoxide crate documentation](https://docs.rs/chromiumoxide/latest/chromiumoxide/index.html)
    pub async fn from_user(
        user: Option<&RedditUser>,
        dimensions: Dimensions,
    ) -> RedditResult<(Self, Handler)> {
        let (width, height) = dimensions.width_height();

        // Device scale factor (or dsf for short) allows us to increase the resolution of the screenshots
        // When the dsf is 1, the width of the screenshot is 600 pixels
        // so we need a dsf such that the width of the screenshot is greater than the final resolution of the video
        let device_scale_factor = (width / 600) + 1;
        let viewport = Viewport {
            width,
            height,
            device_scale_factor: Some(device_scale_factor as f64),
            ..Default::default()
        };

        let browser_config = BrowserConfig::builder()
            .viewport(viewport)
            .with_head() // Just for debuggin purposes
            .build()
            .map_err(|message| BrowserDetectionFailed(message))?;

        let (browser, handler) = Browser::launch(browser_config).await?;

        let browser = RedditBrowser::new(user, browser).await?;

        Ok((browser, handler))
    }

    /// Creates a new instance of `RedditBrowser`.
    ///
    /// # Arguments
    ///
    /// * `browser` - A `chromiumoxide::Browser` instance that represents the browser.
    /// * `user` - An optional reference to a `RedditUser`. If provided, the user will
    ///   be logged in and their theme will be set in the newly created browser instance.
    ///
    /// # Returns
    ///
    /// Returns a `RedditResult<Self>` which is a `Result` that contains the new
    /// `RedditBrowser` instance on success or a `RedditError` on failure.
    pub async fn new(
        user: Option<&RedditUser>,
        browser: chromiumoxide::Browser,
    ) -> RedditResult<Self> {
        if let Some(user) = user {
            user.login_and_set_theme(&browser).await?;
        }

        Ok(Self(browser))
    }
}

impl RedditPage {
    /// Creates a new `RedditPage` for a specific submission.
    ///
    /// # Arguments
    ///
    /// * `browser` - A reference to the `RedditBrowser` instance used to create the page.
    /// * `submission` - A reference to `SubmissionData` containing the subreddit name and submission ID.
    ///
    /// # Returns
    ///
    /// Returns a `chromiumoxide::Result<Self>` which is a `Result` that contains the new
    /// `RedditPage` instance on success or an error from the `chromiumoxide` library on failure.
    pub async fn new(
        browser: &RedditBrowser,
        submission: &roux::submission::SubmissionData,
    ) -> chromiumoxide::Result<Self> {
        let url = format!(
            "https://www.reddit.com/r/{name}/comments/{id}",
            name = submission.subreddit,
            id = submission.id
        );
        let page = browser.0.new_page(url).await?;

        const QUERY: &str = include_str!("../../js/hide_popups.js");
        page.evaluate_function(QUERY).await.unwrap();

        Ok(Self(page))
    }
}

impl Deref for RedditPage {
    type Target = chromiumoxide::Page;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for BrowserDetectionFailed {
    fn from(value: String) -> Self {
        debug_assert_eq!(value.as_str(), "Could not auto detect a chrome executable");
        Self(value)
    }
}
