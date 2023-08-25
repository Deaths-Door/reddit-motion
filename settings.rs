
use self::redditconfig::*;

use std::{
    fs::File,
    path::Path,

};

use serde::Deserialize;
use colored::Colorize;



#[derive(Deserialize)]
pub(in crate) struct Settings {
    reddit : RedditConfig
}

impl Settings {
    pub(in crate::setup) fn print_checking_toml() {
        const CHECKING_TOML : &str = r#"
|------------------------|
|                        |
| Checking configuration |
|                        |
|------------------------|
If you see any prompts, that means that you have unset/incorrectly set variables, please input the correct values.
        "#;
        println!("{}",CHECKING_TOML.blue());
    }

    fn obtain_file_instance() -> Result<File,()> {
        const SETTING_TOML_DIRECTORY : &str = "setting.toml";

        let _path = Path::new(SETTING_TOML_DIRECTORY);

        match _path.exists() {
            false => {
                println!("Attempting to create setting.toml");
                match File::create(SETTING_TOML_DIRECTORY) {
                    Err(error) => {
                        println!("{} : {error}","Error creating setting.toml".red());
                        Err(())
                    }
                    Ok(file) => {
                        println!("{}","Successfully created settings.toml".green());
                        Ok(file)
                    }
                }
            }
            true => {
                println!("Attempting to open setting.toml");
                match File::open(SETTING_TOML_DIRECTORY) {
                    Err(error) => {
                        println!("{} : {error}","Error opening setting.toml".red());
                        Err(())
                    }
                    Ok(file) => {
                        println!("{}","Successfully opened settings.toml".green());
                        Ok(file)
                    }
                }
            }
        }
    }

}