mod crop;
mod utils;
mod concat;
mod split;

use super::VideoGenerator;

impl VideoGenerator {
    // returns output video path
    pub async fn exceute(self) -> std::io::Result<String> {        
        /*let bin_directory = self.video_gen_files.storage_directory.to_str().unwrap();

        // So isntead of bin/.. do to generated_videos/..
        let output_directory = bin_directory.replace("bin", "generated_videos");

        let video_directory = self.crop_and_move(bin_directory)?;
 
        let (title_segment,title_duration) = self.title_segment(&bin_directory,&video_directory)?;
        let mut current_duration = title_duration;

        // Skip 1 as we concat the title segment which is the first element 
        let iter = self.video_gen_files.files.iter()
            .skip(1)
            .enumerate();

        let reset_file = || Self::create_concat_file(bin_directory.clone(),&title_segment);
        // Since first index is 0
        let mut file = reset_file()?;

        for (index,(audio_directory,png_directory)) in iter {
            let (segment_path,segment_duration) = concat::concat_media_files(
                index + 1,
                &mut current_duration,
                &self.ffmpeg,
                bin_directory.clone(),
                &video_directory, 
                &audio_directory, 
                &png_directory
            )?;

            let next_duration = current_duration + segment_duration;

            current_duration = match next_duration >= self.video_length_limit as f64 {
                false => {
                    println!("{index} = false b4");
                    // Write to the file which contains the videos that should be concated 
                    Self::write_segment(&mut file, &segment_path)?;
                    println!("{index} = false after");
                    next_duration
                },
                true => {
                    println!("{index} = YES");
                   // TODO // spilt segment_path into videolimit + others
                    // then others in chucks of videolimit + write to file
                    // conacnt start and others

                    // Create Video 
                    self.create_final_video(&bin_directory,&output_directory)?;
    
                    // TODO : MAKE THIS WORK IN PARALELL?
                    // Now 'redefine' the file , so content is overwritten and it the future this work can be done in paraell
                    // but then create a different file name (using index)
                    file = reset_file()?;

                    // Incase the generated video is multiple times larger then the limit update it correctly
                    title_duration + current_duration % segment_duration
                },
            }
        }
            
        self.cleanup()?;
        println!("CLEANING UP");
        Ok(output_directory)*/

        let bin_directory = self.video_gen_files.storage_directory.display().to_string();

        // So isntead of bin/.. do to generated_videos/..
        let output_directory = bin_directory.replace("bin", "generated_videos");

        let video_directory = self.crop_and_move(&bin_directory)?;
 
        let (title_segment,title_duration) = self.title_segment(&bin_directory,&video_directory)?;
        let mut current_duration = title_duration;

        // Since first index is 0
        let mut concat_file = Self::create_concat_file(&bin_directory,&title_segment)?;

        let mut wrote_segment_on_last_iter = false;

        // Skip 1 as we concat the title segment which is the first element 
        let mut iter = self.video_gen_files.files.iter()
            .skip(1)
            .enumerate();

        for (index,(audio_directory,png_directory)) in iter {
            let (segment_path,segment_duration) = concat::concat_media_files(
                index+1,
                &current_duration,
                &self.ffmpeg,
                &bin_directory,
                &video_directory, 
                &audio_directory, 
                &png_directory
            )?;

            let expected_next_duration = current_duration + segment_duration;

            wrote_segment_on_last_iter = expected_next_duration < self.video_length_limit as f64;

            match wrote_segment_on_last_iter {
                true => {
                    // Write to the file which contains the videos that should be concated 
                    Self::write_segment(&mut concat_file, &segment_path)?;
                },
                false => {
                    // TODO spilt segment_path into videolimit + others
                    // then others in chucks of videolimit + write to file
                    // conacnt start and others

                    // Create Video 
                    self.create_final_video(&bin_directory,&output_directory)?;

                    // Now 'redefine' the file , so content is overwritten 
                    concat_file = Self::create_concat_file(&bin_directory,&title_segment)?;
                },
            };

            // TOD2O : UPDATE CURRENT_DURATION
        }

        // Create video from any other videos, incase video.limit is not reached and some videos are left
        if wrote_segment_on_last_iter {
            self.create_final_video(&bin_directory,&output_directory)?;
        }

        // TODO : Causing access dieneid for some weirdass reason
      // self.cleanup()?;


        Ok(output_directory)
    }
}