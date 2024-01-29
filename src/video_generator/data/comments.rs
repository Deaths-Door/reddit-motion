use std::path::Path;

use chromiumoxide::{Page, Element};
use deepl::Lang;
use roux::{submission::SubmissionData, Subreddit, comment::CommentData, response::BasicThing};

use crate::config::{Translator, VideoCreationArguments, VideoCreationError};

use super::{VideoGenerationFiles, utils::element_and_screenshot};

impl VideoGenerationFiles {
    /*pub(super) async fn exceute_comments_no_translation(
        &mut self,
        max_comments : u32,
        subreddit : &Subreddit,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {
        self.exceute_comments(
            max_comments, 
            subreddit, 
            submission,
            page, 
            args, 
            |text| async move { Ok(text) },
            |e,_| async move { Ok(e) }).await
    }

    pub(super) async fn exceute_comments_with_translation(
        &mut self,
        max_comments : u32,
        subreddit : &Subreddit,
        submission : &SubmissionData,
        target_lang : Lang,
        translater_client : &Translator,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {    
        self.exceute_comments(
            max_comments, 
            subreddit, 
            submission,
            page, 
            args, 
            |text| async move {
                let text = text.as_str();
                translater_client.translate(text, target_lang).await
            },
            |element,text| async move { 
                let multiple_p = element.find_elements("div > div > div > p").await?;
                super::utils::update_p_with_translated_text(text,&multiple_p).await?;
                Ok(element) 
            }
        ).await
    }*/

    const MAX_RETRIES : u8 = 3;

    pub(super) async fn exceute_comments/*<'outer,Fe,Ft>*/(
        &mut self,
        max_comments : u32,
        subreddit : &Subreddit,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        translator : Option<(&Translator,Lang)>
       // map_text : impl FnOnce(String) -> Ft + Clone,
       // map_element : impl FnOnce(Element,&'outer str) -> Fe + Copy
    ) -> Result<(),VideoCreationError> 
       // where Fe: std::future::Future<Output = chromiumoxide::Result<Element>> + 'outer,
         //   Ft: std::future::Future<Output = TranslatorResult>  
    {
        let mut retries = 0;
        let mut skipped = 0;

        loop {
            let mut comments = comments(subreddit,submission,max_comments + skipped)
                .await?;

            // Only remove if not else underflow error
            if skipped != 0 {
                comments.drain(..(skipped as usize - 1));
                // Reset to 0
                skipped = 0
            }

            for comment in comments {
                if let Err(_) = self.exceute_comment_impl(comment.data, page, args,&translator /*, map_text.clone(), map_element*/).await {
                    skipped += 1;
                }
            }

            // Means all passed 
            if skipped == 0 {
                break Ok(()) 
            }

            retries += 1;
            if retries > Self::MAX_RETRIES {
                // Just leave it with the number of comments processed insteaed of err
                break Ok(())
            }
        }
    }
}

impl VideoGenerationFiles {
    /// Return `Result<(),()>` that if none then return error 
    /// Done this as I can't figure out how to get the compiler to convinve that object does in fact life long enough
    async fn exceute_comment_impl/*<'outer,Fe,Ft>*/(
        &mut self,
        comment : CommentData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        translator : &Option<(&Translator,Lang)>
     //   map_text : impl FnOnce(String) -> Ft,
     //   map_element : impl FnOnce(Element,&'outer str) -> Fe,
    ) -> Result<(),()> 
     //   where Fe: std::future::Future<Output = chromiumoxide::Result<Element>> + 'outer,
    //        Ft: std::future::Future<Output = TranslatorResult> 
    {
        // For some fucking reason the comment body can be none by the API when its clearly there
        // eg https://reddit.com/r/AskReddit/comments/1903bgc/what_are_some_unsaid_first_date_rules_everyone
        // has comment with id of kgljxfg return None for body
        if comment.body.is_none()  {
            return Err(())
        }

        // Basically the name is t1_ + the 'id' for the comment
        let comment_id : &str = &comment.name.unwrap();
        let comment_body = comment.body.unwrap();
        let text = comment_body;
        // comment_body vs translate comment_body
        match translator {
            None => self.exceute_on_thread(
                args, 
                comment_id, 
                &text, 
                |file_name| async move {
                    comment_element_and_screenshot(
                        page, 
                        comment_id.to_owned(), 
                        &file_name, 
                        |e| async move { Ok(e) }
                    ).await
            })
            .await,
            Some((translater_client,target_lang)) => {
                let text = &translater_client.translate(&text, target_lang.clone()).await.map_err(|_| ())?;
                self.exceute_on_thread(
                    args, 
                    comment_id, 
                    text, 
                    |file_name| async move {
                        comment_element_and_screenshot(
                            page, 
                            comment_id.to_owned(), 
                            &file_name, 
                            |element| async move { 
                                let multiple_p = element.find_elements("div > div > div > p").await?;
                                super::utils::update_p_with_translated_text(text,&multiple_p).await?;
                                Ok(element) 
                            }
                        ).await
                })
                .await
            }
        }.map_err(|_| ())
      //  let text = map_text(comment_body).await.map_err(|_| ())?;
       // let text = text.as_str();

        /*let text = match target_lang {
            None => comment_body,
            Some(target_lang) => translater_client.translate(text, comment_body).await
        };
        self.exceute_on_thread(
            args, 
            comment_id, 
            text, 
            |file_name| async move {
                comment_element_and_screenshot(
                    page, 
                    comment_id.to_owned(), 
                    &file_name, 
                    |e| async move 
                ).await
                // Done lazly thats why here clone
               // comment_element_and_screenshot(page, comment_id.to_owned(), &file_name, |e| map_element(e,text)).await
        })
        .await
        .map_err(|_| ())*/
    }
}

async fn comments(subreddit : &Subreddit,submission : &SubmissionData,max_comments : u32) -> Result<Vec<BasicThing<CommentData>>,VideoCreationError> {
    Ok(subreddit.article_comments(&submission.id, Some(1), Some(max_comments))
        .await?
        .data
        .children)
}

async fn comment_element_and_screenshot<F>(
    page: &Page,
    comment_id : String,
    file_name : &Path,
    map_element : impl FnOnce(Element) -> F,
) -> chromiumoxide::Result<()> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
    // TODO : CHECK WHY ISNT THIS WORKING , like it takes a screenshot but shws upempty
    let selector = format!("#{comment_id}");
    let _ = element_and_screenshot(selector, page, file_name, map_element).await.unwrap();
    Ok(())
}
