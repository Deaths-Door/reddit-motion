use std::path::{PathBuf, Path};

use chromiumoxide::{Element, Page, cdp::browser_protocol::page::CaptureScreenshotFormat};
use roux::submission::SubmissionData;

use crate::config::{VideoCreationArguments, VideoCreationError};

use super::VideoGenerationArguments;

impl VideoGenerationArguments {
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

macro_rules! exceute {
    (
        $story_mode:expr,
        $title : expr,
        $post :expr,
        $comments : expr,
        $auto : expr
    ) => {{
        $title;

        match $story_mode {
            StoryMode::ReadComments => $comments,
            StoryMode::ReadPost =>$post,
            _ => $auto
        }
    }};
}

// if some process ahead fails it keeps on to the file
macro_rules! if_path_exists {
    (not $path : expr,$code : expr) => {
        if !std::path::Path::new($path).exists() {
           $code
        }
    };
}

pub(super) use {exceute,if_path_exists};


pub(super) async fn post_element_and_screenshot<F>(
    page: &Page,
    submission : &SubmissionData,
    file_name : &Path,
    map_element : impl FnOnce(Element) -> F,
) -> chromiumoxide::Result<()> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
    let class = format!("#t3_{}",submission.id);
    let title_element = page.find_element(class).await?;

    let _ = map_element(title_element)
        .await?
        .save_screenshot(CaptureScreenshotFormat::Png, file_name)
        .await?;

    Ok(())
}

impl VideoGenerationArguments {
    pub(super) async fn __exceute_on_post<'a,F>(
        &mut self,
        _submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        name : &str,
        core_text : &'a str,
        map_text : impl Fn(&'a str) -> &str,
        map_element : impl FnOnce(Element,&'a str) -> F // translate the &str and update the element with it for translate version
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        let text = map_text(core_text);

        let audio_path = self.audio_file(name);
        super::if_path_exists!(not &audio_path,args.config.tts.save_speech_to_file(&audio_path,text).await?);       

        let png_path = self.png_file(name);

        super::if_path_exists!(not &png_path,
            super::post_element_and_screenshot(page, _submission, &png_path,|e| map_element(e,text)
        ).await?);       

        self.push_files(audio_path,png_path);

        Ok(())
    }
}