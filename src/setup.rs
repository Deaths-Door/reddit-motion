use std::{
    path::Path,
    fs::create_dir_all
};
    
use rustube::{Video,Id};

use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};

/// Downloads background videos 
/// * Minecraft Parkour
/// * Subway Surfers
/// * Glass falling down stairs
pub(in crate) async fn download_background_videos() {
    
    const VIDEOS_DIR : &str = "assets/videos";
    let path = Path::new(VIDEOS_DIR);

    if !path.exists() {
        create_dir_all(VIDEOS_DIR);
    }

    println!("Downloading background videos..");

    let video_from_id = |id : &str|{
        let id = Id::from_str(id).unwrap();
        Video::from_id(id.into_owned()).unwrap()
    };

    let mc = video_from_id("Pt5_GSKIWQM").fuse();
    let subway_surfer = video_from_id("VwJaIa_Eyds").fuse();
    let glass_falling = video_from_id("iXlI8hhiP6I").fuse();

    pin_mut!(mc,subway_surfer,glass_falling);

    select! {
        s1 = mc => {
            sq.best_quality().unwrap().download_to_dir(VIDEOS_DIR).await;
            println!("Downloaded ")
        }
    }

//    pin_mut!(mc,subway_surfer,glass_falling);

   /* join!(
        select!(
            result = mc => result.unwrap().best_quality().unwrap().download_to_dir(VIDEOS_DIR),
            result = subway_surfer => result.unwrap().best_quality().unwrap().download_to_dir(VIDEOS_DIR),
            result = glass_falling => result.unwrap().best_quality().unwrap().download_to_dir(VIDEOS_DIR),
        )    
    )
*/
    

    //let mc = Video::from_id("Pt5_GSKIWQM".into_owned()).best_quality().unwrap().download_to_dir(VIDEOS_DIR);
    //let subway_surfer = Video::from_id("VwJaIa_Eyds".into_owned()).best_quality().unwrap().download_to_dir(VIDEOS_DIR);
   // let glass_falling = Video::from_id("iXlI8hhiP6I".into_owned()).best_quality().unwrap().download_to_dir(VIDEOS_DIR);



}
