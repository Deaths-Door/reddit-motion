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

- FFmpeg (System-Wide optional)
- Reddit account (optional) and access to the internet
- Local file system access

## Install

1. Install the tool 
```bash
cargo install reddit-motion
```
2. Intially use the `--edit` command to create a [configuration file](https://github.com/Deaths-Door/reddit-motion/blob/main/CONFIGURATION.md) and edit the configuration file to your liking and save it.

```bash
reddit-motion --edit
```
3. Use the tool 👌

### Youtube Downloading Limitations

**Cannot** download videos that fall into the following:
- Regionally restricted (requires a proxy)
- Private (if you have access, requires cookies)
- Rentals (if you have access, requires cookies)
- YouTube Premium content (if you have access, requires cookies)
- Only HLS Livestreams are currently supported. Other formats not will be fetch
- Generated download links are valid for 6 hours, and may only be downloadable from the same IP address.

When doing to many requests YouTube might block. This will result in your requests getting denied with HTTP Status Code 429.

## Author

👤 **Deaths-Door**

* Github: [@Deaths-Door](https://github.com/Deaths-Door)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check issues page.

## Show your support

Give a ⭐️ if this project helped you!