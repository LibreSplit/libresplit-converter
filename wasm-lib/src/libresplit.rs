use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LibreSplitFile {
    title: String,
    splits: Vec<Split>,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    title: String,
    time: String,
    best_time: String,
    best_segment: String,
}
