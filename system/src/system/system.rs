use wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Map;

use super::MemManager;

use crate::cpu::CPU;
use crate::dev::{DeviceId, DeviceFactory, DeviceRepresentation, DeviceTrait};

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

        self.cpu.tick_with_mem(&mut self.mem);
    }

    pub fn tick_x(&mut self, amm: i32) {
        for _ in 0..amm {
            self.tick();
        }
    }

    pub fn execute_operation(&mut self) {
        let mut continue_execution = true;

        while continue_execution {
            self.tick();

            continue_execution = !self.cpu.operation_is_done();
        }
    }

    /// Resets the system, clearing all non-persistent data containers.
    pub fn reset_system(&mut self) {
        self.cpu.reset_system();

        self.mem.reset_bus();
        self.mem.reset_devices();
    }

    /// Resets the system, clearing all data containers, including persistent ones like the rom.
    pub fn reset_hard(&mut self) {
        self.cpu.reset_hard();

        self.mem.reset_bus();
        self.mem.reset_devices_hard();
    }

    pub fn add_device_with_uid(&mut self, device: DeviceId, start: u16, size: u16, uid: u16) -> bool {
        if (std::u16::MAX - size) >= start { //check for overflows
            let result = DeviceFactory::with_size(device, size);

            match result {
                Ok(dev) => {
                    let end = start + dev.size();

                    self.mem.add_device_unchecked_range(dev, start, end, uid);

                    true
                }

                Err(_) => {
                    // for now the only possible error (in the factory) is an invalid size,
                    // in the future we might want to be more explicit.
                    false
                }
            }
        } else {
            // the range should have already been validated by the client,
            // this wont be reached if properly implemented.
            false
        }
    }


    //
    // Notes about devices index:
    //
    // Device index 0 represents the cpu, so for added devices, the index actually is (index - 1).
    //
    // Instead of checking if index > 0 im just going to let it underflow (unless special behavior is required)
    // and be handled by the invalid index branch.
    //


    pub fn remove_device_by_index(&mut self, index: usize) -> bool {
        self.mem.remove_device_by(index - 1)
    }

    // we cant (yet?) send a Vec/n size array (at least not without using serde and its huge dependencies),
    // maybe we could change this in the future.
    /// Returns a representation of device [Index], if it exists, or a None/null.
    pub fn device_representation_by_index(&self, index: usize) -> Option<DeviceRepresentation> {
        if index == 0 {
            Some(
                DeviceRepresentation::new(DeviceId::CPU, 0, 0, 0, true)
            )
        } else {
            self.mem.devices()
                .get(index - 1)
                .map(|dev| DeviceRepresentation::from_dev_holder(dev))
        }
    }

    /// WARNING: Using raw pointers might cause system instability,
    /// make sure you know what you're doing.
    pub fn device_data_ptr_by_index(&mut self, index: usize) -> Option<usize> {
        self.mem.device_data_ptr(index - 1)
    }

    pub fn device_widget_update_by_index(&mut self, index: usize) -> Option<Map> {
        if index == 0 {
            self.cpu.update_widget()
        } else {
            self.mem.devices_mut()
                .get_mut(index - 1)
                .and_then(|dev| dev.device_mut().update_widget())
        }
    }

    pub fn device_widget_setup_by_index(&mut self, index: usize, data: Map) -> Option<Map> {
        if index == 0 {
            self.cpu.setup_widget(data)
        } else {
            self.mem.devices_mut()
                .get_mut(index - 1)
                .and_then(|dev| dev.device_mut().setup_widget(data))
        }
    }
}
