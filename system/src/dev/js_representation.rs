use super::{DeviceId, DeviceHolder};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(raw_module = "../../js/deviceRepresentation.js")]
extern {
    pub type DeviceRepresentation;

    #[wasm_bindgen(constructor)]
    pub fn new(id: DeviceId, start: u16, end: u16, uid: u16, has_fixed_size: bool) -> DeviceRepresentation;
}

impl DeviceRepresentation {
    pub fn from_dev_holder(holder: &DeviceHolder) -> Self {
        DeviceRepresentation::new(
            holder.device().device_id(),

            holder.range().start,
            holder.range().end,

            holder.uid(),

            holder.device().device_id().has_fixed_size(),
        )
    }
}
