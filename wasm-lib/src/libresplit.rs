use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LibreSplitFile {
    pub title: String,
    pub splits: Vec<Split>,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    pub title: String,
    pub time: String,
    pub best_time: String,
    pub best_segment: String,
}
