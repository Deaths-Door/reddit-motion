use std::path::Path;

use chromiumoxide::{Page, cdp::browser_protocol::page::CaptureScreenshotFormat};
use roux::submission::SubmissionData;

use crate::config::{VideoCreationArguments, VideoCreationError};

use super::VideoGenerationArguments;



impl VideoGenerationArguments {
    pub(super) async fn exceute_title(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {
        let audio_path = self.audio_file(&submission.title);
        args.config.tts.save_speech_to_file(&audio_path,&submission.title).await?;        

        let png_path = self.png_file(&submission.title);
        screenshot_post_title(page, submission, &png_path).await?;

        self.push_files(audio_path,png_path);

        Ok(())
    }
}

// TODO : Translate this
async fn screenshot_post_title(
    page: &Page,
    submission: &SubmissionData,
    file_name : &Path,
) -> chromiumoxide::Result<()> {
    let class = format!("#t3_{}",submission.id);
    let title_element = page.find_element(class).await?;

    let _ = title_element
        .save_screenshot(CaptureScreenshotFormat::Png, file_name)
        .await?;

    Ok(())
}