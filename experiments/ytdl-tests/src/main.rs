use rusty_ytdl::*;

#[tokio::main]
async fn main() {
  let video_url = "https://www.youtube.com/watch?v=FZ8BxMU3BYc"; // FZ8BxMU3BYc works too!
  let video = Video::new(video_url).unwrap();

  // Or direct download to path
  let path = std::path::Path::new(r"test.mp3");

  video.download(path).await.unwrap();
}