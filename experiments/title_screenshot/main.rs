use std::{time::Duration, fmt::format};

use futures::StreamExt;
use chromiumoxide::{Browser, BrowserConfig, cdp::browser_protocol::{page::CaptureScreenshotFormat, dom::{QuerySelectorAllParams, NodeId}}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().window_size(3000, 800).with_head().build()?).await?;

    let handle = tokio::task::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    // Basically , the id of the reddit post is the element to search and take a screenshort of 
    let id = "18s7bmn";

    
    let class = format!("#t3_{id}");
    let url = format!("https://www.reddit.com/r/AskReddit/comments/{id}");
    
    let page = browser.new_page(url).await?;

    let title_element = page.find_element(class).await?;

    println!("{:?}",title_element);
    
    title_element
        .save_screenshot(CaptureScreenshotFormat::Png, "screenshot.png")
        .await?;

    println!("Finished");

    handle.await?;
    Ok(())
}