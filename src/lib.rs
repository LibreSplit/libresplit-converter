use std::io::Cursor;

use spex::parsing::XmlReader;
use wasm_bindgen::prelude::*;

mod libresplit;
mod livesplit;

#[wasm_bindgen]
pub fn convert(file: String) -> String {
    let cursor = Cursor::new(file);
    let xml = XmlReader::parse_auto(cursor).unwrap();
    let livesplit_data = livesplit::LiveSplitFile::new(xml);
    libresplit::LibreSplitFile::from_livesplit(livesplit_data).get()
}
