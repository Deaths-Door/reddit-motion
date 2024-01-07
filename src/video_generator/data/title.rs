use chromiumoxide::{Page, Element};
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

    // TODO TRANSLATE THIS
    // Title.text.selector = document.querySelector("#t3_18rbiqq > div > div > div > div > h1").innerText
    // Title.thingy = #t3_18zski5 > div > div._2FCtq-QzlfuN-SwVMUZMM3._2v9pwVh0VUYrmhoMv1tHPm
    async fn __exceute_title<F>(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        map_text : impl FnOnce(&str) -> &str,
        map_element : impl FnOnce(Element,&str) -> F // translate the &str and update the element with it for translate version
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        self.exceute_on_post(
            submission,
            page,
            args,
            "title",
            &submission.title,
            map_text,
            |element,text| async move {
                let element = element.find_element("div > div._2FCtq-QzlfuN-SwVMUZMM3._2v9pwVh0VUYrmhoMv1tHPm").await?;
                map_element(element,text).await
            }
        ).await
    }
}