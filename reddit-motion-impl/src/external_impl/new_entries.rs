use crate::{RedditTaskError, SubredditIndividualTask};
use reddit_motion_core::{
    roux::{util::FeedOption, Subreddit},
    SubredditTaskEntry,
};

impl SubredditIndividualTask<'_> {
    pub(super) async fn internal_new_entries<'a, 'l>(
        &'a mut self,
        subreddit: &Subreddit,
        count: u8,
    ) -> Result<Vec<SubredditTaskEntry<'l>>, RedditTaskError> {
        let submissions = subreddit.top(count.into(), None).await?;
        todo!()
    }
}
