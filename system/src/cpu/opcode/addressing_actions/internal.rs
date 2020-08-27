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


// A few functions do exactly the same thing, but with different names.
// I'm doing this simplicity and to avoid bugs in the future,
// surely the compiler will detect those and "unify" them.

// ############### Functions ###############

pub fn waste_cycle(_inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {}

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
fn __zp_common(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    if op_mod.has_read() {
        inter.mem.read_at_addr();
    }

    if let AddressingModifier::RMW = op_mod {
        *inter.next_cycle = Some(zp_extra_1);
    } else {
        op_fn(inter);

        if let AddressingModifier::Write = op_mod {
            inter.mem.write_at_addr();
        }
    }
}

pub fn zp_1(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);
}

pub fn zp_2(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    inter.mem.set_addr(inter.mem.data() as u16);

    __zp_common(inter, op_fn, op_mod);
}

fn zp_extra_1(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    op_fn(inter);

    *inter.next_cycle = Some(zp_extra_2);
}

fn zp_extra_2(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    inter.mem.write_at_addr();
}

// ####### Zero Page X #######
pub use zp_1 as zpx_1;

pub use waste_cycle as zpx_2;

pub fn zpx_3(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    inter.mem.set_addr(0);

    inter.mem.set_addr_lo(
        inter.mem.data() + inter.reg.x //wrapping add
    );

    __zp_common(inter, op_fn, op_mod);
}

// ####### Zero Page Y #######
pub use zp_1 as zpy_1;

pub use waste_cycle as zpy_2;

pub fn zpy_3(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    inter.mem.set_addr(0);

    inter.mem.set_addr_lo(
        inter.mem.data() + inter.reg.y //wrapping add
    );

    if let AddressingModifier::Read = op_mod {
        inter.mem.read_at_addr();
    }

    op_fn(inter);

    if let AddressingModifier::Write = op_mod {
        inter.mem.write_at_addr();
    }
}


// ####### Relative #######
pub fn rel(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);

    // We don't have any direct way of communicating with the opcode fn.
    // If the fn changes itr, a branch must be taken.
    inter.reg.itr = 0;

    op_fn(inter);

    if inter.reg.itr != 0 {
        *inter.next_cycle = Some(rel_extra_1); //take branch
    }
}

fn rel_extra_1(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    let offset = inter.mem.data();

    // the cpu can only work with 8bit numbers, if the op carries, it needs a extra cycle.
    if (std::u8::MAX - (inter.reg.pc as u8)) < offset {
        *inter.next_cycle = Some(rel_extra_2);
    } else {
        rel_extra_2(inter, _op_fn, _op_mod);
    }
}

fn rel_extra_2(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    let offset = inter.mem.data();
    let offset_signed: i16 = (offset as i8).into();

    let new_pc = (inter.reg.pc as i16) + offset_signed;

    inter.reg.pc = new_pc as u16;
}

// ####### Absolute #######
fn __abs_common(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    if op_mod.has_read() {
        inter.mem.read_at_addr();
    }

    if let AddressingModifier::RMW = op_mod {
        *inter.next_cycle = Some(abs_extra_1);
    } else {
        op_fn(inter);

        if let AddressingModifier::Write = op_mod {
            inter.mem.write_at_addr();
        }
    }
}

pub fn abs_1(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);

    inter.reg.itr = inter.mem.data()
}

pub fn abs_2(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);
}

pub fn abs_3(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    inter.mem.set_addr_hi(inter.mem.data());
    inter.mem.set_addr_lo(inter.reg.itr);

    __abs_common(inter, op_fn, op_mod);
}

fn abs_extra_1(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    op_fn(inter);

    *inter.next_cycle = Some(abs_extra_2);
}

fn abs_extra_2(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    inter.mem.write_at_addr();
}

// ####### Absolute X #######
#[inline(always)]
fn __abxy_calculate_addr(inter:  &mut CPUInterface, reg: u8) -> u16 {
    let mut new_addr: u16;

    new_addr = inter.reg.itr as u16;
    new_addr |= (inter.mem.data() as u16) << 8;
    new_addr += reg as u16;

    new_addr
}

pub use abs_1 as abx_1;

pub use abs_2 as abx_2;

pub fn abx_3(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    let addr_lo = inter.reg.itr;

    if op_mod.is_write() || (std::u8::MAX - addr_lo) < inter.reg.x {
        *inter.next_cycle = Some(abx_extra_1);
    } else {
        abx_extra_1(inter, op_fn, op_mod);
    }
}

fn abx_extra_1(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    let new_addr = __abxy_calculate_addr(inter, inter.reg.x);
    inter.mem.set_addr(new_addr);

    __abs_common(inter, op_fn, op_mod);
}

// ####### Absolute Y #######
pub use abs_1 as aby_1;

pub use abs_2 as aby_2;

pub fn aby_3(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    let addr_lo = inter.reg.itr;

    if op_mod.is_write() || (std::u8::MAX - addr_lo) < inter.reg.y {
        *inter.next_cycle = Some(aby_extra_1);
    } else {
        aby_extra_1(inter, op_fn, op_mod);
    }
}

fn aby_extra_1(inter: &mut CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier) {
    let new_addr = __abxy_calculate_addr(inter, inter.reg.y);
    inter.mem.set_addr(new_addr);

    if let AddressingModifier::Read = op_mod {
        inter.mem.read_at_addr();
    }

    op_fn(inter);

    if let AddressingModifier::Write = op_mod {
        inter.mem.write_at_addr();
    }
}

// ####### ASB (ABS JUMP) #######
pub fn asb_1(inter: &mut CPUInterface, _op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);

    inter.reg.itr = inter.mem.data()
}

pub fn asb_2(inter: &mut CPUInterface, op_fn: InstructionFn, _op_mod: AddressingModifier) {
    read_at_pc_inc(inter);

    op_fn(inter);
}

// #######  #######
// #######  #######








