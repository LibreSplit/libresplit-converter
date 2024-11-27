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

    game_name.to_string()
}
