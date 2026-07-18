#![allow(dead_code)]
#![allow(clippy::new_without_default)]
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// This struct will be exported to JavaScript
#[wasm_bindgen]
pub struct FastBPETokenizer {
    vocab: HashMap<String, u32>,
}

#[wasm_bindgen]
impl FastBPETokenizer {
    // Constructor called from JS: const tokenizer = new FastBPETokenizer();
    #[wasm_bindgen(constructor)]
    pub fn new() -> FastBPETokenizer {
        FastBPETokenizer {
            vocab: HashMap::new(),
        }
    }

    // The main tokenization function
    #[wasm_bindgen]
    pub fn tokenize(&self, text: &str) -> Vec<u32> {
        // TODO: Implement actual Byte-Pair Encoding merge logic
        // Currently stubbed to return a 0 for every word
        text.split_whitespace().map(|_| 0).collect()
    }
}
