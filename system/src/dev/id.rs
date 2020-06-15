use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone)]
pub enum DeviceId {
    // IO

    // MEM
    Rom = 100,
    Ram = 101,
}