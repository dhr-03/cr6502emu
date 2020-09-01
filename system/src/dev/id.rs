use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DeviceId {
    // IO
    PixelScreen = 10,

    // MEM
    Rom = 100,
    Ram = 101,

    // Special
    CPU = 255,
}

impl DeviceId {
    pub fn fixed_size_of(&self) -> Option<u16> {
        match self {
            Self::PixelScreen => Some(50 * 50),

            Self::Rom => None,
            Self::Ram => None,

            Self::CPU => None,
        }
    }
}
