
// Box as I don't want to specific generics everywhere
pub struct Callback {
    pub invalid_reddit_credentials : Box<dyn Fn()>,
    pub login_successful : Box<dyn Fn()>
}