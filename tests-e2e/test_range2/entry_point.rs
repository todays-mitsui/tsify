use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Range<T, U> {
    t: T,
    u: U,
}

#[wasm_bindgen]
pub fn consume(_range: Range<u32, String>) {}

#[wasm_bindgen]
pub fn into_js() -> Range<u32, String> {
    Range {
        t: 42,
        u: "foobar".to_string(),
    }
}

#[wasm_bindgen]
pub fn consume_vector(_ranges: Vec<Range<u32, String>>) {}

#[wasm_bindgen]
pub fn vector_into_js() -> Vec<Range<u32, String>> {
    vec![
        Range {
            t: 42,
            u: "foobar".to_string(),
        },
        Range {
            t: 42,
            u: "foobar".to_string(),
        },
        Range {
            t: 42,
            u: "foobar".to_string(),
        },
    ]
}
