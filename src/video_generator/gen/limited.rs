use std::ops::Deref;

use mime_guess::Iter;

use crate::video_generator::{self, VideoGenerator};

use super::{shared::SharedGeneratorLogic, utils::get_duration};

pub struct LimitedVideoLength<'a> {
    shared_generator : SharedGeneratorLogic<'a>,
    video_generator : &VideoGenerator,
    limit: f64,
}

impl Deref for LimitedVideoLength<'_> {
    type Target = VideoGenerator;
    fn deref(&self) -> &Self::Target {
        self.video_generator
    }
}

/*
struct TitleSegment<'a> {
    file : (&'a String,&'a String),
    duration : f64
}

impl<'a> TitleSegment<'a> {
    fn new(iter : &mut Iter<'a,(String,String)>) -> Self {
        let file = iter.next().unwrap();

    }
}*/

impl<'a> LimitedVideoLength<'a> {
    pub const fn new(shared_generator: SharedGeneratorLogic<'a>,video_generator : &VideoGenerator,limit: f64) -> Self { 
        Self { shared_generator, video_generator , limit } 
    }

    pub fn exceute(mut self,bin_directory : &str,output_directory : &str) -> std::io::Result<Vec<String>> {
        // TODO : ADD TITLE SEGMENT TO IT
        // TODO : ADD IMAGES TO IT\
        let mut iter = self.video_gen_files.files.iter();
        let mut append_title = || Ok(());
        /*let (title_audio,title_image) = iter.next().unwrap();
        let title_audio_duration : f64 = get_duration(&self.ffmpeg, &title_audio)?;
        let mut append_title = || {
            self.shared_generator.append_audio(&title_audio)?;
            self.shared_generator.append_image(title_audio_duration)
        };*/

   
        loop {
            match iter.next() {
                None => break,
                Some((audio_file,image_file)) => {
                    let duration = get_duration(&self.ffmpeg, &audio_file)?;

                    match duration.partial_cmp(&self.limit) {
                        None => unreachable!(),
                        Some(ordering) if ordering.is_gt() => self.keep_media_within_bounds(&audio_file,duration,append_title),
                        Some(ordering) => {
                            if ordering.is_eq() {
                                append_title()?;
                            }
        
                            self.shared_generator.append_audio(audio_file)?;
                            self.shared_generator.append_image(duration)
                        }
                    }?;
                }
            }

        }
      //  for (audio_file,image_file) in iter {
            
       //}

        let output_video = self.shared_generator.finalize_video(bin_directory, output_directory, |cmd|{
            // TODO : ADD THIS
        })?;

        self.split_video(output_video, output_directory)
    }

    fn number_of_videos_created(&self,duration : f64) -> u64 {
        (duration % self.limit).round() as u64 + 1
    }

    fn keep_media_within_bounds(&mut self,audio_file : &str,duration : f64,mut append_title : impl FnMut() -> std::io::Result<()>) -> std::io::Result<()> {
        // This would be the same as the audio index
        let current_seconds = &mut self.shared_generator.image_start_position;
        let count = self.number_of_videos_created(duration) - 1;

        // Basically go till n-1 as those are whole numbers
        // n is a remainder
        for _ in 0..count {
            append_title()?;

            self.shared_generator.append_audio(audio_file)?;

            let outpoint = *current_seconds + self.limit;
            self.shared_generator.append_audio_point(&current_seconds, &outpoint)?;
            *current_seconds = outpoint;

            self.shared_generator.append_image(self.limit)?;
        }

        // Just continue to the end
        self.shared_generator.append_audio(audio_file)?;
        self.shared_generator.append_audio_inpoint(&current_seconds)?;

        let offset = match count {
            0 => duration,
            _ => duration % self.limit
        };

        self.shared_generator.append_image(offset)
    }

    fn split_video(&self,video_path : String,output_directory : &str) -> std::io::Result<Vec<String>> {
        let duration = get_duration(&self.shared_generator.ffmpeg, &video_path)?;

        if duration < self.limit {
            return Ok(vec![video_path])
        }

        let output_placeholder = format!("{output_directory}/final_video_%03d.mp4");

        // ffmpeg -i "C:\Users\Aarav Aditya Shah\Music\Alan Walker - The Spectre.mp3" -f segment -segment_time 30 -vf "crop=200:200" output_%03d.mp3
        self.shared_generator.ffmpeg.ffmpeg_expect_failure(|cmd|{
            cmd.args([
                "-i" , &video_path,
                "-f" , "segment",
                "-segment_time" , &self.limit.to_string(),
                &output_placeholder
            ]);
        })?;

        std::fs::remove_file(video_path)?;

        // Find number of videos created
        let count = self.number_of_videos_created(duration);

        let mut videos = vec![];
        
        for i in 0..count {
            videos.push(format!("{output_directory}/final_video_{:03}.mp4",i))
        }

        Ok(videos)
    }
}