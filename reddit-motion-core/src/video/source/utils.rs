use std::path::PathBuf;

use chromiumoxide::{cdp::browser_protocol::page::CaptureScreenshotFormat, Element};

use crate::VideoSourceFiles;

use super::{SourceAggregator, SubredditTaskGroup};

impl SourceAggregator<'_> {
    /// Asynchronously captures a screenshot of a specific HTML element in a Chromium-based browser.
    ///
    /// # Arguments
    ///
    /// * `file_name` - A string slice that specifies the name of the output PNG file without the extension.
    /// * `get_element` - A closure that accepts a CSS selector and returns a future resolving to the HTML element (`Element`).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `String` with the name of the saved PNG file if successful, or a `chromiumoxide::Error` if an error occurs during the operation.
    pub(super) async fn screen_shot_post_element(
        &self,
        file_name: &str,
        element: Element,
    ) -> chromiumoxide::Result<PathBuf> {
        const SELECTOR: &str = "div[data-test-id=\"post-content\"]";

        let pathbuf = self.resolve_to_base(file_name, "png");

        element
            .scroll_into_view()
            .await?
            .save_screenshot(CaptureScreenshotFormat::Png, &pathbuf)
            .await?;

        Ok(pathbuf)
    }
}

impl SourceAggregator<'_> {
    pub(super) fn resolve_from_base(&self, file_name: &str, extension: &str) -> PathBuf {
        let mut pathbuf = self.source_files.base_directory.clone();
        pathbuf.push(file_name);
        pathbuf.set_extension(extension);
        pathbuf
    }
}
