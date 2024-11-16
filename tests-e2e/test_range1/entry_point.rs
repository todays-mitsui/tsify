use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Range {}

#[wasm_bindgen]
pub fn consume(_range: Range) {}

#[wasm_bindgen]
pub fn into_js() -> Range {
    Range {}
}

#[wasm_bindgen]
pub fn consume_vector(_ranges: Vec<Range>) {}

#[wasm_bindgen]
pub fn vector_into_js() -> Vec<Range> {
    vec![Range {}, Range {}, Range {}]
}
