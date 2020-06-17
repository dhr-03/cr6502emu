use super::super::{InstructionFn, AddressingModifier};

use crate::cpu::CPUInterface;

//pub fn x_1(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {}

//pub fn {mode}_[1-9](...)

pub fn imp(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    op_fn(inter);
}

pub fn imm(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    inter.mem.set_addr(inter.reg.pc);
    inter.mem.read_at_addr();
    inter.reg.pc += 1;

    op_fn(inter);
}

pub fn a__(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    inter.target_is_mem = false;

    op_fn(inter);
}