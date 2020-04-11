use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

mod utils;

mod assembler;
mod opcodes;

#[wasm_bindgen]
extern {
    pub fn alert(t: &str);

}
