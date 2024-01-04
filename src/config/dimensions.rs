use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    #[serde(default = "dwidth")]
    width : u32,
    #[serde(default = "dheight")]
    height : u32,
}

fn dwidth() -> u32 { 800 }
fn dheight() -> u32 { 600 }