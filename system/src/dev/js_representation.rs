use super::{DeviceId, DeviceHolder};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(raw_module = "../../js/deviceRepresentation.js")]
extern {
    pub type DeviceRepresentation;

    #[wasm_bindgen(constructor)]
    pub fn new(id: DeviceId, start: u16, end: u16, uid: u16) -> DeviceRepresentation;
}

impl DeviceRepresentation {
    pub fn from_dev_holder(holder: &DeviceHolder) -> Self {
        DeviceRepresentation::new(
            DeviceId::Ram,

            holder.range().start,
            holder.range().end,

            holder.uid()
        )
    }
}
