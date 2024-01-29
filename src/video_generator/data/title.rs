use chromiumoxide::{Page, Element};
use deepl::Lang;
use roux::submission::SubmissionData;

use crate::config::{Translator, VideoCreationArguments, VideoCreationError};

use super::{utils, VideoGenerationFiles};



impl VideoGenerationFiles {
    pub(super) async fn exceute_title_no_translation(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {
        let title = title(submission);

        self.exceute_title(submission, page, args, title, |element| async {
            Ok(element)
        }).await
    }

    pub(super) async fn exceute_title_with_translation(
        &mut self,
        submission : &SubmissionData,
        target_lang : Lang,
        translater_client : &Translator,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {    
        let title = translater_client.translate(title(submission), target_lang).await?;
        
        let title : &str = &title;
        
        self.exceute_title(submission, page, args, title, |element| async move {
            let element = element.find_element("div > div > h1").await?;
            utils::set_attribute(&element,title).await?;
            Ok(element)
        }).await
    }

    // TODO SELECT DIFferent part OF IT
    // Title.text.selector = document.querySelector("#t3_18rbiqq > div > div._2FCtq-QzlfuN-SwVMUZMM3._2v9pwVh0VUYrmhoMv1tHPm > div > div > h1").innerText
    // Title.thingy = #t3_18zski5 > div > div._2FCtq-QzlfuN-SwVMUZMM3._2v9pwVh0VUYrmhoMv1tHPm
    async fn exceute_title<F>(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        title : &str,
        map_element : impl FnOnce(Element) -> F 
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        self.exceute_on_post(
            submission,
            page,
            args,
            "title",
            title,
            |element| async move {
                let element = element.find_element("div > div._2FCtq-QzlfuN-SwVMUZMM3._2v9pwVh0VUYrmhoMv1tHPm").await?;
                map_element(element).await
            }
        ).await
    }
}

fn title(submission : &SubmissionData) -> &str {
    &submission.title
}