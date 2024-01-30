use chromiumoxide::{Browser, Page};
use roux::{Subreddit, util::FeedOption, submission::SubmissionData};
use unic_langid::LanguageIdentifier;
use whatlang::{Detector, Lang};

use crate::{config::VideoCreationError, db::Database};

pub(super) async fn wait_for(secs : u64) {
    tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
}

pub(super) async fn retry_till_new_submission<'a>(
    db : &Database,
    count : &mut u32,
    extra_langs: &'a [LanguageIdentifier],
    subreddit: &Subreddit,
) -> Result<(SubmissionData,Vec<&'a LanguageIdentifier>),VideoCreationError> {
    // Basically without this infinite loop 😅
    if extra_langs.is_empty() {
        let submission = submission(&subreddit,*count).await?;
        *count += 1;
        return Ok((submission,vec![]));
    }

    loop {
        let submission = submission(&subreddit,*count).await?;

        *count += 1;

        let langs = db.unprocessed_threads(&submission.id,extra_langs);

        if !langs.is_empty() {
            return Ok((submission,langs));
        }
    }
}

pub(super) async fn submission(subreddit : &Subreddit,count : u32)  -> Result<SubmissionData,VideoCreationError> {
    let options = FeedOption::new()
        .limit(1)
        .count(count);
    
    let v = subreddit.top(1, Some(options))
        .await?
        .data
        .children
        .pop()
        .unwrap()
        .data;

    Ok(v)
}

pub(super) fn detect_post_language(detector : &Detector,submission : &SubmissionData) -> LanguageIdentifier {
    let detected_lang = detector
        .detect_lang(&submission.title)
        .unwrap_or(Lang::Eng)
        .code();

    detected_lang.parse().unwrap()
}


pub(super) async fn create_new_page(
    browser : &Browser,
    submission : &SubmissionData
) -> chromiumoxide::Result<Page> {
    let url = format!("https://www.reddit.com/r/{name}/comments/{id}",name = submission.subreddit,id = submission.id);
    let page = browser.new_page(url).await?;

    // Refer to ./experiments/hide_popups.js and ./experiments/as_single_string.kt
    const FUNCTION : &str = "function() {document.querySelector(\"shreddit-async-loader[bundlename=\"reddit_cookie_banner\"]\")?.remove();document.querySelector(\"shreddit-async-loader[bundlename=\"desktop_rpl_nsfw_blocking_modal\"]\")?.remove();document.querySelector(\"shreddit-app > div\")?.style.filter=\"none\";}";
    page.evaluate_function(FUNCTION).await?;
    Ok(page)
}