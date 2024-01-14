# Welcome to reddit-motion 👋

[![made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)

[![Crates.io](https://img.shields.io/crates/v/reddit-motion.svg)]([https://www.rust-lang.org/](https://crates.io/crates/reddit-motion))
![docs.rs](https://img.shields.io/docsrs/reddit-motion)
![Crates.io License](https://img.shields.io/crates/l/reddit-motion)

> If you encounter any bugs, have suggestions for new features, or need help using this project, please file an issue on the GitHub issues

Transform Reddit Posts and Custom Text into Engaging Videos with Localization. Customize Content, Boost Speed, Share on Social Media – Experience its Excellence!

Key Features:

* **Dynamic Reddit Integration:** Convert Reddit posts into captivating videos, expanding your content's reach.
* **Customizable Creations:** Tailor videos to perfection with customiable video creation
* **Global Reach:** Seamlessly localize videos for international viewers, building connections worldwide.
* **Lightning-Fast Rendering:** Experience unparalleled speed and efficiency in video production.

## Motivation

We've all seen those viral videos on TikTok, YouTube, and Instagram – they effortlessly amass millions of views, yet the creative process behind them often involves minimal effort. The real magic lies in smart editing and skillful content aggregation...

... Now, imagine if you could automate that entire process? 🤯

## Prerequisites:

- FFmpeg System-Wide (optional)
- Reddit account (optional) and access to the internet
- Local file system access

## Install
1. Install the tool 
```bash
cargo install reddit-motion
```
2. Intially use the `--edit` command to create a configuration file and edit the configuration file to your liking and save it.
```bash
reddit-motion --edit
```
3. Use the tool 👌

## Assets Directory

This tool supports videos and audio from local and YouTube sources. To include videos and audio from local sources, place them in the assets directory. For YouTube videos, you can specify the video URL in the configuration file. The tool will automatically download the video and include it in the final video.

The `assets directory` can be a single directory or a recursive structure of directories. The tool will search the `assets directory` and its subdirectories for any videos or audio files that match the specified patterns. The patterns can be specified using the following syntax:

- `*`: Matches any character except a slash ('/').
- `**`: Matches zero or more directories.

Eg : `/media/**/*.jpg`

For more information look at [this](https://docs.rs/glob/latest/glob)

### Youtube Downloading Limitations

**Cannot** download videos that fall into the following:
- Regionally restricted (requires a proxy)
- Private (if you have access, requires cookies)
- Rentals (if you have access, requires cookies)
- YouTube Premium content (if you have access, requires cookies)
- Only HLS Livestreams are currently supported. Other formats not will be fetch
- Generated download links are valid for 6 hours, and may only be downloadable from the same IP address.

When doing to many requests YouTube might block. This will result in your requests getting denied with HTTP Status Code 429.

## Configuration File
This tool uses a configuration file to store various settings related to the language, assets, dimensions, text-to-speech, Reddit, and translation. 

*Example Configuration File*
```toml
# Language in which the tool will be in
lang = "en"

# Assets to be used in the video
[assets]
# Ensure it is filled with some value
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
    video_length_limit = 60
},
{
    name = "subreddit2"
    mode = "auto"
}
]
```
The following table describes the various settings that can be specified in the configuration file:
| Setting    | Description                                                                                                                                     |
|------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| lang       | The language in which the tool is in. *Available languages:* German, English, Spanish, French, Hindi, Italian, Japanese, Turkish, Ukrainian                                                                          |
| assets     | A list of structures containing information about the assets to be used in the video. See the Assets section for more details.                  |
| dimensions | A structure defining the dimensions of the output video. See the Dimensions section for more details.                                           |
| tts        | The text-to-speech service to use for generating the video narration. Defaults to Google. See the TextToSpeechService section for more details. |
| reddit     | A structure containing configuration options for accessing Reddit. See the Reddit section for more details.                                     |


The `Dimensions` struct defines the dimensions of the output video. It specifies the width and height of the video.

Member | Description                                                      | 
------ | -----------------------------------------------------------------| 
width  | The width of the output video in pixels. Defaults to 640 pixels. | 
height | The height of the output video in pixels. Defaults to 360 pixels.| 

The tts setting defines the text-to-speech service to use for generating the video narration. Currently, the following services are supported:

Service    | Description                                                                                                         |
---------- | ------------------------------------------------------------------------------------------------------------------- |
Google     | The Google text-to-speech service. This is the default service.                                                     |
Elevenlabs | The Elevenlabs text-to-speech service. This requires an API key, which can be obtained from the Elevenlabs website. |

If you are using the `Elevenlabs` service, you will need to specify your `API key` in the configuration file. The API key should be placed in the `api_key` field of the TextToSpeechService structure.  For example:

```toml
[tts.elevenlabs]
api_key = "YOUR_API_KEY"
# You can also specify the model and voice name to be used for the Elevenlabs service.
# model = "..."
# voice = "..."
```

For more information about this API , visit this [documentation](https://elevenlabs.io/docs/api-reference)

The `RedditConfig` struct contains configuration options for accessing Reddit.

Member     | Description                                                                                             
---------- | --------------------------------------------------------------------------------------------------------
user       | An optional RedditUser structure to specify a Reddit user account. Defaults to None.                    
subreddits | A list of SubredditConfig structures defining the subreddits to be processed. Defaults to an empty list.

The `RedditUser` struct holds the credentials for a Reddit user account.

Member        | Description                                                       
------------- | ------------------------------------------------------------------
username      | The Reddit username.                                              
password      | The Reddit password.                                              
use_dark_mode | Whether to use dark mode when accessing Reddit. Defaults to false.

The `SubredditConfig` struct defines a single subreddit to be processed. It specifies the subreddit name, repeat count, story mode, TTS service to use for narration, extra languages for narration, and video length limit.

Member             | Description                                                                                                     
------------------ | ----------------------------------------------------------------------------------------------------------------
name               | The name of the subreddit.                                                                                      
repeat_count       | The number of times to process the subreddit. Defaults to 1.                                                    
story_mode         | The story mode for the subreddit. Defaults to StoryMode::Default.                                               
for_tts_use        | The TTS service to use for narration for this subreddit. Defaults to the global tts setting.                    
extra_langs        | A list of extra languages to use for narration in addition to the language specified in the global lang setting.
video_length_limit | The maximum video length in seconds                                                    

## Author

👤 **Deaths-Door**

* Github: [@Deaths-Door](https://github.com/Deaths-Door)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check issues page.

## Show your support

Give a ⭐️ if this project helped you!
