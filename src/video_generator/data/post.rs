use chromiumoxide::{Page, Element};
use roux::submission::SubmissionData;

use crate::{config::{VideoCreationError, VideoCreationArguments}, video_generator::VideoGenerationFiles};

impl VideoGenerationFiles {
    pub(super) async fn exceute_post_no_translation(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {
        self.exceute_post_content(submission, page, args, |s|s, |element,_| async {
            Ok(element)
        }).await
    }


    // post.content.element = #t3_4ifj4i > div > div[data-click-id=\"text\"]"
    // post.content.text = post.content.element > div > (has p elements which need to be translated)
    async fn exceute_post_content<F>(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        map_text : impl FnOnce(&str) -> &str,
        map_element : impl FnOnce(Element,&str) -> F 
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        self.exceute_on_post(
            submission,
            page,
            args,
            "post",
            &submission.selftext,
            map_text,
            |element,text| async move {
                if submission.over_18 {
                    // Content hidden by button
                    show_nsfw_body(&element).await?;
                }

                let element = element.find_element("div > div[data-click-id=\"text\"]").await?;
                map_element(element,text).await
            }
        ).await
    }
}


async fn show_nsfw_body(element : &Element) -> chromiumoxide::Result<()> {
    element.find_element(
        "div > div._3xX726aBn29LDbsDtzr_6E._1Ap4F5maDtT1E1YuCiaO0r.D3IL3FD0RFy_mkKLPwL4 > div > div > button"
    )
    .await?
    .click()
    .await
    .map(|_| ())
}