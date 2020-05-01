use wasm_bindgen::prelude::*;

mod utils;
mod assembler;
mod opcodes;
mod lang;


#[wasm_bindgen]
pub fn build_date() -> String {
    env!("build_timestamp").to_owned()
}