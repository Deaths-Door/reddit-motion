use std::ops::{Deref, DerefMut};

use diesel::SqliteConnection;
use reddit_motion_core::{
    chromiumoxide::{
        error::CdpError, handler::viewport::Viewport, Browser, BrowserConfig, Handler,
    },
    BrowserDetectionFailed, Dimensions, RedditError, RedditResult,
};

use crate::{
    DatabaseConnection, DatabaseError, RedditTaskError, RedditTaskInducer,
    RetrievingCachedEntriesError,
};

impl RedditTaskInducer {
    pub(super) async fn create_browser(
        dimensions: &Dimensions,
    ) -> RedditResult<(Browser, Handler)> {
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
            .map_err(|e| BrowserDetectionFailed::from(e))?;

        Ok(Browser::launch(browser_config).await?)
    }
}

impl From<CdpError> for RedditTaskError {
    fn from(value: CdpError) -> Self {
        Self::Reddit(RedditError::Chromiumoxide(value))
    }
}

impl Deref for DatabaseConnection {
    type Target = SqliteConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DatabaseConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<RetrievingCachedEntriesError> for RedditTaskError {
    fn from(value: RetrievingCachedEntriesError) -> Self {
        Self::DatabaseError(DatabaseError::RetrievingCachedEntries(value))
    }
}
