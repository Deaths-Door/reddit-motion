#![doc = include_str!("../README.md")]

mod assets;
mod errors;

use errors::*;

#[tokio::main]
async fn main() -> Result<(),AssetsError>  {
    println!(r#"
    ____           __    ___ __     __  ___      __  _           
   / __ \___  ____/ /___/ (_) /_   /  |/  /___  / /_(_)___  ____ 
  / /_/ / _ \/ __  / __  / / __/  / /|_/ / __ \/ __/ / __ \/ __ \
 / _, _/  __/ /_/ / /_/ / / /_   / /  / / /_/ / /_/ / /_/ / / / /
/_/ |_|\___/\__,_/\__,_/_/\__/  /_/  /_/\____/\__/_/\____/_/ /_/ 
              
Thanks for using this tool! 
Feel free to contribute to this project on GitHub! 
If you have any questions, feel free to join my Discord server or submit a GitHub issue. 
You can find solutions to many common problems in the documentation: https://docs.rs/reddit_motion
"# );


    assets::download_background_videos().await?;

    //.. config checked

    

    Ok(())
}