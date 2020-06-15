use crate::cpu::{
    register::RegisterContainer,
    opcode::AddressingCycleRef,
};

use crate::system::MemManager;

pub struct CPUInterface<'a> {
    pub mem: &'a mut MemManager,

    pub reg: &'a mut RegisterContainer,

    pub next_cycle: &'a mut Option<AddressingCycleRef>,

    pub target_is_mem: bool,
}

impl CPUInterface<'_> {
    pub fn target_mut(&mut self) -> &mut u8 {
        if self.target_is_mem {
            self.mem.data_ref_mut()
        } else {
            &mut self.reg.a
        }
    }

    pub fn target_mut_read(&mut self) -> &mut u8 {
        self.mem.read_at_addr();

        self.mem.data_ref_mut()
    }
}
