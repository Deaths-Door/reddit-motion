use std::slice::Iter;

use crate::{ffmpeg::FFmpeg, video_generator::VideoGenerator};

use super::{shared::SharedGeneratorLogic, utils::get_duration};

pub struct LimitedVideoLength<'a> {
    shared_generator : SharedGeneratorLogic,
    images : Vec<&'a str>,
    limit: f64,
}


struct TitleSegment<'a> {
    file : (&'a String,&'a String),
    duration : f64
}

impl<'a> TitleSegment<'a> {
    fn new(ffmpeg : &FFmpeg,iter : &mut Iter<'a,(String,String)>) -> std::io::Result<Self> {
        let (title_audio,title_image) = iter.next().unwrap();
        let duration : f64 = get_duration(ffmpeg, title_audio)?;

        Ok(Self { file : (title_audio,title_image) , duration })
    }

    fn append_to(&'a self,limited_video_length : &mut LimitedVideoLength<'a>) -> std::io::Result<()> {
        limited_video_length.shared_generator.append_audio(self.file.0)?;
        limited_video_length.append_image(self.file.1, self.duration); 
        Ok(())
    }    
}

impl<'main> LimitedVideoLength<'main> {
    pub const fn new(
        shared_generator: SharedGeneratorLogic,
        limit: f64
    ) -> Self { 
        Self { shared_generator , limit , images : Vec::new() } 
    }

    pub fn append_image(&mut self,image_file : &'main str,offset_by : f64){
        self.images.push(image_file);
        self.shared_generator.append_image(offset_by)
    }

    pub fn exceute<'sec>(
        self,        
        video_generator : &'sec VideoGenerator,
        bin_directory : &str,
        output_directory : &str
    ) -> std::io::Result<Vec<String>> {
        fn inner<'a : 'l,'l,'t : 'l>(
            mut limited_video_length: LimitedVideoLength<'l>,
            video_generator : &'a VideoGenerator,
            title_segment : &'t TitleSegment<'_>,
            iter : &mut Iter<'l,(String,String)>,
            bin_directory : &str,
            output_directory : &str
        ) -> std::io::Result<Vec<String>> {
            for (audio_file,image_file) in iter {
                let duration = get_duration(&video_generator.ffmpeg, &audio_file)?;
    
                match duration.partial_cmp(&limited_video_length.limit) {
                    None => unreachable!(),
                    Some(ordering) if ordering.is_gt() => limited_video_length.keep_media_within_bounds(&audio_file,&image_file,duration,&title_segment)?,
                    Some(ordering) => {
                        if ordering.is_eq() {
                            title_segment.append_to(&mut limited_video_length)?;
                        }
    
                        limited_video_length.shared_generator.append_audio(audio_file)?;
                        limited_video_length.shared_generator.append_image(duration)
                    }
                };
            }
            
            // So it does not clash with other generated files 
            let bin_directory = format!("{bin_directory}/limited");
            
            std::fs::create_dir_all(&bin_directory)?;

            let output_video = limited_video_length.shared_generator.finalize_video(video_generator,&bin_directory, output_directory, |cmd|{
                for i in &limited_video_length.images {
                    cmd.args(["-i",i]);
                }
            })?;
    
            limited_video_length.split_video(video_generator,output_video, output_directory)
        }

        let mut iter = video_generator.video_gen_files.files.iter();
        let title_segment = TitleSegment::new(&video_generator.ffmpeg, &mut iter)?;
    
        inner(self,video_generator,&title_segment,&mut iter,bin_directory,output_directory)
    }

    fn number_of_videos_created(&self,duration : f64) -> u64 {
        (duration % self.limit).round() as u64 + 1
    }

    const fn current_seconds(&self) -> &f64 {
        &self.shared_generator.image_start_position
    }

    fn keep_media_within_bounds<'t : 'main>(
        &mut self,
        audio_file : &str,
        image_file : &'main str,
        duration : f64,
        title_segment : &'main TitleSegment<'_>
    ) -> std::io::Result<()> {
        let count = self.number_of_videos_created(duration) - 1;

        // Basically go till n-1 as those are whole numbers
        // n is a remainder
        for _ in 0..count {
            title_segment.append_to(self)?;

            self.shared_generator.append_audio(audio_file)?;

            let outpoint = *self.current_seconds() + self.limit;

            let inpoint = self.current_seconds().clone();
            self.shared_generator.append_audio_point(&inpoint, &outpoint)?;
        
            self.append_image(image_file, self.limit)
        }

        // Just continue to the end
        self.shared_generator.append_audio(audio_file)?;

        let inpoint = self.current_seconds().clone();
        self.shared_generator.append_audio_inpoint(&inpoint)?;

        let offset = match count {
            0 => duration,
            _ => duration % self.limit
        };

        self.shared_generator.append_image(offset);

        Ok(())
    }

    fn split_video(
        &self,
        video_generator : &VideoGenerator,
        video_path : String,
        output_directory : &str
    ) -> std::io::Result<Vec<String>> {
        let duration = get_duration(&video_generator.ffmpeg, &video_path)?;

        if duration < self.limit {
            return Ok(vec![video_path])
        }

        let output_placeholder = format!("{output_directory}/final_video_%03d.mp4");

        // ffmpeg -i "C:\Users\Aarav Aditya Shah\Music\Alan Walker - The Spectre.mp3" -f segment -segment_time 30 -vf "crop=200:200" output_%03d.mp3
        video_generator.ffmpeg.ffmpeg_expect_failure(|cmd|{
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