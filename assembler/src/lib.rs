use wasm_bindgen::prelude::*;

mod utils;
mod assembler;
mod opcodes;

#[wasm_bindgen]
extern {
    pub fn alert(t: &str);

}