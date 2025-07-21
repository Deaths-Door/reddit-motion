use std::path;

use serde::{Deserialize, Serialize};

use crate::{AudioSource, Dimensions, FileAssociation, ImageSource, SubredditTaskPoolResources};

use super::SourceAggregator;

/// A collection of `PostContentItem`s representing metadata for processed post elements.
///
/// This struct holds a vector of `PostContentItem` objects, which mirror the structure used
/// in the JavaScript `processPostContent` function. Each item contains metadata about a text
/// element, such as its CSS selector, content, and an optional flag for adding white noise.
///
/// **Keep this in sync with the corresponding JS function:**  
/// - `process_post_content.js`
///
/// This ensures consistency between Rust and JS when working with metadata objects for text elements.
#[derive(Serialize, Deserialize)]
struct PostContent(Vec<PostContentItem>);

/// Metadata for a single processed text element in a post.
/// This structure aligns with the JS return type:
/// ```js
/// * @returns {Array<Object>} An array of metadata objects for the processed text elements.
/// * Each object contains:
/// *   - `selector`: A unique CSS selector for the element.
/// *   - `text`: The text content of the element.
/// *   - **Optional** `separator` {string}: Specifies the type of separation between consecutive elements.
/// *      - `"whitenoise"`: Adds audio gaps before and after the blockquote to distinguish it.
/// *      - `"space"`: Adds space or silence to separate `<p>` elements for readability.
/// *      - `"none"`: No additional separation.
/// ```
#[derive(Serialize, Deserialize)]
struct PostContentItem {
    selector: String,
    text: String,
    seperator: Option<Seperator>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum Seperator {
    Whitenoise,
    Space,
}

impl SourceAggregator<'_> {
    pub(in crate::video) async fn proccess_post_content<T>(
        &mut self,
        pool_resource: SubredditTaskPoolResources<T>,
    ) -> chromiumoxide::Result<()> {
        const QUERY: &str = include_str!("../js/process_post_content.js");

        let query_with_args = format!(
            "{QUERY}(\"{id}\",{dimensions})",
            id = self.resources.submission.id,
            pool_resource.dimensions.to_js_object()
        );

        let content: PostContent = self
            .resources
            .page
            .evaluate(query_with_args)
            .await?
            .into_value()?;

        // TODO - translate this shit
        self.entry.language;

        for element in content.0.into_iter() {
            if let Some(seperator) = element.seperator {
                let tts = &pool_resource.tts;

                let pathbuf = match seperator {
                    Seperator::Whitenoise => tts.add_noise().await?,
                    Seperator::Space => tts.add_silence().await?,
                };

                let association = FileAssociation::AudioOnly(AudioSource(pathbuf));
                self.source_files.sources.push(association);
            }

            // So that the same file name is used each time for the same dimensions
            let file_name = &element.selector;

            let audio_pathbuf = pool_resource
                .tts
                .save_speech_at(&element.text, &self.source_files.base_directory, file_name)
                .await?;

            let element = self.resources.page.find_element(element.selector).await?;

            let image_pathbuf = self.screen_shot_post_element(&file_name, element).await?;

            self.source_files.sources.push(FileAssociation::Single {
                audio: AudioSource(audio_pathbuf),
                image: ImageSource(image_pathbuf),
            });
        }

        Ok(())
    }
}

impl Dimensions {
    // Just so I control the exact output
    fn to_js_object(&self) -> String {
        format!(r#"{{ "width" : {width} , "height" : {height} }}"#)
    }
}
