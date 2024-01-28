use crate::{ffmpeg::FFmpeg, video_generator::VideoGenerator};

pub(crate) fn get_duration_str<'a,T>(ffmpeg : &FFmpeg,file_path : &str,map : impl FnOnce(&str) -> std::io::Result<T>) -> std::io::Result<T> {
    // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1 fifa.mp4
    let output = ffmpeg.ffprobe_expect_failure(|cmd|{
        cmd.args([
            "-v" , "error",
            "-show_entries" , "format=duration",
            "-of" , "default=noprint_wrappers=1",
            "-i" , file_path,
        ]);
    })?;

    map(
        String::from_utf8(output.stdout)
            .unwrap()
            .strip_prefix("duration=")
            .unwrap()
            .trim_end()
    )
}

pub(crate) fn get_duration(ffmpeg : &FFmpeg,file_path : &str) -> std::io::Result<f64> {
    get_duration_str(ffmpeg, file_path, |v| Ok(v.parse().unwrap()))
}

impl VideoGenerator {
    pub(super) fn cleanup(&self,bin_directory : String) -> std::io::Result<()> {
        std::fs::create_dir_all(bin_directory)
    }
} 