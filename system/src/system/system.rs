use wasm_bindgen::prelude::wasm_bindgen;

use super::MemManager;

use crate::cpu::CPU;
use crate::dev::{DeviceId, DeviceFactory, DeviceHolder};

#[wasm_bindgen]
pub struct System {
    cpu: CPU,

    mem: MemManager,
}

#[wasm_bindgen]
impl System {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        System {
            cpu: CPU::new(),

            mem: MemManager::new(),
        }
    }

    pub fn with_default_mem_map() -> Self {
        let mut sys = Self::new();

        //TODO: invalid
        sys.add_device(DeviceId::Ram, 0, 0x1000);
        sys.add_device(DeviceId::Rom, 0x1000, 0x1000);

        sys
    }

    pub fn tick(&mut self) {
        self.mem.tick(); //tick the bus and all the devices

        self.cpu.tick(&mut self.mem);
    }

    pub fn tick_x(&mut self, amm: i32) {
        for _ in 0..amm {
            self.tick();
        }
    }

    pub fn reset(&mut self) {
        unimplemented!();
    }

    pub fn add_device(&mut self, device: DeviceId, start: u16, size: u16) -> bool{
        if (std::u16::MAX - size) >= start { //check for overflows
            let result = DeviceFactory::with_size(device, size);

            match result {
                Ok(dev) => {
                    let end = start + dev.size();

                    let holder = DeviceHolder::new(dev, start, end);

                    self.mem.devices_mut().push(holder);

                    true
                }

                Err(_) => {
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn remove_device(&mut self, index: usize) -> bool {
        if index < self.mem.devices_mut().capacity() {
            self.mem.devices_mut().remove(index);

            true
        } else {
            false
        }
    }

    /// WARNING: Raw pointers might cause system instability,
    /// make sure you know what you're doing.
    #[allow(non_snake_case)]
    pub fn UNSAFE_device_data_ptr(&mut self, index: usize) -> Option<usize> {
        self.mem.devices_mut().get_mut(index)
            .map_or_else(
                || None,
                |dev| Some(dev.device_mut().data_ptr() as usize)
            )
    }

    pub fn device_size(&self, index: usize) -> Option<u16> {
        self.mem.devices().get(index)
            .map_or(
                None,
            |dev| Some(dev.device().size())
            )
    }

    pub fn tmp_to_str(&self) -> String {
        format!("{}\nbus data: {}\nbus addr: {}",
                self.cpu.tmp_to_str(),
                self.mem.data(),
                self.mem.addr()
        )
    }
}