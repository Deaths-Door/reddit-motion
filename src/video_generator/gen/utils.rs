use std::fs::File;

use rand::Rng;

use crate::{ffmpeg::FFmpeg, video_generator::{VideoGenerator, data::if_path_exists}};

pub(super) fn random_start_point(ffmpeg : &FFmpeg,file_path : &str) -> std::io::Result<f64> {
    let duration = get_duration(ffmpeg, file_path)?;
    let value = rand::thread_rng().gen_range(0f64..duration);
    Ok(value)
}

pub(super) fn get_duration(ffmpeg : &FFmpeg,file_path : &str) -> std::io::Result<f64> {
    // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1 fifa.mp4
    ffmpeg.ffprobe_expect_failure_map(|cmd|{
        cmd.args([
            "-v" , "error",
            "-show_entries" , "format=duration",
            "-of" , "default=noprint_wrappers=1",
            "-i" , file_path,
        ]);
    },|o| {
        String::from_utf8(o.stdout)
            .unwrap()
            .strip_prefix("duration=")
            .unwrap()
            .trim_end()
            .parse()
            .unwrap()
    })
}

impl VideoGenerator {
    pub(super) fn cleanup(self) -> std::io::Result<()> {
        std::fs::remove_dir_all(&self.video_gen_files.storage_directory)
    }
    
    pub(super) fn create_concat_file(mut concat_file : String,segment_path : &str) -> std::io::Result<File> {  
        concat_file.push_str("/concat.txt");

        let mut file = File::create(concat_file)?;
        Self::write_segment(&mut file,segment_path)?;
        Ok(file)
    }

    pub(super) fn write_segment(file : &mut File,segment_path : &str) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(file,"file \"{segment_path}\"")
    }

    pub(super) fn title_segment(
        &self,
        bin_directory : &str,
        video_directory : &str,
    )  -> std::io::Result<(String,f64)> {
        let current_position = random_start_point(&self.ffmpeg,&video_directory)?;

        let (audio_directory,png_directory) = self.video_gen_files.files.first().unwrap();

        super::concat::concat_media_files(
            0,
            &current_position,
            &self.ffmpeg,
            bin_directory.to_owned(),
            &video_directory, 
            &audio_directory, 
            &png_directory
        )
    }

    pub(super) fn create_final_video(&self,
        _temp_directory : &str,
        final_output_directory : &str
    ) -> std::io::Result<()> {
        let temp_directory= format!("{_temp_directory}/final_temp.mp4");
        let concat_file= format!("{_temp_directory}/concat.txt");

        if_path_exists!(not &temp_directory,{
            super::concat::concat_for_mp4s(&self.ffmpeg, &concat_file, &temp_directory)?;
        });

        std::fs::create_dir_all(&final_output_directory)?;
        
        add_background_music(
            &self.ffmpeg, 
            &self.audio_asset_directory,
            &temp_directory,
            final_output_directory
        )
    }
}


fn add_background_music(
    ffmpeg : &FFmpeg,
    mp3_file : &str,
    mp4_file : &str,
    output_directory : &str
) -> std::io::Result<()> {
    /*
    ffmpeg 
    -i "C:\Users\Aarav Aditya Shah\Downloads\input.mp4" 
    -i "C:\Users\Aarav Aditya Shah\Music\Alesso - When I’m Gone (with Katy Perry).mp3" 
    -c:v copy  
    -filter_complex "[0:a]aformat=fltp:44100:stereo,apad[0a];[1]aformat=fltp:44100:stereo,volume=1.5[1a];[0a][1a]amerge[a]" 
    -map 0:v -map "[a]" 
    -ac 2 
    "output.mp4" */
    ffmpeg.ffmpeg_expect_failure(|cmd|{
        cmd.args([
            "-i" , mp4_file,
            "-i" , mp3_file,
            "-c:v" , "copy",
            "-filter-complex", 
            "[0:a]aformat=fltp:44100:stereo,apad[0a];[1]aformat=fltp:44100:stereo,volume=0.6[1a];[0a][1a]amerge[a]",
            "-map" , "0:v",
            "-map" , "[a]",
            "-ac" , "2",
            output_directory
        ]);
    })
}