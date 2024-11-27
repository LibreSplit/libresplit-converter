use std::io::Cursor;

use spex::parsing::XmlReader;
use wasm_bindgen::prelude::*;

mod livesplit;

#[wasm_bindgen]
pub fn convert(file: String) -> String {
    let cursor = Cursor::new(file);
    let xml = XmlReader::parse_auto(cursor).unwrap();
    livesplit::read(xml).get()
}
