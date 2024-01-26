use std::{fs::File, io::Write, ops::Deref, path::Path};

use crate::video_generator::VideoGenerator;

pub struct SharedGeneratorLogic<'a> {
    video_generator : &'a VideoGenerator,

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
    image_start_position : f64
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
        self.audio_concat_file.write_all("\n".as_bytes())?;
        Ok(())
    }

    pub fn append_image(
        &mut self,
        offset_by : f64,
    ) -> std::io::Result<()> {
        let image_index = &mut self.image_index;
        let next = *image_index + 1;
        println!("image_index={image_index} next={next}");
        // TODO : UPDATE THIS
        let start_position = &mut self.image_start_position;

        // for this part [v1][2]
       
        let prefix = match image_index == &0 {
            true => "",
            false => "v",
        };

        let end = *start_position + offset_by;

        //[v1][2]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,44,61)'[v2];
        self.image_filter_complex.push_str(
            &format!("[{prefix}{image_index}][{next}]overlay=x=(main_w-overlay_w)/2:y=(main_h-overlay_h)/2:enable='between(t,{start_position},{end})'[v{next}];")
        );

        *image_index = next;
        *start_position = end;

        Ok(())
    }
}