use std::io::Cursor;

use spex::parsing::XmlReader;
use wasm_bindgen::prelude::*;

mod livesplit;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn convert(file: String) -> String {
    let cursor = Cursor::new(file);
    let xml = XmlReader::parse_auto(cursor).unwrap();
    livesplit::read(xml).get()
}
