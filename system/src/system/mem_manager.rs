use super::{BoxedDev, DevHolderVec, Bus};

// The design is kind of weird because i was having trouble with the lack of support for
// self referencing structs in Rust.

/// This struct owns the devices, acts as a proxy for the Bus, and maps addresses.
pub struct MemManager {
    bus: Bus,

    devices: DevHolderVec,
}

impl MemManager {
    pub fn new() -> Self {
        MemManager {
            bus: Bus::new(),

            devices: DevHolderVec::new(),
        }
    }

    pub fn devices_mut(&mut self) -> &mut DevHolderVec {
        &mut self.devices
    }

    pub fn tick(&mut self) {
        self.bus.set_rw(true);

        for dev in &mut self.devices {
            dev.device_mut().tick();
        }
    }
}

//utils
impl MemManager {
    fn map_addr_mut(&mut self, addr: u16) -> Option<(&mut BoxedDev, u16)> {
        let mut result = None;
        let iter = &mut self.devices.iter_mut();

        while let (Some(holder), None) = (iter.next(), &result) {
            if holder.range().contains(&addr) {
                let offset = addr - holder.range().start;
                let dev_ref = holder.device_mut();

                result = Some((
                    dev_ref, offset
                ))
            }
        }

        result
    }
}

//bus proxy
impl MemManager {
    /// Returns the data bus value
    pub fn get_data(&self) -> u8 {
        self.bus.data()
    }

    /// Sets the data bus value
    pub fn set_data(&mut self, data: u8) {
        *self.bus.data_mut_ref() = data;
    }

    /// Returns the address that the bus in pointing to
    pub fn addr(&self) -> u16 {
        self.bus.addr()
    }

    /// Reads and returns the value in the address `self.addr()`
    pub fn read_at_addr(&mut self) -> u8 {
        self.bus.set_rw(true);

        let mapped = self.map_addr_mut(self.bus.addr());

        if let Some((dev, offset)) = mapped {
            let val = dev.read(offset);

            *self.bus.data_mut_ref() = val;
        } else {
            //TODO: temp
            use wasm_bindgen::prelude::wasm_bindgen;
            #[wasm_bindgen]
            extern {
                fn alert(msg: &str);
            }

            alert("unk addr: read");

            *self.bus.data_mut_ref() = 0;
        }

        self.bus.data()
    }

    /// Writes some value to the address in `self.addr()`
    pub fn write_at_addr(&mut self) {
        self.bus.set_rw(false);

        // this is needed here because mapped uses a mut ref to self
        let current_data = self.bus.data();

        let mapped = self.map_addr_mut(self.bus.addr());

        if let Some((dev, offset)) = mapped {
            dev.write(offset, current_data);
        } else {
            //TODO: temp
            use wasm_bindgen::prelude::wasm_bindgen;
            #[wasm_bindgen]
            extern {
                fn alert(msg: &str);
            }

            alert("unk addr: write");
        }
    }

    /// Sets the address pointer to some value
    pub fn set_addr(&mut self, addr: u16) {
        *self.bus.addr_mut_ref() = addr;
    }

    /// Sets the low part of the current address pointer, keeping whatever is on the high part
    pub fn set_addr_lo(&mut self, addr: u8) {
        *self.bus.addr_mut_ref() &= 0xFF00;
        *self.bus.addr_mut_ref() |= addr as u16;
    }

    /// Sets the high part of the current address pointer, keeping whatever is on the low part
    pub fn set_addr_hi(&mut self, addr: u8) {
        *self.bus.addr_mut_ref() &= 0x00FF;
        *self.bus.addr_mut_ref() |= (addr as u16) << 8;
    }
}