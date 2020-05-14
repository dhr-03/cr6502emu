use wasm_bindgen::prelude::*;

mod utils;
mod opcodes;
mod lang;
mod js_regex;

#[allow(dead_code)]
mod js_logger;

mod assembler;
mod parser;

#[wasm_bindgen]
pub fn build_date() -> String {
    env!("build_timestamp").to_owned()
}