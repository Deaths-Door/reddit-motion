#![doc = include_str!("../README.md")]

mod localizer;
mod config;

use localizer::println;
use config::Config;

fn main() {
    println!(
r#"
    ____           __    ___ __     __  ___      __  _           
   / __ \___  ____/ /___/ (_) /_   /  |/  /___  / /_(_)___  ____ 
  / /_/ / _ \/ __  / __  / / __/  / /|_/ / __ \/ __/ / __ \/ __ \
 / _, _/  __/ /_/ / /_/ / / /_   / /  / / /_/ / /_/ / /_/ / / / /
/_/ |_|\___/\__,_/\__,_/_/\__/  /_/  /_/\____/\__/_/\____/_/ /_/ 
"#
    );

    println("intro");
 
    let config = Config::try_from("config.toml");
}