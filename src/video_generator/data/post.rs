use chromiumoxide::{Page, Element};
use deepl::Lang;
use roux::submission::SubmissionData;

use crate::{config::{Translator, VideoCreationArguments, VideoCreationError}, video_generator::VideoGenerationFiles};

impl VideoGenerationFiles {
    pub(super) async fn exceute_post_no_translation(
        &mut self,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {
        let content = post_content(submission);

        self.exceute_post_content(submission, page, args, content, |element| async {
            Ok(element)
        }).await
    }

    pub(super) async fn exceute_post_with_translation(
        &mut self,
        submission : &SubmissionData,
        target_lang : Lang,
        translater_client : &Translator,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {    
        let content = translater_client.translate(post_content(submission), target_lang).await?;
        
        let content : &str = &content;
        
        self.exceute_post_content(submission, page, args, content, |element| async move {
            let multiple_p = element.find_elements("div > p").await?;
            super::utils::update_p_with_translated_text(&content,&multiple_p).await?;
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
        content : &str,
        map_element : impl FnOnce(Element) -> F 
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        self.exceute_on_post(
            submission,
            page,
            args,
            "post",
            content,
            |element| async move {
                if submission.over_18 {
                    // Content hidden by button
                    show_nsfw_body(&element).await?;
                }

                let element = element.find_element("div > div[data-click-id=\"text\"]").await?;
                map_element(element).await
            }
        ).await
    }
}

fn post_content(submission : &SubmissionData) -> &str {
    &submission.selftext
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