use spex::xml::XmlDocument;

pub struct LiveSplitFile {
    pub game_name: String,
    pub category_name: String,
    pub platform: String,
    pub attempt_count: u32,
    pub segments: Vec<Segment>,
}

pub struct Segment {
    pub name: String,
    pub split_time: String,
}

pub fn read(file: XmlDocument) -> LiveSplitFile {
    // Read game name.
    let elm_game_name = file.root().opt("GameName").element();
    let game_name = match elm_game_name {
        Some(name) => name.text().expect("Unknown Game"),
        None => "Unknown Game",
    }.to_string();

    // Read category.
    let elm_category_name = file.root().opt("CategoryName").element();
    let category_name = match elm_category_name {
        Some(category) => category.text().expect("Unknown Category"),
        None => "Unknown Category",
    }.to_string();

    // Read platform.
    let elm_platform = file.root().opt("Platform").element();
    let platform  = match elm_platform {
        Some(plat) => plat.text().expect("Unknown Platform"),
        None => "Unknown Platform",
    }.to_string();

    // Read attempt count.
    let elm_attempt_count = file.root().opt("AttemptCount").element();
    let attempt_count_str = match elm_attempt_count {
        Some(count_str) => count_str.text().expect("0"),
        None => "0"
    };
    let attempt_count: u32 = attempt_count_str.trim().parse().unwrap_or(0);

    // Read splits.
    let mut segments: Vec<Segment> = Vec::new();
    // TODO: this expect() is weird
    let elm_segments = file.root().opt("Segments").element().expect("");
    for elm_segment in elm_segments.elements().filter(|e| e.is_named("Segment")) {
        let segment = Segment {
            name: elm_segment.opt("Name").element().expect("").text().unwrap_or("Unknown Split").to_string(),
            split_time: elm_segment.opt("BestSegmentTime").opt("RealTime").element().expect("").text().unwrap_or("0.000000").to_string(),
        };
        segments.push(segment);
    }

    LiveSplitFile {game_name, category_name, platform, attempt_count, segments}
}
