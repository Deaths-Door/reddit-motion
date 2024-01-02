use futures::StreamExt;
use chromiumoxide::{Browser, BrowserConfig, cdp::{browser_protocol::page::CaptureScreenshotFormat, Event},page::ScreenshotParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().window_size(3000, 800).with_head().build()?).await?;

    let handle = tokio::task::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });

    let page = browser.new_page("https://www.reddit.com/r/AskReddit").await?;

    page.evaluate("document.body.style.zoom=400").await?;

    page.reload().await?;

    let screenshort_parms = ScreenshotParams::builder()
        .full_page(true)
        .format(CaptureScreenshotFormat::Png)
        .build();

    page.save_screenshot(screenshort_parms, "zooming.png").await?;

    println!("Finished");

    handle.await?;
    Ok(())
}