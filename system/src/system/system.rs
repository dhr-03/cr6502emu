use wasm_bindgen::prelude::wasm_bindgen;

use super::MemManager;

use crate::cpu::CPU;
use crate::dev::{DeviceId, DeviceFactory};

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

    pub fn tick(&mut self) {
        self.mem.tick(); //tick the bus and all the devices

        self.cpu.tick(&mut self.mem);
    }

    pub fn tick_x(&mut self, amm: i32) {
        for _ in 0..amm {
            self.tick();
        }
    }

    /// Resets the system, clearing all non-persistent data containers.
    pub fn reset_system(&mut self) {
        self.cpu.reset();

        self.mem.reset_bus();
        self.mem.reset_devices();
    }

    /// Resets the system, clearing all data containers, including persistent ones like the rom.
    pub fn reset_hard(&mut self) {
        self.cpu.reset();

        self.mem.reset_bus();
        self.mem.reset_devices_hard();
    }

    pub fn add_device(&mut self, device: DeviceId, start: u16, size: u16) -> bool {
        if (std::u16::MAX - size) >= start { //check for overflows
            let result = DeviceFactory::with_size(device, size);

            match result {
                Ok(dev) => {
                    let end = start + dev.size();

                    self.mem.add_device_unchecked_range(dev, start, end);

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
        self.mem.remove_device_by(index)
    }

    /// WARNING: Using raw pointers might cause system instability,
    /// make sure you know what you're doing.
    pub fn device_data_ptr(&mut self, index: usize) -> Option<usize> {
        self.mem.device_data_ptr(index)
    }

    pub fn device_size(&self, index: usize) -> Option<u16> {
        self.mem.devices().get(index)
            .map_or(
                None,
                |dev| Some(dev.device().size()),
            )
    }
}
