use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use crate::livesplit::LiveSplitFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct LibreSplitFile {
    pub title: String,
    pub attempt_count: u32,
    pub splits: Vec<Split>,
    pub width: u32,
    pub height: u32,
}

impl LibreSplitFile {
    pub fn from_livesplit(lss: LiveSplitFile) -> Self {
        // Get title.
        let title = lss.game_name + " (" + &lss.platform + ") " + &lss.category_name;
        let attempt_count = lss.attempt_count;

        // Constructs splits vector.
        let mut splits: Vec<Split> = Vec::new();
        for lss_split in lss.segments {
            let split = Split {
                title: lss_split.name,
                time: lss_split.split_time,
                best_time: "0.000000".to_string(),
                best_segment: "0.000000".to_string(),
            };
            splits.push(split);
        }

        // Get size. Default for now.
        let width = 600;
        let height = 800;

        LibreSplitFile {
            title,
            attempt_count,
            splits,
            width,
            height,
        }
    }

    pub fn get(&self) -> String {
        let rtn = to_string_pretty(&self).unwrap_or("".to_string());
        format!("{}", rtn)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Split {
    pub title: String,
    pub time: String,
    pub best_time: String,
    pub best_segment: String,
}
