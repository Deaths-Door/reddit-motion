use crate::{AudioSource, FileAssociation, ImageSource, RedditPage, SubredditTaskPoolResources};

use super::{utils::screen_shot_post_element, SourceAggregator, TextToSpeech};

impl SourceAggregator<'_> {
    pub(in crate::video) async fn proccess_title<T>(
        &mut self,
        pool_resource: SubredditTaskPoolResources<T>,
    ) -> chromiumoxide::Result<()>
    where
        T: TextToSpeech,
    {
        let id = &self.resources.submission.id;
        const QUERY: &str = include_str!("../js/change_post_title_and_return_selector.js");

        let (title, title_selector) = {
            // TODO - Translate
            let title_parm_og: Option<String> = match self.entry.language {
                Some(language) => pool_resource.translate.translate(..).await?,
                None => None,
            };

            let title_parm = title_parm_og.as_ref().or(Some("null"));

            let selector: String = self
                .resources
                .page
                .evaluate_function(format!("(QUERY)(\"{id}\",\"{title_parm}\")"))
                .await?
                .into_value()?;

            let text = title_ptitle_parm_ogarm.unwrap_or(self.resources.submission.title);

            (text, selector)
        };

        let png_path = {
            let element = self.resources.page.find_element(title_selector).await?;
            self.screen_shot_post_element("title", element).await
        }?;

        let audio_path = resource
            .tts
            .save_speech_at(title, &self.source_files.base_directory, "title")
            .await?;

        let association = FileAssociation::Single {
            audio: AudioSource(audio_path),
            image: ImageSource(png_path),
        };

        self.source_files.sources.push(association);

        Ok(())
    }
}
