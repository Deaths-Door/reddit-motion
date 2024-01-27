use std::{fs::File, io::Write, ops::Deref, path::Path, process::Command};

use crate::video_generator::VideoGenerator;

pub struct SharedGeneratorLogic<'a> {
    // TODO : REMOVE THIS causes errors
    pub(super) video_generator : &'a VideoGenerator,

    audio_concat_file : File,
    pub(super) audio_concat_file_path : String,

    /// The goal is to create this command 
    /// ffmpeg -i video -i image1 -i image2 -i image3
    ///  -filter_complex
    ///  "[0][1]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,23,27)'[v1];
    ///   [v1][2]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,44,61)'[v2];
    ///   [v2][3]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='gt(t,112)'[v3]"
    /// -map "[v3]" -map 0:a  out.mp4
    pub(super) image_filter_complex : String,
    /// How many images have been added to the filter_complex
    pub(super) image_index : u16,

    /// The start_position of the image
    pub(super) image_start_position : f64
}

impl Deref for SharedGeneratorLogic<'_> {
    type Target = VideoGenerator;
    fn deref(&self) -> &Self::Target {
        self.video_generator   
    }
}

impl<'a> SharedGeneratorLogic<'a> {
    pub fn new(video_generator : &'a VideoGenerator,bin_directory : &str) -> std::io::Result<Self> { 
        let audio_concat_file_path = format!("{bin_directory}/concat.txt");

        let audio_concat_file = File::create(&audio_concat_file_path)?;

        Ok(Self { 
            video_generator,
            
            audio_concat_file, 
            audio_concat_file_path,

            image_filter_complex : Default::default(),
            image_index : 0,
            image_start_position : 0.0
        })
    }
}

impl SharedGeneratorLogic<'_> {
    pub fn append_audio(&mut self,audio_file : &str) -> std::io::Result<()> {
        // Update File
        self.audio_concat_file.write_all("file ".as_bytes())?;

        let name_bytes = Path::new(audio_file).file_name().unwrap().as_encoded_bytes();
        self.audio_concat_file.write_all(name_bytes)?;
        self.audio_concat_file.write_all(b"\n")?;
        Ok(())
    }

    pub fn append_audio_inpoint(&mut self,inpoint : &f64) -> std::io::Result<()> {        
        self.audio_concat_file.write_all(b"inpoint ")?;
        self.audio_concat_file.write_all(inpoint.to_string().as_bytes())
    }

    pub fn append_audio_outpoint(&mut self,outpoint : &f64) -> std::io::Result<()> {        
        self.audio_concat_file.write_all(b"outpoint ")?;
        self.audio_concat_file.write_all(outpoint.to_string().as_bytes())
    }

    pub fn append_audio_point(&mut self,inpoint : &f64,outpoint : &f64) -> std::io::Result<()> {        
        self.append_audio_inpoint(inpoint)?;
        self.append_audio_outpoint(outpoint)
    }

    fn shared_append_image<'a>(&mut self,enable : impl FnOnce() -> &'a str) -> std::io::Result<()> {
        let image_index = &mut self.image_index;
        let next = *image_index + 1;

        let start_position = &mut self.image_start_position;

        // for this part [v1][2]
       
        let prefix = match image_index == &0 {
            true => "",
            false => "v",
        };

        let enable_filter = enable();

        //[v1][2]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,44,61)'[v2];
        self.image_filter_complex.push_str(
            &format!("[{prefix}{image_index}][{next}]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='{enable_filter}'[v{next}];")
        );

        *image_index = next;

        Ok(())
    }
    /// Should be called after [Self::append_audio_point] , [Self::append_audio_outpoint] , [Self::append_audio_inpoint] to ensure expected behaviour
    pub fn append_image(
        &mut self,
        offset_by : f64,
    ) -> std::io::Result<()> {
        let end = self.image_start_position + offset_by;

        self.shared_append_image(||{
            let start_position = &self.image_start_position;
            &format!("between(t,{start_position},{end})")
        })?;

        self.image_start_position = end;
        Ok(())
    }

    /// **Note** : This method does not set [Self::image_start_position] to the end
    pub fn append_image_till_end(&mut self) -> std::io::Result<()> {
        self.shared_append_image(||{
            let start_position = &self.image_start_position;
            &format!("gt(t,{start_position}")
        })
    }

    /// Returns Generated Video Path
    pub fn finalize_video(
        &self,
        bin_directory : &str,
        output_directory : &str,
        image_inputs : impl FnOnce(&mut Command),
    ) -> std::io::Result<String> {
        let concated_audio = self.concat_audio_files(bin_directory)?;

        super::utils::get_duration_str(
            &self.video_generator.ffmpeg,
            &concated_audio,
            |concated_audio_length| {
                let background_audio = self.prepare_background_music(bin_directory,concated_audio_length)?;
                let video_path = self.prepare_background_video(bin_directory,concated_audio_length,image_inputs)?;

                // TODO : Use this https://filmora.wondershare.com/video-editor/ffmpeg-merge-audio-and-video.html 
                // to fix this :  Have to do this as can't find a way to successfully create video with audio in a single command
                let audio_path = self.combine_background_and_concated_audio(bin_directory,&background_audio,&concated_audio)?;
                self.concat_video_with_audio(
                    output_directory, 
                    &video_path, 
                    &audio_path
                )
            }
        )
    }
}