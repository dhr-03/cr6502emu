use super::DeviceTrait;

pub trait AddressableDeviceTrait: DeviceTrait {
    /// Returns the actual size of the device in the address bus.
    fn size(&self) -> u16;

    /// Read value at `offset`.
    ///
    /// The offset has already been validated by `system::MemManager`.
    fn read_unchecked(&self, offset: u16) -> u8;

    /// Write `value` at `offset`.
    ///
    /// The offset has already been validated by `system::MemManager`.
    #[allow(unused_variables)]
    fn write_unchecked(&mut self, offset: u16, value: u8) {
        //TODO: temp
        use wasm_bindgen::prelude::wasm_bindgen;
        #[wasm_bindgen]
        extern {
            fn alert(msg: &str);
        }

        alert("non writable");
    }

    /// # Returns
    /// Returns a raw ptr to the device data if possible or a null ptr.
    fn data_ptr(&mut self) -> *const u8 {
        std::ptr::null()
    }
}
