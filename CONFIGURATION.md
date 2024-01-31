# Configuration File

This tool uses a configuration file to store various settings related to the language, assets, dimensions, text-to-speech, Reddit, and translation.

*Example Configuration File*

```toml
# Language in which the tool will be in
lang = "en"

# Assets to be used in the video
[assets]
video = []
audio = []

# Dimensions of the output video
[dimensions]
width = 800
height = 600

# Text-to-speech service to use for generating the video narration
tts = "Google"

# Reddit configuration
[reddit]
subreddits = [
{
    name = "subreddit1"
    repeat_count = 2
    for_tts_use = "Elevenlabs"
    extra_langs = ["de", "fr"]
    video_duration = "infinite"
},
    { name = "askreddit" , mode = { comments = { max_comments = 2 } } , extra_langs = [] }

]
```


| Setting    | Description                                                                                                                                                            |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| lang       | The language in which the tool is in.*Available languages:* German, English, Spanish, French, Hindi, Italian, Japanese, Turkish, Ukrainian                             |
| assets     | A list of structures containing information about the [assets] to be used in the video. See the Assets section for more details.                                       |
| dimensions | A structure defining the[dimensions](#dimensions) of the output video. See the Dimensions section for more details.                                                    |
| tts        | The[text-to-speech service](#texttospeechservice) to use for generating the video narration. Defaults to Google. See the TextToSpeechService section for more details. |
| reddit     | A structure containing[configuration options for accessing Reddit](#reddit-config). See the Reddit section for more details.                                           |

## Assets Directory

This tool supports videos and audio from local and YouTube sources. To include videos and audio from local sources, place them in the assets directory. For YouTube videos, you can specify the video URL in the configuration file. The tool will automatically download the video and include it in the final video.

The `assets directory` can be a single directory or a recursive structure of directories. The tool will search the `assets directory` and its subdirectories for any videos or audio files that match the specified patterns. The patterns can be specified using the following syntax:

- `*`: Matches any character except a slash ('/').
- `**`: Matches zero or more directories.

Eg : `/media/**/*.jpg`

For more information look at [this](https://docs.rs/glob/latest/glob)


| Field Name | Description                                                                                                   | Default Value |
| ------------ | --------------------------------------------------------------------------------------------------------------- | --------------- |
| videos     | A vector of paths to video files to be used in the generated video.                                           | Empty vector  |
| audio      | A vector where each specifies a path to an audio file to be used in the generated video and its volume level. | Empty vector  |

## Reddit Config

The  struct contains configuration options for accessing Reddit.


| Member     | Description                                                                                       | Default Value  |
| ------------ | --------------------------------------------------------------------------------------------------- | ---------------- |
| user       | An optional[RedditUser](#reddit-user) structure to specify a Reddit user account.                 | `None`.        |
| subreddits | A list of[SubredditConfig](#subreddit-config) structures defining the subreddits to be processed. | An empty list. |

## RedditUser

The struct holds the credentials for a Reddit user account.


| Member        | Description                                     | Default Value  |
| --------------- | ------------------------------------------------- | ---------------- |
| username      | The Reddit username.                            | Not specified. |
| password      | The Reddit password.                            | Not specified. |
| use_dark_mode | Whether to use dark mode when accessing Reddit. | `false`.       |

## Subreddit Config

The struct defines a single subreddit to be processed. It specifies the subreddit name, repeat count, story mode, TTS service to use for narration, extra languages for narration, and video length limit.


| Member         | Description                                                                                                       | Default Value                             |
| ---------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| name           | The name of the subreddit.                                                                                        | Not specified.                            |
| repeat_count   | The number of times to process the subreddit.                                                                     | 1.                                        |
| story_mode     | The story mode for the subreddit.                                                                                 | [StoryMode::Auto](#story-mode).           |
| for_tts_use    | The[TTS service](#texttospeechservice) to use for narration for this subreddit.                                   | `TextToSpeechService::Google`             |
| extra_langs    | A list of extra languages to use for narration in addition to the language specified in the global`lang` setting. | Empty list.                               |
| video_duration | The maximum video length in seconds.Refer to                                                                      | [VideoDuration::Infinite](#videoduration) |

## ExternalScripts

The `ExternalScripts` struct specifies scripts to be executed after the videos are generated. The tool supports two types of scripts:

* **Limited scripts:** These scripts are executed after generating videos with a limited duration (specified by the `VideoDuration` setting).
* **Infinite scripts:** These scripts are executed after generating videos with an infinite duration (the default).

The `ExternalScripts` struct has two fields:

* `limited`: An optional the command to be executed after generating videos with a limited duration.
* `infinite`: An optional command to be executed after generating videos with an infinite duration.

If both `limited` and `infinite` are set, both command will be executed.

The scripts are executed asynchronously, so the tool will continue to run while the scripts are executing.

The scripts can be used to do anything you want, such as:

* **Upload the videos to a video sharing service.**
* **Send the videos to a recipient.**
* **Process the videos further, such as adding captions or effects.**

**Parsing Command-Line Arguments**

If you want to parse the command-line arguments in your own script, you can use the `./examples/reddit_motion.py` script as a reference or wrapper. This script will parse the command-line arguments and pass them to your script as environment variables

**Example:**
````toml
[scripts]
infinite = "python ./examples/external_script.py" 
limited = "python ./examples/external_script.py" 
```
````

## Story Mode

The enum defines the mode in which the tool will process the content of a subreddit.

**Possible Values:**

- **Auto**: This is the default mode. The tool will automatically determine the most appropriate way to process the content based on the subreddit's rules and structure.

* **ReadComments:** In this mode, the tool will only process the top-level comments of posts. The `max_comments` field can be used to specify the maximum number of comments to process.
* **ReadPost:** In this mode, the tool will process the entire content of posts, including the title, body, and comments.

## TextToSpeechService

The `TextToSpeechService` enum defines the text-to-speech service to use for generating the narration of Reddit posts.

**Possible Values:**

* **Google:** This is the default service. It uses the Google Cloud Text-to-Speech API to generate high-quality narration.
* **Elevenlabs:** This service uses the Elevenlabs text-to-speech API to generate narration. It requires an API key, which can be obtained from the Elevenlabs website.

**Elevenlabs Service Options:**

* **api_key:** The Elevenlabs API key.
* **model:** The name of the text-to-speech model to use. The default model is "default_model".
* **voice_name:** The name of the text-to-speech voice to use. The default voice is "default_voice".

For more information about this API , visit this [documentation](https://elevenlabs.io/docs/api-reference)

## TranslationServices

The struct stores the API keys for various translation services that can be used for the tool. It is used by the `SubredditConfig` struct to specify the translation service to use for narration for a particular subreddit and the `extra_langs` field to specify additional languages to translate the narration into.


| **Field Name**  | **Description**                                |
| ----------------- | ------------------------------------------------ |
| `deepl_api_key` | The API key for the DeepL translation service. |

## VideoDuration

The `VideoDuration` enum defines the maximum length of the generated videos.

**Possible Values:**

* **Infinite:** This is the default mode. The tool will not limit the length of the videos.
* **Limited:** In this mode, the tool will limit the length of the videos to the specified `limit` in seconds.
* **Both:** In this mode, the tool will generate two versions of the video: one with infinite duration and one with the specified `limit` in seconds.

## Dimensions

The struct defines the dimensions of the output video. It specifies the width and height of the video.


| Member | Description                               | Default Value |
| -------- | ------------------------------------------- | --------------- |
| width  | The width of the output video in pixels.  | 640 pixels.   |
| height | The height of the output video in pixels. | 360 pixels.   |
