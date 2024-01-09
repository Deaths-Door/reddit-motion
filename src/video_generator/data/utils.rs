use std::path::{PathBuf, Path};

use chromiumoxide::{Element, Page, cdp::browser_protocol::page::CaptureScreenshotFormat};
use roux::submission::SubmissionData;

use crate::config::{VideoCreationArguments, VideoCreationError};

use super::VideoGenerationFiles;

impl VideoGenerationFiles {
    pub(super) fn push_files(&mut self,audio : PathBuf,png : PathBuf) {
        self.files.push((audio.display().to_string(),png.display().to_string()))
    }

    pub(super) fn audio_file(&self,name : &str) -> PathBuf {
        let mut pathbuf = self.storage_directory.clone();
        pathbuf.set_file_name(format!("{}.mp3",name));
        pathbuf
    }
    
    pub(super) fn png_file(&self,name : &str) -> PathBuf {
        let mut pathbuf = self.storage_directory.clone();
        pathbuf.set_file_name(format!("{}.png",name));
        pathbuf
    }
}

// if some process ahead fails it keeps on to the file
macro_rules! if_path_exists {
    (not $path : expr,$code : expr) => {
        if !std::path::Path::new($path).exists() {
           $code
        }
    };
}

pub(crate) use if_path_exists;

pub(super) async fn post_element_and_screenshot<F>(
    page: &Page,
    submission : &SubmissionData,
    file_name : &Path,
    map_element : impl FnOnce(Element) -> F,
) -> chromiumoxide::Result<()> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
    let selector = format!("#t3_{}",submission.id);
    element_and_screenshot(selector, page, file_name, map_element).await
}

pub(super) async fn element_and_screenshot<F>(
    selector : String,
    page: &Page,
    file_name : &Path,
    map_element : impl FnOnce(Element) -> F,
) -> chromiumoxide::Result<()> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
    let title_element = page.find_element(selector).await?;

    let _ = map_element(title_element)
        .await?
        .save_screenshot(CaptureScreenshotFormat::Png, file_name)
        .await?;

    Ok(())
}

impl VideoGenerationFiles {
    pub(super) async fn exceute_on_post<'a,F>(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        name : &str,
        core_text : &'a str,
        map_text : impl FnOnce(&'a str) -> &str,
        map_element : impl FnOnce(Element,&'a str) -> F // translate the &str and update the element with it for translate version
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        self.exceute_on_thread(submission, args, name, core_text, map_text, |png_path,text| async move {
            super::post_element_and_screenshot(page, submission, &png_path,|e| map_element(e,text)).await
        }).await
    }

    /// Lowest API for VideoGenerationArguments
    pub(super) async fn exceute_on_thread<'a,F>(
        &mut self,
        _submission : &SubmissionData,
        args : &VideoCreationArguments<'_>,
        name : &str,
        core_text : &'a str,
        map_text : impl FnOnce(&'a str) -> &str,
        // Text is already translated
        screenshot : impl FnOnce(PathBuf,&'a str) -> F
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<()>> {
        let text = map_text(core_text);

        let audio_path = self.audio_file(name);
        if_path_exists!(not &audio_path,args.config.tts.save_speech_to_file(&audio_path,text).await?);       

        let png_path = self.png_file(name);
        // TODO : Find a way to pass &Path instead of cloning
        if_path_exists!(not &png_path,screenshot(png_path.clone(),text).await?);       

        self.push_files(audio_path,png_path);

        Ok(())
    }
}