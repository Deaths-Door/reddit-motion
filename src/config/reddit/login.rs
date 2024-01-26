use chromiumoxide::{Browser,error::CdpError, Page, Element, cdp::browser_protocol::page::NavigateParams};
use super::{RedditUser, wait_for};

impl RedditUser {
    /// Returns if login was successful
    pub async fn login_and_set_theme(&self,browser : &Browser) -> chromiumoxide::Result<bool> {
        let page = browser.new_page("https://www.reddit.com/login").await?;
 
        // If visible it means we are already logged in , so return
        // extra time cuz if login then msg shown for like 5secs so we need to add more time to wait
        let successful = match self.try_to_login(&page).await {
            Ok(_successful) => {
                // need to wait till redircet takes place
                wait_for(5).await;
                _successful
            },
            // Hence not error , in_visisble
            Err(error) if not_visible_cond(&error) => {
                let parms = NavigateParams::builder().url("https://www.reddit.com").build().unwrap();
                page.goto(parms).await?;
        
                // We are redirected from /login to well / reddit
                page.wait_for_navigation().await?;
                true
            },
            err @ _ => return err
        };

        self.set_theme(&page).await?;
        
        page.close().await?;
        Ok(successful)
    }

    async fn try_to_login(&self,page: &Page) -> chromiumoxide::Result<bool> {
        // enter username
        page.find_element(r#"[name="username"]"#)
            .await?
            .click()
            .await?
            .type_str(&self.username)
            .await?;

        // enter password
        page.find_element(r#"[name="password"]"#)
            .await?
            .click()
            .await?
            .type_str(&self.password)
            .await?;

        // click login button
        page.find_element(r#".AnimatedForm__submitButton"#)
            .await?
            .click()
            .await?;

        element_visibility(
            &page,
            ".AnimatedForm__errorMessage",
            |_| Ok(false),
            || Ok(true)
        ).await
    }
}


async fn element_visibility<T>(
    page: &Page,
    selector : impl Into<String>,
    on_visible : impl FnOnce(Element) -> chromiumoxide::Result<T>,
    on_not_visible : impl FnOnce() -> chromiumoxide::Result<T>,
) -> chromiumoxide::Result<T> {
    match page.find_element(selector).await {
        Ok(e) => on_visible(e),
        // Hence not error , in_visisble
        Err(error) if not_visible_cond(&error) => on_not_visible(),
        Err(error) => Err(error)
    }
}

fn not_visible_cond(error : &CdpError) -> bool {
    match &error {
        CdpError::Chrome(inner_msg@_) => inner_msg.code == -32000,
        _ => false  
    }
} 