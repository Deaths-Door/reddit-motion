use thiserror::Error;

#[derive(Error,Debug)]
pub enum AssetsError {
    #[error("Failed to create asset directory : {}",.0)]
    CreatingDirectory(#[from] std::io::Error),

    #[error("Failed to find video stream , maybe check for internet connection")]
    VideoStreamUnavailable,

    #[error("Failed to download video due to , {}",.0)]
    DownloadError(#[from] rustube::Error),

    #[error("Failed to join threads downloading the assets")]
    ErrorJoiningThreads
}