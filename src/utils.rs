use std::path::PathBuf;

use indicatif::ProgressBar;
use maplit::{convert_args, hashmap};
use mime::Name;
use unic_langid::LanguageIdentifier;
use colored::Colorize;
use crate::{localize::{lookup, lookup_args, lookup1}, db::Database, ffmpeg::FFmpeg, config::Assets};

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

    let thanks = lookup(lang, "thanks")?.bold();

    let proccess = |id ,arg_id , value| -> anyhow::Result<String> {
        let _string = lookup1(lang, id,arg_id,value)?;
        let (a,b) = _string.split_once(value).unwrap();
        Ok(format!("{}{}{}",a.green(),value.blue(),b.green()))
    };
    
    let questions = {
        const LINK : &str = "https://github.com/Deaths-Door/reddit-motion";
        proccess("questions","link",LINK)?
    };

    let solutions = {
        const LINK : &str = "https://docs.rs/reddit_motion";
        proccess("solutions","link",LINK)?
    };

    for i in [&*thanks,&questions,&solutions] {
        println!("{i}");
    }

    print_seperator();

    Ok(())
}

pub async fn check_and_install_latest_version(db : &mut Database,lang : &LanguageIdentifier) -> anyhow::Result<()> {
    use chrono::*;
    let last_checked = db.last_version_check;
    let now = Utc::now();
    
    let duration = now.signed_duration_since(last_checked);

    // Check Every Week Once
    if duration.num_weeks() < 1 {
        return Ok(())
    } 

    {
        use crates_io_api::AsyncClient;

        let client = AsyncClient::new(
            "reddit-motion",
            std::time::Duration::from_millis(1000)
        )?;
        
        let mut _crate = client.get_crate("reddit-motion").await?;
        
        let release_version = &_crate.versions.first_mut().unwrap().num;
    
        use version_compare::*;
        let package_version = env!("CARGO_PKG_VERSION");
    
        // less then
        if let Cmp::Lt = compare(package_version,&release_version).unwrap() {
            const LINK : &str = "https://crates.io/crates/reddit-motion";
            let string = lookup_args(lang, "using-old-version",&convert_args!(hashmap!(
                "package_version" => package_version,
                "release_version" => &**release_version,
                "link" => LINK
            )))?;

            let (start_old_v,__end_old_v) = string.split_once(package_version).unwrap();
            let (start_new_v,__end_new_v) = __end_old_v.split_once(release_version).unwrap();
            let (start_end,end_end) = __end_new_v.split_once(LINK).unwrap();

            println!(
                "{}{}{}{}{}{}{}",
                start_old_v.red(),
                package_version.yellow(),
                start_new_v.red(),
                release_version.yellow(),
                start_end.red(),
                LINK.blue(),
                end_end.red()
            );

            print_seperator()
        }
    }
    
    db.last_version_check = now;

    Ok(())
}

pub async fn create_ffmpeg<'a>(lang : &LanguageIdentifier) -> anyhow::Result<FFmpeg> {
    let mut ffmpeg = FFmpeg::new();
    let local_path = "ffmpeg-6.0";

    fn end(ffmpeg : FFmpeg,lang : &LanguageIdentifier) -> anyhow::Result<FFmpeg> {
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

pub async fn download_assets(assets : &mut Assets,lang : &LanguageIdentifier) -> anyhow::Result<()> {
    assets.on_empty_assets(||{
        let s = lookup(lang, "assets.empty")?.bright_red();
        println!("{s}");
        Ok(())
    })?;

    let progress_bar = ProgressBar::new(assets.count() as u64);

    let downloading = lookup(lang, "assets.downloading")?.bold();
    let on_each_download =|| {
        progress_bar.inc(1);
        println!("{downloading}");
    };

    const FILE : &str = "FILE";
    const ACTUAL : &str = "ACTUAL";
    const EXPECTED : &str = "EXPECTED";

    // Need to get it outside the function so putting default values
    let wrong_mime = lookup_args(lang, "assets.wrong-mime", &convert_args!(hashmap!(
        "file" => FILE,
        "actual" => ACTUAL,
        "expected" => EXPECTED
    )))?;

    let warn_wrong_mime = |path: PathBuf,actual_mime : &Name<'_>,expected_mime : &Name<'_>|{
        let s = wrong_mime.replace(FILE, &path.display().to_string())
            .replace(ACTUAL,actual_mime.as_str())
            .replace(EXPECTED, expected_mime.as_str())
            .bright_yellow();
        println!("{s}")
    };

    assets.process_and_download(warn_wrong_mime,on_each_download).await?;

    progress_bar.finish_and_clear();
    println!("{}",lookup(lang, "assets")?.green());
    print_seperator();
    Ok(())
}