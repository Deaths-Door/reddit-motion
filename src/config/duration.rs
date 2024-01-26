use serde::{Deserialize,Serialize};

#[derive(Default,Clone,Deserialize,Serialize)]
pub enum VideoDuration {
    #[default]
    Infinite,
    Short { limit: u32 },
    Both { limit: u32 }
}