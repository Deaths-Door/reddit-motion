#![doc = include_str!("../README.md")]

mod setup;
mod settings;
mod redditconfig;

use self::setup::*;

fn main() {
    print_initial_messages();

    let setting = obtain_and_validate_settings_toml();
}