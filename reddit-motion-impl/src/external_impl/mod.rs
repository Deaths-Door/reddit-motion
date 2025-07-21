mod cache_entries;
mod new_entries;
mod retrieve_entries;

use reddit_motion_core::{
    roux::Subreddit, RedditBrowser, SubredditGenerationTask, SubredditTaskConfiguration,
    SubredditTaskEntry,
};

use crate::{RedditTaskError, SubredditIndividualTask};

impl SubredditGenerationTask for SubredditIndividualTask<'_> {
    type Error = RedditTaskError;

    fn reddit_browser(&self) -> &RedditBrowser {
        &self.resources.browser
    }

    fn subreddit_configuration(&self) -> &SubredditTaskConfiguration {
        &self.configuration.external
    }

    async fn new_entries<'a, 'l>(
        &'a mut self,
        subreddit: &Subreddit,
        count: u8,
    ) -> Result<Vec<SubredditTaskEntry<'l>>, Self::Error> {
        self.internal_new_entries(subreddit, count).await
    }

    async fn get_cached_entries<'a, 'l>(
        &'a mut self,
        subreddit: &Subreddit,
        wanted_count: u8,
    ) -> Result<Vec<SubredditTaskEntry<'l>>, Self::Error> {
        self.internal_get_cached_entries(subreddit, wanted_count)
            .await
    }

    async fn cache_entries<'a, 'l>(
        &'a mut self,
        entries: Vec<SubredditTaskEntry<'l>>,
    ) -> Result<(), Self::Error> {
        self.internal_cache_entries(entries).await
    }

    fn report_creating_task_failed(&mut self, error: Self::Error) {
        todo!()
    }
}
