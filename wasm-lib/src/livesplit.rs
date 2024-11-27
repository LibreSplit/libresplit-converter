pub struct LiveSplitFile {
    game_name: String,
    category_name: String,
    platform: String,
    attempt_count: u32,
    segments: Vec<Segment>,
}

pub struct Segment {
    split_time: String,
}
