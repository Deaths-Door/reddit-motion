use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct Assets {
    videos : Vec<String>,
    audio : Vec<String>
}