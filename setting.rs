use std::fs::File;
use std::path::Path;
use std::io::Read;

use colored::*;

use serde::Deserialize;
use toml::from_str as toml_str_to;


#[derive(Deserialize)]
pub(in crate) struct Settings {
    reddit : RedditConfig
};

impl Settings {
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

    pub(in crate) fn read_toml(mut file : File) -> Result<Self,()> {
        let mut toml_content = String::new();

        match file.read_to_string(&mut toml_content) {
            Err(error) => {
                println!("{} : {error}","Error reading settings.toml".red());
                Err(())
            }
            Ok(_) => {
                
            }/*match toml_str_to(&toml_content) {
                Err(error) => {
                    println!("{} : {error}","Error deserializing settings.toml".red());
                    Err(())
                }
                Ok(setting) => Ok(setting)
            }*/
        }
    }

    pub(in crate) fn validate_file() -> Result<Self,()> {
        Self::print_checking_toml();

        let mut file = match Self::obtain_file_instance() {
            Ok(file) => file,
            _ => return Err(())
        };
    
        let setting = match Self::read_toml(file) {
            Ok(setting) => setting,
            _ => return Err(())
        };
        Ok(Settings())
    }
}
