use super::{CPUInterface, CPUOperationManager};
use super::super::{
    register::RegisterContainer,
    opcode::AddressingFn,
};

use crate::system::MemManager;

use crate::dev::{DeviceTrait, DeviceId};

use js_sys::Map;

pub struct CPU {
    reg: RegisterContainer,

    opcode: CPUOperationManager,

    extra_cycle: Option<AddressingFn>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: RegisterContainer::new(),

            opcode: CPUOperationManager::new(),

            extra_cycle: None,
        }
    }

    pub fn tick_with_mem(&mut self, mem_ref: &mut MemManager) {
        let mut inter = CPUInterface {
            mem: mem_ref,

            reg: &mut self.reg,

            next_cycle: &mut self.extra_cycle,

            target_is_mem: true,
        };

        self.opcode.execute(&mut inter);
    }
}

impl DeviceTrait for CPU {
    #[cfg(debug_assertions)]
    fn tick(&mut self) {
        panic!("Disabled method. Use CPU::tick_with_mem instead.")
    }

    fn reset_system(&mut self) {
        self.reg.reset();
        self.opcode.force_is_done();
        self.extra_cycle = None;
    }

    fn reset_hard(&mut self) {
        self.reset_system();
    }

    fn update_widget(&self) -> Option<Map> {
        todo!()
    }

    fn device_id(&self) -> DeviceId {
        DeviceId::CPU
    }
}
