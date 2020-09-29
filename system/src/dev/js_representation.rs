use super::{DeviceId, DeviceHolder};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(raw_module = "../../js/deviceRepresentation/deviceRepresentationFactory.js")]
extern {
    pub type DeviceRepresentationFactory;
    pub type DeviceRepresentation;

    #[wasm_bindgen(static_method_of = DeviceRepresentationFactory)]
    pub fn new(id: DeviceId, start: u16, end: u16, uid: u16) -> DeviceRepresentation;
}

impl DeviceRepresentationFactory {
    pub fn from_dev_holder(holder: &DeviceHolder) -> DeviceRepresentation {
        DeviceRepresentationFactory::new(
            holder.device().device_id(),

            holder.range().start,
            holder.range().end,

            holder.uid(),
        )
    }
}
