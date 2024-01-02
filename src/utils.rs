use crate::callback::Callback;
use crate::ffmpeg::{self, FFmpeg};
use colored::Colorize;

pub fn print_banner() {
    let banner = r#"
    ____           __    ___ __     __  ___      __  _           
   / __ \___  ____/ /___/ (_) /_   /  |/  /___  / /_(_)___  ____ 
  / /_/ / _ \/ __  / __  / / __/  / /|_/ / __ \/ __/ / __ \/ __ \
 / _, _/  __/ /_/ / /_/ / / /_   / /  / / /_/ / /_/ / /_/ / / / /
/_/ |_|\___/\__,_/\__,_/_/\__/  /_/  /_/\____/\__/_/\____/_/ /_/ 
"#.bright_red();

    let thanks = "Thanks for using this tool! Feel free to contribute to this project on GitHub!".bold();
    let questions = "If you have any questions, feel free to contact me by submitting a GitHub issue at ".green();
    let solutions = "You can find solutions to many common FAQs at ".green();

    let github_link = "https://github.com/Deaths-Door/reddit-motion".blue();
    let doc_link = "https://docs.rs/reddit_motion".blue();

    // As they are the longest
    let seperater = "=".repeat(questions.len() + github_link.len());
    println!("{banner}\n{thanks}\n{questions}{github_link}\n{solutions}{doc_link}\n{seperater}");
}

use serde_json::Value;
pub async fn check_and_install_latest_version() -> anyhow::Result<()> {
    const URL : &str = "https://api.github.com/repos/Deaths-Door/reddit-motiont/releases/latest";
   
    let client = reqwest::Client::builder()
        .user_agent("reddit-motion")
        .build()?;

    let response = client.get(URL).send().await?;
    let data : Value = response.json().await?;

    // If none then that means its the first release 
    if let Some(_release_version) = data.get("tag_name") {
        if let Value::String(release_version) = _release_version {
            use version_compare::*;
            let package_version = env!("CARGO_PKG_VERSION");
    
            // less then
            if let Cmp::Lt = compare(package_version,&release_version).unwrap() {
                let message = format!(
                    "You are using an older version ({package_version}) of the bot. Download the newest version ({release_version}) from https://github.com/Deaths-Door/reddit-motion/releases/latest"
                ).bright_red();
                println!("{message}");
            }
        }
    }

    Ok(())
}

// TODO : Format all printlns with colored crate
pub async fn create_ffmpeg<'a>() -> Result<ffmpeg::FFmpeg,ffmpeg::FFmpegInstallError> {
    let mut ffmpeg = ffmpeg::FFmpeg::new();

    ffmpeg.check_and_install(
        "ffmpeg-6.0",
        || {
            println!("FFmpeg is not installed on this system.");

            println!("We can try to automatically install it for you. Would you like to do that? (y/n):");
    
            use std::io::*;
            let mut input = String::new();
    
            stdin().read_line(&mut input).expect("Error reading input");
    
            if input.to_lowercase() == "n" {
                println!("Please install FFmpeg manually and try again.");
                std::process::exit(0);
            }    
    
            println!("Downloading FFmpeg...");
        },
    ).await?;

    println!("FFmpeg is availiable!");

    Ok(ffmpeg)
}


use crate::config::Config;
use indicatif::ProgressBar;

pub async fn download_assets(config : &mut Config,ffmpeg : &FFmpeg) -> anyhow::Result<()> {
    let progress_bar = ProgressBar::new(config.assets.count() as u64);

    config.assets.download(ffmpeg,||{
        progress_bar.inc(1);
        println!("Downloaded another asset...");
    }).await?;
    
    println!("Assets are availiable!");

    Ok(())
}

pub fn create_callback() -> Callback {
    Callback {
        on_new_subreddit : |subreddit| println!("Checking {} subreddit...",subreddit.name),
        on_end_subreddit : || println!("Finished with subreddit!"),
        info : |submission| {
            println!("Video will be {} 👍",submission.title);
            println!("Thread url is https://reddit.com{} 👍",submission.permalink);
            println!("Thread has a upvote ratio of {}%",submission.upvote_ratio);
        },
        skipping_post : |error| eprintln!("{error}"),
        dimesions_out_of_bounds : |cd,vd|{
            eprintln!("Dimesions set {}x{} , are bigger then video dimensions {}x{}.\nHence we are not cropping the video the the specificed dimesions",cd.width,cd.height,vd.width,vd.height);
        }
    }
}