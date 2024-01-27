use serde::{Deserialize,Serialize};

#[derive(Default,Clone,Deserialize,Serialize)]
pub enum VideoDuration {
    #[default]
    Infinite,
    Limited { limit: f64 },
    Both { limit: f64 }
}