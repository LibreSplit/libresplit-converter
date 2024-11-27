use spex::xml::XmlDocument;

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

pub fn read(file: XmlDocument) -> String {
    // Read game name.
    let elm_game_name = file.root().opt("GameName").element();
    let game_name = match elm_game_name {
        Some(name) => name.text().expect("Unknown Game"),
        None => "Unknown Game",
    };

    // Read category.
    let elm_category_name = file.root().opt("CategoryName").element();
    let category_name = match elm_category_name {
        Some(category) => category.text().expect("Unknown Category"),
        None => "Unknown Category",
    };

    // Read platform.
    let elm_platform = file.root().opt("Platform").element();
    let platform  = match elm_platform {
        Some(plat) => plat.text().expect("Unknown Platform"),
        None => "Unknown Platform",
    };

    // Read attempt count.
    let elm_attempt_count = file.root().opt("AttemptCount").element();
    let attempt_count_str = match elm_attempt_count {
        Some(count_str) => count_str.text().expect("0"),
        None => "0"
    };
    let attempt_count: u32 = attempt_count_str.trim().parse().unwrap_or(0);

    game_name.to_string()
}
