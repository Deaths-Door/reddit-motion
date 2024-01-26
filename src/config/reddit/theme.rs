use chromiumoxide::Page;

use super::RedditUser;

impl RedditUser {
    pub(super) async fn set_theme(&self,page : &Page) -> chromiumoxide::Result<()> {
        // Basically we change the users theme settings
        
        // click to open popup
        page.find_element("button[id=\"USER_DROPDOWN_ID\"]")
            .await?
            .click()
            .await?;

        let _elements =  page.find_elements("button._2e2g485kpErHhJQUiyvvC2")
            .await?;
        
        // the last element from a 3 element list
        let theme_checkbox = _elements.last().unwrap();
        
        let is_dark_mode = {
            let attr = theme_checkbox.attribute("aria-checked").await?.unwrap();
            attr.parse::<bool>().unwrap()
        };
    
        // is_dark_mode | use_dark_mode
        // true         | true
        // false        | true
        // true         | false 
        // Hence we can simply inverse it by clicking
        if !(self.use_dark_mode && is_dark_mode) {
            theme_checkbox.click().await?;  
        }

        Ok(())
    }
}