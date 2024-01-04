# reddit-motion

Transform Reddit Posts and Custom Text into Engaging Videos with Localization. Customize Content, Boost Speed, Share on Social Media – Experience RedditVid Excellence!

Key Features:

* **Dynamic Reddit Integration:** Convert Reddit posts into captivating videos, expanding your content's reach.
* **Customizable Creations:** Tailor videos to perfection with customiable video creation
* **Global Reach:** Seamlessly localize videos for international viewers, building connections worldwide.
* **Lightning-Fast Rendering:** Experience unparalleled speed and efficiency in video production.

## Motivation

We've all seen those viral videos on TikTok, YouTube, and Instagram – they effortlessly amass millions of views, yet the creative process behind them often involves minimal effort. The real magic lies in smart editing and skillful content aggregation...

... Now, imagine if you could automate that entire process? 🤯

# TODO MAYBE CHANGE VOICE TTS PROVIDER
# TODO : FINISH THE README

requires ffmpeg 
required reddit / internet and local file system access
show config ..
can publsih ..
assets can be dirs /* => in dir (no subdirs) and /** means in all subdirs
# UTUBE DOWNLOADING
Limitations
rusty_ytdl cannot download videos that fall into the following

Regionally restricted (requires a proxy)
Private (if you have access, requires cookies)
Rentals (if you have access, requires cookies)
YouTube Premium content (if you have access, requires cookies)
Only HLS Livestreams are currently supported. Other formats not will be fetch
Generated download links are valid for 6 hours, and may only be downloadable from the same IP address.

Ratelimits
When doing to many requests YouTube might block. This will result in your requests getting denied with HTTP Status Code 429. The following steps might help you:

Use proxies (you can find an example proxy)
Extend on the Proxy Idea by rotating (IPv6)Addresses (you can find an example IPv6)
Use cookies (you can find an example cookies)
for this to take effect you have to first wait for the current ratelimit to expire!
Wait it out

cargo isntall -> crate -- --edit ... create config.toml , then run it crate
Translate assets into more langs
# assets dirs can follow this pattern
Support for matching file paths against Unix shell style patterns.

The glob and glob_with functions allow querying the filesystem for all files that match a particular pattern (similar to the libc glob function). The methods on the Pattern type provide functionality for checking if individual paths match a particular pattern (similar to the libc fnmatch function).

For consistency across platforms, and for Windows support, this module is implemented entirely in Rust rather than deferring to the libc glob/fnmatch functions.

Examples
To print all jpg files in /media/ and all of its subdirectories.

use glob::glob;

for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
To print all files containing the letter “a”, case insensitive, in a local directory relative to the current working directory. This ignores errors instead of printing them.

use glob::glob_with;
use glob::MatchOptions;

let options = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};
for entry in glob_with("local/*a*", options).unwrap() {
    if let Ok(path) = entry {
        println!("{:?}", path.display())
    }
}