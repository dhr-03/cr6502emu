use super::{DeviceId, DeviceHolder};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct DeviceRepresentation {
    /// The device type.
    id: DeviceId, // type is a reserved keyword, we should rename this so it doesnt get confused with uid.

    /// The range used in the address bus by the device.
    ///
    /// **WARNING:**
    /// This doesnt mean that the user/cpu can access the hole range, another device might be partially/totally overlapping.
    range: (u16, u16),

    /// The uid (unique-id) assigned to this device by the client.
    uid: u16,
}

impl DeviceRepresentation {
    pub fn from_dev_holder(holder: &DeviceHolder) -> Self {
        let range = (
            holder.range().start,
            holder.range().end
        );

        DeviceRepresentation {
            id: DeviceId::Ram,

            range,

            uid: holder.uid(),
        }
    }
}

#[wasm_bindgen]
impl DeviceRepresentation {
    #[wasm_bindgen(getter)]
    pub fn type_id(&self) -> DeviceId {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn range_start(&self) -> u16 {
        self.range.0
    }

    #[wasm_bindgen(getter)]
    pub fn range_end(&self) -> u16 {
        self.range.1
    }

    // This struct is only used by the client, its kind of stupid to implement this method in rust
    // and add extra calls between js and wasm. The ideal thing would be to do something like:
    // ** #[wasm_bindgen(export_js = "function range_size(...){...}")]
    // but i cant find any option for that.
    //
    //pub fn range_size(&self) -> u16 {}

    #[wasm_bindgen(getter)]
    pub fn uid(&self) -> u16 {
        self.uid
    }
}
