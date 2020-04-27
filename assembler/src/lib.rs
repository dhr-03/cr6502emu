use wasm_bindgen::prelude::*;

mod utils;
mod assembler;
mod opcodes;

#[wasm_bindgen]
extern {
    pub fn alert(t: &str);

}


#[wasm_bindgen]
pub fn build_date() -> String {
    env!("build_timestamp").to_owned()
}