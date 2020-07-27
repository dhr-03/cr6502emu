use super::{CPUInterface, CurrentOpcode};
use super::super::{
    register::RegisterContainer,
    opcode::AddressingFn,
};

use crate::system::MemManager;

pub struct CPU {
    reg: RegisterContainer,

    opcode: CurrentOpcode,

    extra_cycle: Option<AddressingFn>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg: RegisterContainer::new(),

            opcode: CurrentOpcode::new(),

            extra_cycle: None,
        }
    }

    pub fn tick(&mut self, mem_ref: &mut MemManager) {
        let mut inter = CPUInterface {
            mem: mem_ref,

            reg: &mut self.reg,

            next_cycle: &mut self.extra_cycle,

            target_is_mem: true,
        };

        self.opcode.execute(&mut inter);
    }

    pub fn reset(&mut self) {
        self.reg.reset();
        self.opcode.force_is_done();
        self.extra_cycle = None;
    }
}
