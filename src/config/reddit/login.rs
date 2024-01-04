use chromiumoxide::{Browser,error::CdpError, Page, Element};
use super::{RedditUser, wait_for};

impl RedditUser {
    /// Returns if login was successful
    pub async fn login_and_set_theme(&self,browser : &Browser) -> chromiumoxide::Result<bool> {
        let page = browser.new_page("https://www.reddit.com/login").await?;
    
        // If visible it means we are already logged in , so return
        // extra time cuz if login then msg shown for like 5secs so we need to add more time to wait
        let (successful,extra_time) = match is_element_visible(&page,"h1.Title").await? {
            Some(_) => (true,10),
            None => (self.inner(&page).await?,0),
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

    async fn inner(&self,page: &Page) -> chromiumoxide::Result<bool> {
        println!("TRYING TO LOGIN");

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

async fn is_element_visible(
    page: &Page,
    selector : impl Into<String>,
) -> chromiumoxide::Result<Option<Element>> {
    element_visibility(
        page, 
        selector, 
        |element| Ok(Some(element)), 
    || Ok(None)
    ).await
}