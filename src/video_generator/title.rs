use std::path::Path;

use chromiumoxide::{Page, cdp::browser_protocol::page::CaptureScreenshotFormat, Element};
use roux::submission::SubmissionData;

use crate::config::{VideoCreationArguments, VideoCreationError};

use super::VideoGenerationArguments;



impl VideoGenerationArguments {
    pub(super) async fn exceute_title_no_translation(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {
        self.__exceute_title(submission, page, args, |s|s, |element,_| async {
            Ok(element)
        }).await
    }
    
    async fn __exceute_title<F>(
        &mut self,
        _submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        map_text : impl Fn(&str) -> &str,
        map_element : impl FnOnce(Element,&str) -> F // translate the &str and update the element with it for translate version
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        let title = map_text(&_submission.title);

        let audio_path = self.audio_file("title");
        super::if_path_exists!(not &audio_path,args.config.tts.save_speech_to_file(&audio_path,title).await?);       

        let png_path = self.png_file("title");
        super::if_path_exists!(not &png_path,screenshot_post_title(page, _submission, &png_path,|e| map_element(e,title)).await?);       

        self.push_files(audio_path,png_path);

        Ok(())
    }
}

// TODO : Translate this
 
// text seclector for it document.querySelector("#t3_18rbiqq > div > div > div > div > h1").innerText = ...
async fn screenshot_post_title<F>(
    page: &Page,
    submission: &SubmissionData,
    file_name : &Path,
    map_element : impl FnOnce(Element) -> F
) -> chromiumoxide::Result<()> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
    let class = format!("#t3_{}",submission.id);
    let title_element = page.find_element(class).await?;

    let _ = map_element(title_element)
        .await?
        .save_screenshot(CaptureScreenshotFormat::Png, file_name)
        .await?;

    Ok(())
}