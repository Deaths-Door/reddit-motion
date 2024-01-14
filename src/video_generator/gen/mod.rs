mod crop;
mod utils;
mod concat;
mod split;

use super::VideoGenerator;

impl VideoGenerator {
    // returns output video path
    pub async fn exceute(self) -> std::io::Result<String> {        
        let bin_directory = self.video_gen_files.storage_directory.display()
            .to_string();

        // So isntead of bin/.. do to generated_videos/..
        let output_directory = bin_directory.replace("bin", "generated_videos");

        let video_directory = self.crop_and_move(bin_directory.clone())?;
 
        let (title_segment,title_duration) = self.title_segment(&bin_directory,&video_directory)?;
        let mut current_duration = title_duration;

        // Skip 1 as we concat the title segment which is the first element 
        let iter = self.video_gen_files.files.iter()
            .skip(1)
            .enumerate();

        let reset_file = || Self::create_concat_file(&title_segment);
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

            current_duration = match true {// next_duration >= self.video_length_limit as f64 {
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
                    self.create_final_video(bin_directory.clone(),&output_directory)?;
    
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

        Ok(output_directory)
    }
}