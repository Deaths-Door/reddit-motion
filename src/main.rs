mod utils;
mod ffmpeg;
mod config;
mod db;
mod callback;

use db::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils::print_banner();
    utils::check_and_install_latest_version().await?;

    let ffmpeg = utils::create_ffmpeg().await?;

    let mut config = config::Config::from_file("config.toml")?;
    utils::download_assets(&mut config, &ffmpeg).await?;

    let mut db = Database::from_file_or_create("db.toml")?;  
    let callback = utils::create_callback(); 

    println!("Reading reddit!!");

    let handler = config.create_videos(&mut db,ffmpeg,&callback).await?;

    db.update_database()?;
    
    handler.await?;
    Ok(())
}