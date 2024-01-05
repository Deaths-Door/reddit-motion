use std::path::{PathBuf, Path};

use chromiumoxide::{Page, Browser};
use roux::submission::SubmissionData;

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

pub(super) async fn create_new_page(
    browser : &Browser,
    submission : &SubmissionData
) -> chromiumoxide::Result<Page> {
    let url = format!("https://www.reddit.com/r/{name}/comments/{id}",name = submission.name,id = submission.id);
    let page = browser.new_page(url).await?;

    // TODO : CLOSE ALL POPUPS + NSFW + ANOYMUS BROWSING + COOKIES ACCEPT
    Ok(page)
}