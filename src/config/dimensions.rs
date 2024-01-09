use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Dimensions {
    #[serde(default = "dwidth")]
    pub(crate) width : u32,
    #[serde(default = "dheight")]
    pub(crate) height : u32,
}

fn dwidth() -> u32 { 800 }
fn dheight() -> u32 { 600 }

impl Dimensions {
    pub const fn width_height(&self) -> (u32,u32) {
        (self.width,self.height)
    }
}