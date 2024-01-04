mod config;
mod command;
mod utils;
mod localize;
mod db;
mod ffmpeg;

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    command::Command::parse().exceute().await
}