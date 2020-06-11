use crate::cpu::{
    register::RegisterContainer,
    opcode::AddressingCycleRef,
};

use crate::system::MemManager;

//As long as this is borrowed as a non mut, there is no need to add getters and setters
pub struct CPUInterface<'a> {
    pub mem: &'a mut MemManager,

    pub reg: &'a mut RegisterContainer,

    pub next_cycle: &'a mut Option<AddressingCycleRef>,
}