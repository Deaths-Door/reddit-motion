use std::path::{PathBuf, Path};

use chromiumoxide::{Element, Page, cdp::browser_protocol::page::CaptureScreenshotFormat};
use unic_langid::LanguageIdentifier;
use crate::config::{VideoCreationArguments, VideoCreationError};

use super::VideoGenerationFiles;

impl VideoGenerationFiles {
    pub(super) fn push_files(&mut self,audio : PathBuf,png : PathBuf) {
        self.files.push((audio.display().to_string(),png.display().to_string()))
    }

    pub(super) fn audio_file(&self,name : &str) -> PathBuf {
        let mut pathbuf = self.storage_directory.clone();
        pathbuf.push(format!("{}.mp3",name));
        pathbuf
    }
    
    pub(super) fn png_file(&self,name : &str) -> PathBuf {
        let mut pathbuf = self.storage_directory.clone();
        pathbuf.push(format!("{}.png",name));
        pathbuf
    }
}

// if some process ahead fails it keeps on to the file
macro_rules! if_path_exists {
    ($path : expr,return ok) => {
        if std::path::Path::new($path).exists() {
           return Ok(())
        }
    };
    (not $path : expr,$code : expr) => {
        if !std::path::Path::new($path).exists() {
           $code
        }
    };
}

pub(crate) use if_path_exists;

pub(super) async fn post_element_and_screenshot<F>(
    page: &Page,
    file_name : &Path,
    map_element : impl FnOnce(Element) -> F,
) -> chromiumoxide::Result<()> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
    const SELECTOR : &str= "div[data-test-id=\"post-content\"]";
    element_and_screenshot(SELECTOR.to_string(), page, file_name, map_element).await
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
    pub(super) async fn exceute_on_post<F>(
        &mut self,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        name : &str,
        text : &str,
        map_element : impl FnOnce(Element) -> F
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        self.exceute_on_thread(
            args, name, text,
            |png_path| async move {
                super::post_element_and_screenshot(page, &png_path,|e| map_element(e)).await
        }).await
    }

    /// Lowest API for [VideoGenerationFiles]
    pub(super) async fn exceute_on_thread<F>(
        &mut self,
        args : &VideoCreationArguments<'_>,
        name : &str,
        text : &str,
        screenshot : impl FnOnce(PathBuf) -> F
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<()>> {
        
        let audio_path = self.audio_file(name);
        if_path_exists!(not &audio_path,args.config.tts.save_speech_to_file(&audio_path,&text).await?);       

        let png_path = self.png_file(name);
        if_path_exists!(
            not &png_path,
            screenshot(png_path.clone()).await?
        ); 

        self.push_files(audio_path,png_path);

        Ok(())
    }
}


pub async fn set_attribute(element : &Element,value : &str) -> chromiumoxide::Result<()> {
    let js_fn = format!("function() {{ this.innerText = {value}; }}");
    let return_value = element.call_js_fn(js_fn, true).await?;

    // Check is successfully called
    assert!(return_value.exception_details.is_none());
    Ok(())
}

pub async fn update_p_with_translated_text(content : &str,elements : &[Element]) -> chromiumoxide::Result<()> {
    let mut split_content = content.split("\n");
            
    for p in elements {
        // Since translator will preserve formatting it should work
        let value = split_content.next().unwrap();
        set_attribute(p,value).await?;
    }

    Ok(())
}

pub fn unic_langid_to_deepl_lang(value : LanguageIdentifier) -> deepl::Lang {
    // TODO : When more translators are added extend this
    deepl::Lang::try_from(value.language.as_str()).unwrap()
}
