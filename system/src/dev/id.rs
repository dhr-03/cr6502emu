use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DeviceId {
    // IO
    PixelScreen = 10,
    AsciiIOBuffer = 20,

    // MEM
    Rom = 100,
    Ram = 101,

    // Special
    CPU = 255,
}

impl DeviceId {
    pub fn has_fixed_size(&self) -> bool {
        match self {
            Self::PixelScreen => true,
            Self::AsciiIOBuffer => true,

            Self::Rom => false,
            Self::Ram => false,

            Self::CPU => true,
        }
    }
}
