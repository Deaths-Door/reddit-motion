use std::{
    path::Path,
    fs::create_dir_all
}

use rustube::Video;

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

    let mc = Video::from_id("Pt5_GSKIWQM").best_quality().unwrap().download_to_dir(VIDEOS_DIR);
    let subway_surfer = Video::from_id("VwJaIa_Eyds").best_quality().unwrap().download_to_dir(VIDEOS_DIR);
    let glass_falling = Video::from_id("iXlI8hhiP6I").best_quality().unwrap().download_to_dir(VIDEOS_DIR);



}