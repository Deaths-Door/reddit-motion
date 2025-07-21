use crate::{SubredditTaskPool, VideoSourceFiles};

use super::{SourceAggregator, SubredditTaskGroup, SubredditTaskPoolResources, TextToSpeech};

impl<T> SubredditTaskPool<T>
where
    T: TextToSpeech,
{
    async fn begin_exceution(self) {
        for group in self.groups {
            group.begin_exceution(&self.resources);
        }
    }
}

impl SubredditTaskGroup {
    async fn begin_exceution<T>(self, pool_resources: &SubredditTaskPoolResources<T>)
    where
        T: TextToSpeech,
    {
        for task in self.tasks {
            // TODO - thiink about this for languages - maybe reload page ... or delete the created elements
            let source_aggregator = SourceAggregator::new(base_directory, &task, &self.resources);
            if let Err(error) = soucre_aggregator.proccess_title(pool_resources).await {
                // TODO : Report this error
            }
        }
    }
}
