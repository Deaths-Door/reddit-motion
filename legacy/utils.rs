impl VideoGenerator {
    pub(super) fn cleanup(self) -> std::io::Result<()> {
        std::fs::remove_dir_all(&self.video_gen_files.storage_directory)
    }

    pub(super) fn title_segment(
        &self,
        bin_directory : &str,
        video_path : &str,
    )  -> std::io::Result<(CoreCreationCommand,f64,Iter<'_, (String, String)>)> {
        let start_position = random_start_point(&self.ffmpeg,video_path)?;

        let mut iter = self.video_gen_files.files.iter();
        let (audio_file,png_file) = iter.next().unwrap();

        let audio_length = get_duration(&self.ffmpeg, &audio_file)?;

        let mut core_creation_command = CoreCreationCommand::new(bin_directory,start_position)?;
        core_creation_command.extend_command(
            png_file,
            &audio_file,
            audio_length,
        )?;

        Ok((core_creation_command,audio_length,iter))
    }
}