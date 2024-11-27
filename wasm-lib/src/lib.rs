use wasm_bindgen::prelude::*;

mod livesplit;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
