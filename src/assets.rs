use std::{
    path::Path,
    fs::create_dir_all
};

use indicatif::ProgressBar;

use rustube::{
    tokio::{
        spawn,
        join
    },
    Video,
    Id,
    Callback
};

pub(in crate) async fn download_background_videos() {
    println!("Checking for assets else downloding them..");

    const VIDEOS_DIR : &str = "assets/videos";
    let asset_vid = Path::new(VIDEOS_DIR);

    if !asset_vid.exists() {
        create_dir_all(VIDEOS_DIR);
    }

    let progress_bar = ProgressBar::new(3);

    let check_download_vid_with_id = |id : &str| async {
        let _dir = format!("{VIDEOS_DIR}/{id}.mp4");
        let path = Path::new(&_dir);
        
        match path.exists() {
            true => progress_bar.inc(1),
            false => {
                let _id = Id::from_str(id).unwrap();
                let _video = Video::from_id(_id.into_owned()).await.unwrap();
                let stream = _video.best_video().unwrap();

                let callback = Callback::new().connect_on_complete_closure(|_| progress_bar.inc(1) );

                stream.download_to_dir_with_callback(VIDEOS_DIR,callback).await;
            }
        }
    };

    let mc = spawn(check_download_vid_with_id("Pt5_GSKIWQM"));
    let subway_surfer = spawn(check_download_vid_with_id("VwJaIa_Eyds"));
    let glass_falling = spawn(check_download_vid_with_id("iXlI8hhiP6I"));

    join!(mc,subway_surfer,glass_falling);

    progress_bar.finish();
    println!("Finished checking for assets!")
}