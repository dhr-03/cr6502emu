use super::super::{InstructionFn, AddressingModifier};

use crate::cpu::CPUInterface;

//pub fn x_1(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {}

//pub fn {mode}_[1-9](...)

// ############### Abstractions ###############
#[inline]
fn read_at_pc_inc(inter: &mut CPUInterface) {
    inter.mem.set_addr(inter.reg.pc);
    inter.mem.read_at_addr();
    inter.reg.pc += 1;
}


// ############### Functions ###############


// ####### Implied #######
pub fn imp(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    op_fn(inter);
}

// ####### Immediate #######
pub fn imm(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);

    op_fn(inter);
}

// ####### Accumulator #######
pub fn a__(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    inter.target_is_mem = false;

    op_fn(inter);
}

// ####### Zero Page #######
pub fn zp_1(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);
}

pub fn zp_2(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    inter.mem.set_addr_hi(0);
    inter.mem.set_addr_lo(inter.mem.data())
}

pub fn zp_3(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    if op_mod.is_read() {
        inter.mem.read_at_addr();
    }

    op_fn(inter);

    if op_mod.is_write() {
        inter.mem.write_at_addr()
    }
}

// #######  #######
// #######  #######
// #######  #######
// #######  #######








