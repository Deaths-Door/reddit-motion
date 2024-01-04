use unic_langid::LanguageIdentifier;


// Box as I don't want to specific generics everywhere
pub struct Callback {
    pub(in crate::config) invalid_reddit_credentials : Box<dyn Fn(&LanguageIdentifier)>,
    pub(in crate::config) login_successful : Box<dyn Fn(&LanguageIdentifier)>,
    pub(in crate::config) on_new_subreddit : Box<dyn Fn(&LanguageIdentifier,&str)>,
    pub(in crate::config) on_end_subreddit : Box<dyn Fn(&LanguageIdentifier)>,
}

impl Callback {
    pub fn new(
        invalid_reddit_credentials: impl Fn(&LanguageIdentifier) + 'static, 
        login_successful: impl Fn(&LanguageIdentifier) + 'static, 
        on_new_subreddit: impl Fn(&LanguageIdentifier,&str) + 'static, 
        on_end_subreddit: impl Fn(&LanguageIdentifier) + 'static
    ) -> Self { 
        Self { 
            invalid_reddit_credentials : Box::new(invalid_reddit_credentials), 
            login_successful  : Box::new(login_successful), 
            on_new_subreddit  : Box::new(on_new_subreddit) , 
            on_end_subreddit  : Box::new(on_end_subreddit) 
        } 
    }
}