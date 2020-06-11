use wasm_bindgen::prelude::wasm_bindgen;

use crate::cpu::CPU;

use super::MemManager;

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

    pub fn reset(&mut self) {
        unimplemented!();
    }

    pub fn add_device(&mut self, device: u32, start: u16, end: u16) {
        unimplemented!();
    }

    pub fn remove_device(&mut self, index: usize) {
        unimplemented!();
    }

    pub fn tmp_to_str(&self) -> String {
        format!("{}\nbus data: {}\nbus addr: {}",
                self.cpu.tmp_to_str(),
                self.mem.get_data(),
                self.mem.addr()
        )
    }
}