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
    let mut buf = String::new();

    // Read game name.
    let game_name = file.root().req("GameName").element().unwrap();
    buf.push_str(game_name.text().unwrap());

    // Read category name.
    let category_name = file.root().req("CategoryName").element().unwrap();
    buf.push_str(category_name.text().unwrap());

    buf
}
