use chromiumoxide::{Page, cdp::browser_protocol::network::CookieParam};

use crate::config::subreddit::SubredditConfig;

use super::args::ParameterArgs;

#[derive(serde::Deserialize)]
pub struct RedditConfig {
    #[serde(rename="dark_mode")]
    color_scheme_is_dark : bool,
    subreddits : Vec<SubredditConfig>,
}

impl RedditConfig {
    pub async fn handle(
        &self,
        parms : &mut ParameterArgs<'_>
    ) -> anyhow::Result<()> {        
        for subreddit in &self.subreddits {
            (parms.callback.on_new_subreddit)(&subreddit);
            
            subreddit.handle(
                self,
                parms
            ).await?;

            (parms.callback.on_end_subreddit)()
        }
        Ok(())
    }

    // TODO : Check why cookies are not working , is it cuz im not logged in??
    pub(in crate::config) async fn set_color_scheme(&self,page : &Page) -> chromiumoxide::Result<()> {
        if self.color_scheme_is_dark {
            page.set_cookie((*DARK_MODE).clone()).await?;
        };

        page.set_cookie((*DEFAULT_COLOR_SCHEME).clone()).await?;
        Ok(())
    }
}

lazy_static::lazy_static! {
    static ref DEFAULT_COLOR_SCHEME : CookieParam = create_cookie(  
        "eu_cookie", 
        "{%22opted%22:true%2C%22nonessential%22:false}"
    );

    static ref DARK_MODE : CookieParam = create_cookie(
        "USER", 
        "eyJwcmVmcyI6eyJ0b3BDb250ZW50RGlzbWlzc2FsVGltZSI6MCwiZ2xvYmFsVGhlbWUiOiJSRURESVQiLCJuaWdodG1vZGUiOnRydWUsImNvbGxhcHNlZFRyYXlTZWN0aW9ucyI6eyJmYXZvcml0ZXMiOmZhbHNlLCJtdWx0aXMiOmZhbHNlLCJtb2RlcmF0aW5nIjpmYWxzZSwic3Vic2NyaXB0aW9ucyI6ZmFsc2UsInByb2ZpbGVzIjpmYWxzZX0sInRvcENvbnRlbnRUaW1lc0Rpc21pc3NlZCI6MH19"
    );
}

fn create_cookie(
    name : &str,
    value : &str,
) -> CookieParam {
    CookieParam::builder()
        .name(name)
        .value(value)
        .domain(".reddit.com".to_string())
        .path("/".to_string())
        .build()
        .expect("Error creating cookie")
}