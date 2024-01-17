use chromiumoxide::{Browser,error::CdpError, Page, Element};
use super::{RedditUser, wait_for};

impl RedditUser {
    /// Returns if login was successful
    pub async fn login_and_set_theme(&self,browser : &Browser) -> chromiumoxide::Result<bool> {
        let page = browser.new_page("https://www.reddit.com/login").await?;
    
        // If visible it means we are already logged in , so return
        // extra time cuz if login then msg shown for like 5secs so we need to add more time to wait
        let (successful,extra_time) = match self.try_to_login(&page).await {
            Ok(result) => (result,0),
            // Hence not error , in_visisble
            Err(error) if matches!(error,CdpError::NotFound) => (true,10),
            err @ _ => return err
        };

        // We are redirected from /login to well / reddit
        page.wait_for_navigation().await?;

        // Give time for page to load
        // choriumoxide doesnt offer a fn to wait till page load
        wait_for(5 + extra_time).await;

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
        Err(error) if matches!(error,CdpError::NotFound) => on_not_visible(),
        Err(error) => Err(error)
    }
}