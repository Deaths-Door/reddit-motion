use roux::submission::SubmissionData;

use crate::{utils::VecCollectionDebug, RedditPage, StoryMode, SubredditTaskConfiguration};

use super::SubredditTaskPool;
use super::*;

impl<T, L> SubredditTaskPool
where
    T: TextToSpeech,
    L: Translate,
{
    fn new(tts: T, translate: L) -> Self {
        Self {
            resources: SubredditTaskPoolResources { tts, translate },
            groups: Vec::new(),
        }
    }
}

///
pub trait ExceuteTask: SubredditGenerationTask {
    /// Asynchronously creates a pool of subreddit tasks.
    ///
    /// This method generates a `SubredditTaskPool` by performing the following steps:
    /// 1. Initializes a new `Subreddit` instance using the name specified in the
    ///    `SubredditTaskConfiguration`.
    /// 2. Creates a default `SubredditTaskPool` to hold the task groups.
    /// 3. Iterates over the number of times specified by `repeat_count`, creating
    ///    a new task group for each iteration. For each iteration:
    ///    - Calls `create_task` to attempt to create a new task group associated
    ///      with the subreddit.
    ///    - If task creation is successful, the new task group is added to the pool.
    ///    - If an error occurs during task creation, the error is reported using
    ///      `report_creating_task_failed`.
    ///
    /// This function returns the populated `SubredditTaskPool`.
    ///
    /// # Returns
    /// A `SubredditTaskPool` containing the generated task groups based on the
    /// subreddit configuration.
    /// This will the [Option::None] when fetching submissions in bulk returns error
    async fn create_pool(self) -> Option<SubredditTaskPoolHidden<Self>>;
}

impl<T> ExceuteTask for T
where
    T: SubredditGenerationTask,
{
    /// Asynchronously creates a pool of subreddit tasks.
    ///
    /// This method generates a `SubredditTaskPool` by performing the following steps:
    /// 1. Initializes a new `Subreddit` instance using the name specified in the
    ///    `SubredditTaskConfiguration`.
    /// 2. Creates a default `SubredditTaskPool` to hold the task groups.
    /// 3. Iterates over the number of times specified by `repeat_count`, creating
    ///    a new task group for each iteration. For each iteration:
    ///    - Calls `create_task` to attempt to create a new task group associated
    ///      with the subreddit.
    ///    - If task creation is successful, the new task group is added to the pool.
    ///    - If an error occurs during task creation, the error is reported using
    ///      `report_creating_task_failed`.
    ///
    /// This function returns the populated `SubredditTaskPool`.
    ///
    /// # Returns
    /// A `SubredditTaskPool` containing the generated task groups based on the
    /// subreddit configuration.
    /// This will the [Option::None] when fetching submissions in bulk returns error
    async fn create_pool(mut self) -> Option<SubredditTaskPoolHidden<Self>> {
        let configuration = self.subreddit_configuration();
        let subreddit = roux::Subreddit::new(&configuration.name);

        let tts = self.text_to_speech();
        let translator = self.translator();
        let mut pool = SubredditTaskPool::new(tts, translator);

        // Attempt to retrieve the new entries
        let count = configuration.repeat_count.get();

        // First, we attempt to retrieve cached entries for the given subreddit.
        // If successful, we proceed by calculating how many new entries we still need to fetch
        let mut entries = {
            let output = match self.get_cached_entries(&subreddit, count).await {
                Ok(cached_entries) => {
                    const MESSAGE: &str = "Failed to convert cached_entries length to u8";

                    let len: u8 = cached_entries.len().try_into().expect(MESSAGE);

                    // Fetch new entries to make up for the difference between the total desired `count`
                    // and the number of cached entries we already have.
                    self.new_entries(&subreddit, count - len).await
                }
                error @ Err(_) => error,
            };

            self.report_if_required(output)
        }?;

        assert_eq!(entries.len(), count as usize);

        // Example Array of Entries:
        // entries = ["entry1", "entry2", "entry3", "entry4"]
        // For test purposes - https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2f9e7a95398843c5cd4c0a7caa829dce

        // Initialize the lower_bound to track the current index of entries being processed.
        let mut lower_bound: usize = 0;

        // Initialize unprocessed_upper_bound to keep track of the length of entries to be processed.
        // This is used to ensure we only iterate over the relevant portion of the entries.
        let mut unproccessed_upper_bound = entries.len();

        // Iterate over the entries until the lower_bound is less than unprocessed_upper_bound.
        while lower_bound < unproccessed_upper_bound {
            let entry_ref = &entries[lower_bound];

            // We only start processing the task if `self.report_if_required(self.prequsite_task(entry_ref).await)`
            // returns `Some((story_mode, page))`. This means the prerequisite task was successful, and we can
            // proceed to the next step.
            // Until this point, we don't need ownership of the entry because we're only checking if it's
            // ready to be processed.
            match self.report_if_required(self.prequsite_task(entry_ref).await) {
                Some((story_mode, page)) => {
                    // Example Visualization:
                    // Before swap_remove:
                    // entries = ["entry1", "entry2", "entry3", "entry4"]
                    //                  ^
                    //               lower_bound
                    //
                    // After swap_remove:
                    // entries = ["entry4", "entry2", "entry3"]  // "entry1" removed, "entry4" moved to index 0
                    //                  ^
                    //               lower_bound1

                    // If the prerequisite task is successful, swap the current entry with the last
                    // entry in the vector and remove it from the list.
                    // **IMPORTANT** - This does not preserve ordering of the remaining elements, but is *O*(1).
                    let entry = entries.swap_remove(lower_bound);
                    let group = self.postrequisite_task(story_mode, page, entry);

                    pool.extend(std::iter::once(group));

                    // Decrement unprocessed_upper_bound as we've successfully processed an entry.
                    // This indicates that the vector's effective length has decreased due to the
                    // swap_remove operation.
                    unproccessed_upper_bound -= 1;
                }
                None => {
                    // Increment lower_bound to process the next entry.
                    lower_bound += 1;
                }
            }
        }

        let cache_result = self.cache_entries(entries).await;
        self.report_if_required(cache_result);

        Some(pool)
    }
}

/// Just there so I can still use self...
trait SealedUtils<T>
where
    T: SubredditGenerationTask,
{
    async fn prequsite_task<'l>(
        &self,
        entry: &SubredditTaskEntry<'l>,
    ) -> Result<(StoryMode, RedditPage), T::Error>;

    fn postrequisite_task<'l>(
        &self,
        story_mode: StoryMode,
        page: RedditPage,
        entry: SubredditTaskEntry<'l>,
    ) -> SubredditTaskGroup;

    fn report_if_required<O>(&mut self, result: Result<O, T::Error>) -> Option<O>;
}

impl<T> SealedUtils<T> for T
where
    T: SubredditGenerationTask,
{
    async fn prequsite_task<'l>(
        &self,
        entry: &SubredditTaskEntry<'l>,
    ) -> Result<(StoryMode, RedditPage), T::Error> {
        // Resolve the story mode
        let story_mode = self
            .subreddit_configuration()
            .story_mode
            .clone()
            .resolve_mode(&entry.submission)?;

        // Create a new Reddit page
        let page = RedditPage::new(self.reddit_browser(), &entry.submission).await?;

        Ok((story_mode, page))
    }

    fn postrequisite_task<'l>(
        &self,
        story_mode: StoryMode,
        page: RedditPage,
        entry: SubredditTaskEntry<'l>,
    ) -> SubredditTaskGroup {
        let mut group = SubredditTaskGroup::new(page, entry.submission);

        // Add default language task
        let task = SubredditTask::new(story_mode.clone(), None);
        group.extend(std::iter::once(task));

        // TODO - Not sure why this is needed , and why not .to_owned() is sufficed
        let owned = match entry.extra_languages {
            std::borrow::Cow::Borrowed(value) => value.to_owned(),
            std::borrow::Cow::Owned(value) => value,
        };

        let tasks = owned
            .into_iter()
            .map(|language| SubredditTask::new(story_mode.clone(), Some(language)));

        group.extend(tasks);

        group
    }

    fn report_if_required<O>(&mut self, result: Result<O, T::Error>) -> Option<O> {
        match result {
            Ok(value) => Some(value),
            Err(error) => {
                self.report_creating_task_failed(error);
                None
            }
        }
    }
}

impl std::fmt::Display for StoryModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self.0 {
            StoryMode::Auto => "neither mode was possible",
            StoryMode::ReadComments { .. } => "there are no comments on this post",
            StoryMode::ReadPost => "the post body is empty",
        };

        write!(f, "Unable to proceed: {message}")
    }
}

impl StoryMode {
    const fn default_read_comments() -> Self {
        Self::ReadComments {
            max_comments: Some(20),
        }
    }

    fn resolve_mode(self, submission: &SubmissionData) -> Result<Self, StoryModeError> {
        match self {
            Self::Auto => match Self::ReadPost.resolve_mode(submission) {
                Err(_) => Self::default_read_comments().resolve_mode(submission),
                Ok(value) => Ok(value),
            },
            Self::ReadComments { .. } if submission.num_comments == 0 => {
                Err(StoryModeError(Self::default_read_comments()))
            }
            Self::ReadPost if submission.selftext.is_empty() => Err(StoryModeError(Self::ReadPost)),
            _ => Ok(self.clone()),
        }
    }
}

impl SubredditTaskGroup {
    const fn new(page: RedditPage, submission: roux::submission::SubmissionData) -> Self {
        Self {
            resources: SubredditTaskGroupResources { page, submission },
            tasks: Vec::new(),
        }
    }
}

impl SubredditTask {
    const fn new(story_mode: StoryMode, language: Option<unic_langid::LanguageIdentifier>) -> Self {
        Self {
            story_mode,
            language,
        }
    }
}

impl Extend<SubredditTask> for SubredditTaskGroup {
    fn extend<T: IntoIterator<Item = SubredditTask>>(&mut self, iter: T) {
        self.tasks.extend(iter)
    }
}

impl std::fmt::Debug for SubredditTaskGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubredditTaskPool")
            .field("tasks", &VecCollectionDebug::from(self.tasks.as_slice()))
            .finish()
    }
}

impl IntoIterator for super::SubredditTaskPool {
    type IntoIter = <Vec<super::SubredditTaskGroup> as IntoIterator>::IntoIter;
    type Item = super::SubredditTaskGroup;
    fn into_iter(self) -> Self::IntoIter {
        self.groups.into_iter()
    }
}

impl Extend<SubredditTaskGroup> for super::SubredditTaskPool {
    fn extend<T: IntoIterator<Item = SubredditTaskGroup>>(&mut self, iter: T) {
        self.groups.extend(iter)
    }
}
