use chromiumoxide::{Page, cdp::browser_protocol::page::CaptureScreenshotFormat};
use roux::submission::SubmissionData;

pub async fn screenshot_post_title(
    page: &Page,
    submission: &SubmissionData,
    dir : &str,
    file_name : &str
) -> chromiumoxide::Result<String>{
    let class = format!("#t3_{}",submission.id);
    let title_element = page.find_element(class).await?;

    let screenshot_dir = format!("{dir}/{file_name}");
    println!("{screenshot_dir}");
    let _ = title_element
        .save_screenshot(CaptureScreenshotFormat::Png, &screenshot_dir)
        .await?;

    Ok(screenshot_dir)
}