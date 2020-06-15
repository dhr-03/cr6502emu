pub trait DeviceTrait {
    /// This method is called on every system + cpu tick.
    fn tick(&mut self) {}

    /// This method is called when a system reset is requested.
    fn reset_system(&mut self);

    /// This method is called when a device hard reset is requested.
    ///
    /// After executing this, the device should look brand new, without any data on it.
    fn reset_hard(&mut self);
}

pub trait AddressableDeviceTrait: DeviceTrait {
    /// Returns the actual size of the device in the address bus.
    fn size(&self) -> u16;

    /// Read value at `offset`.
    ///
    /// The offset has already been validated by `system::MemManager`.
    fn read(&self, offset: u16) -> u8;

    /// Write `value` at `offset`.
    ///
    /// The offset has already been validated by `system::MemManager`.
    #[allow(unused_variables)]
    fn write(&mut self, offset: u16, value: u8) {
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
