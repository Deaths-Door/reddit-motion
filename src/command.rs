use std::{path::Path, fs::File};
use clap::Parser;
use crate::{config::Config, utils, db::Database};

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Command {
    #[arg(short,long,action)]
    edit : bool
}

const CONFIG_PATH : &str = "config.toml";

impl Command {
    pub async fn exceute(self) -> anyhow::Result<()> {
        match self.edit {
            true => Ok(Self::exceute_edit()?),
            false => Self::exceute_create_videos().await,
        }
    }

    fn exceute_edit() -> std::io::Result<()> {
        let path : &Path = CONFIG_PATH.as_ref();
        if !path.exists() {
            let _ = File::create(path)?;
        }

        open::that(CONFIG_PATH)
    }

    async fn exceute_create_videos() -> anyhow::Result<()> {
        let mut config = Config::from_file(CONFIG_PATH)?;
        utils::print_banner(&config.lang);

        let mut db = Database::from_file_or_create("db.toml")?;  
        utils::check_and_install_latest_version(&mut db,&config.lang).await?;
        utils::download_assets(&mut config.assets, &config.lang).await?;

        let ffmpeg = utils::create_ffmpeg(&config.lang).await?;

        db.update_database()?;
        Ok(())
    }
}