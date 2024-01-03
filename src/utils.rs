use crate::callback::Callback;
use crate::db::Database;
use crate::ffmpeg::{self, FFmpeg};
use crate::localize::lookup;
use colored::Colorize;

lazy_static::lazy_static! {
    static ref SEPERATOR : String = "=".repeat(20);
}

fn print_seperator() {
    println!("{}",*SEPERATOR);
}

pub fn print_banner(lang : &LanguageIdentifier) -> anyhow::Result<()> {
    let banner = r#"
    ____           __    ___ __     __  ___      __  _           
   / __ \___  ____/ /___/ (_) /_   /  |/  /___  / /_(_)___  ____ 
  / /_/ / _ \/ __  / __  / / __/  / /|_/ / __ \/ __/ / __ \/ __ \
 / _, _/  __/ /_/ / /_/ / / /_   / /  / / /_/ / /_/ / /_/ / / / /
/_/ |_|\___/\__,_/\__,_/_/\__/  /_/  /_/\____/\__/_/\____/_/ /_/ 
"#.bright_red();

    println!("{banner}");

    const LINK_PLACEHOLDER : &str = "|-|";

    let thanks = lookup(lang, "thanks")?.bold();

    let proccess = |id , link : &str| -> anyhow::Result<String> {
        let _string = lookup(lang, id)?;
        let (a,b) = _string.split_once(LINK_PLACEHOLDER).unwrap();
        Ok(format!("{}{}{}",a.green(),link.blue(),b.green()))
    };
    
    let questions = {
        const LINK : &str = "https://github.com/Deaths-Door/reddit-motion";
        proccess("questions",LINK)?
    };

    let solutions = {
        const LINK : &str = "https://docs.rs/reddit_motion";
        proccess("solutions",LINK)?
    };

    for i in [&*thanks,&questions,&solutions] {
        println!("{i}");
    }

    print_seperator();

    Ok(())
}
use unic_langid::LanguageIdentifier;

pub async fn check_and_install_latest_version(db : &mut Database) -> anyhow::Result<()> {
    use chrono::*;
    // Check Every Week Once
    let last_checked = db.last_version_check;
    let now = Utc::now();
    
    let duration = now.signed_duration_since(last_checked);

    if duration.num_weeks() < 1 {
        return Ok(())
    } 

    {
        use crates_io_api::AsyncClient;

        let client = AsyncClient::new(
            "reddit-motion",
            std::time::Duration::from_millis(1000)
        )?;
        
        let _crate = client.get_crate("reddit-motion").await?;
        
        let release_version = &_crate.versions.first().unwrap().num;
    
        use version_compare::*;
        let package_version = env!("CARGO_PKG_VERSION");
    
        // less then
        if let Cmp::Lt = compare(package_version,release_version).unwrap() {
            let message = format!(
                "You are using an older version ({package_version}) of the bot. Download the newest version ({release_version}) from https://github.com/Deaths-Door/reddit-motion/releases/latest"
            ).bright_red();
            println!("{message}");
        }
    }
    
    db.last_version_check = now;

    Ok(())
}

// TODO : Format all printlns with colored crate
pub async fn create_ffmpeg<'a>(lang : &LanguageIdentifier) -> anyhow::Result<ffmpeg::FFmpeg> {
    let mut ffmpeg = ffmpeg::FFmpeg::new();
    let local_path = "ffmpeg-6.0";

    fn end(ffmpeg : FFmpeg,lang : &LanguageIdentifier) -> anyhow::Result<ffmpeg::FFmpeg> {
        println!("{}",lookup(lang, "ffmpeg")?.green());

        print_seperator();
    
        Ok(ffmpeg)
    }

    if ffmpeg.check_if_installed(local_path).is_some_and(|v| v) {
        return end(ffmpeg,lang);
    }

    println!("{}",lookup(lang, "ffmpeg.not_installed")?.bright_red());

    println!("{}",lookup(lang, "ffmpeg.auto_download")?);

    use std::io::*;
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Error reading input");

    if input.to_lowercase() == "n" {
        println!("{}",lookup(lang,"ffmpeg.manually")?);
        std::process::exit(0);
    }    

    println!("{}",lookup(lang, "ffmpeg.downloading")?.yellow());

    ffmpeg.install(local_path).await?;

    end(ffmpeg,lang)
}


use crate::config::Config;
use indicatif::ProgressBar;

pub async fn download_assets(config : &mut Config,ffmpeg : &FFmpeg) -> anyhow::Result<()> {
    let progress_bar = ProgressBar::new(config.assets.count() as u64);
    let lang = &config.lang;

    let s = lookup(lang, "assets.downloading")?;
    config.assets.download(ffmpeg,||{
        progress_bar.inc(1);
        println!("{s}");
    }).await?;
    
    println!("{}",lookup(lang, "assets")?.green());

    print_seperator();

    Ok(())
}

// TODO : Trasnlate this as wel
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