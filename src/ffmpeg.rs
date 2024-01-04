use std::{process::{Command, Output}, path::Path};

/// The `FFmpeg` struct is used to manage the installation and usage of FFmpeg.
///
/// It provides methods for checking if FFmpeg is installed, installing FFmpeg, and using FFmpeg to encode or decode videos.
///
/// The `local_download_path` field is the path to the local directory where FFmpeg will be downloaded.
/// The `command` field is the command to run FFmpeg.
pub struct FFmpeg {
    ffmpeg_command : String,
    ffprobe_command : String
}

/// This enum represents errors that can occur when installing FFmpeg.
#[derive(thiserror::Error,Debug)]
pub enum FFmpegInstallError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[cfg(windows)]
    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[cfg(windows)]
    #[error(transparent)]
    ZipExtraction(#[from] zip_extract::ZipExtractError),
}

impl FFmpeg {
    /// Creates a new `FFmpeg` struct.
    ///
    /// # Returns
    /// A new `FFmpeg` struct.
    pub fn new() -> Self { 
        let ffmpeg_command = "ffmpeg".to_string();
        let ffprobe_command =  "ffprobe".to_string();
        Self { ffmpeg_command, ffprobe_command  }
    }

    /// Creates command from local path
    pub fn ffmpeg_command(&self) -> Command {
        Command::new(&(*self.ffmpeg_command))
    }

    /// Creates command from local path
    pub fn ffprobe_command(&self) -> Command {
        Command::new(&(*self.ffprobe_command))
    }

    pub fn ffmpeg_expect_failure(
        &self,
        builder : impl FnOnce(&mut Command) -> (),
    ) -> std::io::Result<()> { 
        self.expect_failure_map(self.ffmpeg_command(),builder,|_|())
    }

    pub fn ffprobe_expect_failure<T>(
        &self,
        builder : impl FnOnce(&mut Command) -> (),
        map_output : impl FnOnce(Output) -> T
    ) -> std::io::Result<()> {
        self.ffprobe_expect_failure_map(builder, |_| ())
    }

    pub fn ffprobe_expect_failure_map<T>(
        &self,
        builder : impl FnOnce(&mut Command) -> (),
        map_output : impl FnOnce(Output) -> T
    ) -> std::io::Result<T> { 
        self.expect_failure_map(self.ffprobe_command(),builder,map_output)
    }

    fn expect_failure_map<T>(
        &self,
        mut command : Command,
        builder : impl FnOnce(&mut Command) -> (),
        map_output : impl FnOnce(Output) -> T
    ) -> std::io::Result<T> {
        builder(&mut command);
        let output =  command.output()?;
        let status = output.status.success();

        println!("{:?}",command);
        if !status {
            eprintln!("stderr={}",String::from_utf8(output.stderr).unwrap());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "ffmpeg command failed"
            ))
        };

        Ok(map_output(output))
    }
}

impl FFmpeg {
    // Checks if FFmpeg is installed.
    ///
    /// # Returns
    /// `Some(true)` if FFmpeg is installed, or `Some(false)` if it is not installed.
    /// `None` when it is not possible to tell if its installed ()
    pub fn check_if_installed<P : AsRef<Path>>(&mut self,local_path : P) -> Option<bool> {
        let command_result = self.ffmpeg_command().spawn();

        if command_result.is_ok() {
            return Some(true)
        }

        #[cfg(windows)]{
            if local_path.as_ref().exists() {
                self.update_to_local_paths(local_path);
                return Some(true)
            }
        }

        let kind = command_result.unwrap_err().kind();

        if kind == std::io::ErrorKind::NotFound { Some(false) } else { None }
    }

    #[cfg(windows)]
    fn update_to_local_paths<P : AsRef<Path>>(&mut self,local_path : P) {
        self.ffmpeg_command = format!("{}/bin/ffmpeg.exe",local_path.as_ref().display());
        self.ffprobe_command = format!("{}/bin/ffprobe.exe",local_path.as_ref().display());
    }

    /// Installs FFmpeg.
    ///
    /// # Arguments
    /// `on_download_complete`: A callback function that is called after the FFmpeg download is complete.
    ///
    /// # Returns
    /// An `FFmpegInstallError` if the installation failed, or `Ok(())` if successful.
    pub async fn install<P : AsRef<Path>>(
        &mut self,
        local_path : P
    ) -> Result<(),FFmpegInstallError> {
        #[cfg(target_os = "macos")] {
            Command::new("brew install ffmpeg").spawn()?;
        }

        #[cfg(target_os = "linux")] {
            Command::new("sudo apt install ffmpeg").spawn()?;
        }

        #[cfg(windows)] {
            use reqwest::*;

            const URL : &str = "https://github.com/GyanD/codexffmpeg/releases/download/6.0/ffmpeg-6.0-full_build.zip";
        
            let response = get(URL).await?;
        
            use std::fs::*;
            use std::io::Cursor;

            create_dir_all(local_path.as_ref())?;

            let body = response.bytes().await?;
            let cursor = Cursor::new(body);

            zip_extract::extract(
                cursor,
                local_path.as_ref(),
                true
            )?;

            self.update_to_local_paths(local_path);
        }

        Ok(())
    }
}