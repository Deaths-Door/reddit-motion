use super::RedditUser;
use chromiumoxide::{Browser, Page};

/// RedditThemeError
#[derive(thiserror::Error, Debug)]
#[error("Failed setting theme: {0}")]
pub struct RedditThemeError(#[from] chromiumoxide::error::CdpError);

/// RedditLoginError
#[derive(thiserror::Error, Debug)]
#[error("Failed Login : {0}")]
pub struct RedditLoginError(#[from] chromiumoxide::error::CdpError);

impl RedditUser {
    /// Returns if login was successful
    pub async fn login_and_set_theme(&self, browser: &Browser) -> super::RedditResult<()> {
        let page = browser.new_page("https://www.reddit.com/login").await?;
        self.try_to_login(&page).await?;
        self.set_theme(&page).await?;
        page.close().await?;
        Ok(())
    }

    async fn try_to_login(&self, page: &Page) -> Result<(), RedditLoginError> {
        const QUERY: &str = include_str!("../../js/login.js");

        let _ = page
            .evaluate_function(format!(
                "{QUERY}({username},{password})",
                username = self.username,
                password = self.password
            ))
            .await?;

        Ok(())
    }

    async fn set_theme(&self, page: &Page) -> Result<(), RedditThemeError> {
        const QUERY: &str = include_str!("../../js/change_theme.js");

        let _ = page
            .evaluate_function(format!(
                "{QUERY}({useDarkMode})",
                useDarkMode = self.theme.is_dark()
            ))
            .await?;

        Ok(())
    }
}
