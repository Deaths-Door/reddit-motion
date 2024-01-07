use std::path::Path;

use chromiumoxide::{Page, Element};
use roux::{submission::SubmissionData, Subreddit, comment::CommentData, response::BasicThing};

use crate::config::{VideoCreationArguments, VideoCreationError};

use super::{VideoGenerationArguments, utils::element_and_screenshot};

impl VideoGenerationArguments {
    pub(super) async fn exceute_comments_no_translation(
        &mut self,
        max_comments : u32,
        subreddit : &Subreddit,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
    ) -> Result<(),VideoCreationError> {
        self.__exceute_comments(max_comments, subreddit, submission, page, args, |text| text,|e,_| async move {
            Ok(e)
        }).await
    }

    async fn __exceute_comments<F>(
        &mut self,
        max_comments : u32,
        subreddit : &Subreddit,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        map_text : impl FnOnce(&str) -> &str + Copy,
        map_element : impl FnOnce(Element,&str) -> F + Copy
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        const MAX_RETRIES : u32 = 3;

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
                if let Err(_) = self.exceute_comment(comment.data, submission, page, args, map_text, map_element).await {
                    skipped += 1;
                }
            }

            // Means all passed 
            if skipped == 0 {
                break Ok(()) 
            }

            retries += 1;
            if retries > MAX_RETRIES {
                // Just leave it with the number of comments processed insteaed of err
                break Ok(())
            }
        }
    }
}

impl VideoGenerationArguments {
    async fn exceute_comment<F>(
        &mut self,
        comment : CommentData,
        submission : &SubmissionData,
        page : &Page,
        args : &VideoCreationArguments<'_>,
        map_text : impl FnOnce(&str) -> &str,
        map_element : impl FnOnce(Element,&str) -> F,
    ) -> Result<(),VideoCreationError> where F: std::future::Future<Output = chromiumoxide::Result<Element>> {
        // For some fucking reason the comment body can be none by the API when its clearly there
        // eg https://reddit.com/r/AskReddit/comments/1903bgc/what_are_some_unsaid_first_date_rules_everyone
        // has comment with id of kgljxfg return None for body
        if comment.body.is_none()  {
            println!("comment_body={:?}",comment);
            return Ok(())
        }

        // Basically the name is t1_ + the 'id' for the comment
        let comment_id : &str = &comment.name.unwrap();

        // No reason to see why not
        let comment_body = comment.body.unwrap();

        self.exceute_on_thread(
            submission, 
            args, 
            comment_id, 
            &comment_body,
            map_text,
            |file_name,text| async move {
                // Done lazly thats why here clone
                comment_element_and_screenshot(page, comment_id.to_owned(), &file_name, |e|map_element(e,text)).await
            }
        ).await
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
