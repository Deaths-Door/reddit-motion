use std::{
    path::Path,
    fs::create_dir_all,
};

use indicatif::ProgressBar;

use rustube::{
    Video,
    Id,
    Callback,
    Stream
};

use futures_util::future::{BoxFuture,FutureExt};

use tokio::{
    spawn,
    try_join
};

use crate::AssetsError;

const VIDEOS_DIR : &str = "assets/videos";

pub(in crate) async fn download_background_videos() -> Result<(),AssetsError> {
    println!("Checking for assets else downloding them..");

    let asset_vid = Path::new(VIDEOS_DIR);

    if !asset_vid.exists() {
        create_dir_all(VIDEOS_DIR)?
    }

    let mc = spawn(check_download_vid_with_id("Pt5_GSKIWQM"));
    let subway_surfer = spawn(check_download_vid_with_id("VwJaIa_Eyds"));
    let glass_falling = spawn(check_download_vid_with_id("iXlI8hhiP6I"));

    try_join!(mc,subway_surfer,glass_falling);

    println!("Finished checking for assets!");
    Ok(())
}


async fn check_download_vid_with_id(id : &str) -> Result<(),AssetsError> {
    let _dir = format!("{VIDEOS_DIR}/{id}.mp4");
    let path = Path::new(&_dir);
        
    match path.exists()  {
        true => Ok(()),
        false => {
            let _id = Id::from_str(id).unwrap();
            let _video = Video::from_id(_id.into_owned()).await.unwrap();
            
            match _video.best_video() {
                None => Err(AssetsError::VideoStreamUnavailable),
                Some(stream) => {
                    let _content_length = stream.content_length().await.unwrap();
                    let progress_bar = ProgressBar::new(_content_length);

                    let mut callback = {
                        let _progress_bar = progress_bar.clone();

                        Callback::new()
                            .connect_on_progress_closure_slow(move |arg| _progress_bar.inc(arg.current_chunk as u64)) 
                            .connect_on_complete_closure(move |_| progress_bar.finish() )
                    };

                    stream.download_to_dir_with_callback(VIDEOS_DIR,callback).await.map(|_| Ok(()))?
                }
            }
        }
    }
}

//downloader(stream,stream.content_length().await.unwrap())/*{
                   // let _content_length = stream.content_length().await.unwrap();
                 //   create_progress_bar(_content_length);
                    /*let callback = Callback::new();

                    stream.download_to_dir_with_callback(VIDEOS_DIR,callback).await.map(|_|{
                        Ok(())
                    })?*/
                  /*
                    let callback = Callback::new().connect_on_progress_closure_slow(|arg| progress_bar().inc(arg.current_chunk as u64));
                    stream.download_to_dir(VIDEOS_DIR).await.map(|_| {
                      //  progress_bar.finish();
                        Ok(())
                    })?*/