use crate::cpu::CPUInterface;

// ############### Const ###############
#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
enum FlagPositionOffset {
    Carry = 0,
    Zero,
    Interrupt,
    Decimal,
    Break,
    __Ignored,
    Overflow,
    Negative,
}

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(inline_js = "function alert_2(a, b) {alert(a + b)}")]
    fn alert_2(a: &str, b: &str);
}

fn on_unsupported_feature(name: &str) {
    alert_2("This feature is not supported: ", name);
}

//This should only be used when a feature such as decimal mode is not yet implemented,
//but might be in the future.
fn on_unimplemented_feature(name: &str) {
    alert_2("This feature is not implemented: ", name);
}

// ############### Flags ###############
#[inline]
fn set_flag(inter: &mut CPUInterface, flag: FlagPositionOffset) {
    inter.reg.p |= (1 << flag as u8);
}

#[inline]
fn clear_flag(inter: &mut CPUInterface, flag: FlagPositionOffset) {
    inter.reg.p &= !(1 << flag as u8);
}

#[inline]
fn set_flag_bool(inter: &mut CPUInterface, flag: FlagPositionOffset, value: bool) {
    if value {
        set_flag(inter, flag);
    } else {
        clear_flag(inter, flag);
    }
}

#[inline]
fn set_flag_is_zero(inter: &mut CPUInterface, value: u8) {
    set_flag_bool(inter, FlagPositionOffset::Zero, value == 0);
}

#[inline]
fn set_flag_is_negative(inter: &mut CPUInterface, value: u8) {
    // if the most significant bit is set, the number is considered negative
    inter.reg.p &= !(1 << 7);
    inter.reg.p |= (value & (1 << 7))
}

#[inline]
fn set_flag_is_overflow(inter: &mut CPUInterface, val_1: u16, val_2: u16, result: u16) {
    //https://stackoverflow.com/a/16861251
    let is_overflow = (!(val_1 ^ val_2)) & (val_1 ^ result) & 0x80 != 0;

    set_flag_bool(inter, FlagPositionOffset::Overflow, is_overflow);
}

// ############### Abstractions ###############

#[inline]
#[allow(non_snake_case)]
fn alu_add__flag_zn(inter: &mut CPUInterface, val_1: u8, val_2: u8) -> u8 {
    inter.reg.alu = val_1 + val_2;

    set_flag_is_zero(inter, inter.reg.alu);
    set_flag_is_negative(inter, inter.reg.alu);

    inter.reg.alu
}

#[inline]
#[allow(non_snake_case)]
fn alu_sub__flag_zn(inter: &mut CPUInterface, val_1: u8, val_2: u8) -> u8 {
    inter.reg.alu = val_1 - val_2;

    set_flag_is_zero(inter, inter.reg.alu);
    set_flag_is_negative(inter, inter.reg.alu);

    inter.reg.alu
}

fn stack_push(inter: &mut CPUInterface, value: u8) {
    inter.mem.set_addr(0x0100);
    inter.mem.set_addr_lo(inter.reg.s);

    inter.reg.s -= 1;

    inter.mem.set_data(value);
    inter.mem.write_at_addr();
}

fn stack_pull(inter: &mut CPUInterface) -> u8 {
    inter.reg.s += 1;

    inter.mem.set_addr(0x0100);
    inter.mem.set_addr_lo(inter.reg.s);

    inter.mem.read_at_addr()
}

// ############### Operations ###############

/* #######################  Load/Store Operations  ####################### */
pub fn lda(inter: &mut CPUInterface) {
    inter.reg.a = inter.mem.data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn ldx(inter: &mut CPUInterface) {
    inter.reg.x = inter.mem.data();

    set_flag_is_zero(inter, inter.reg.x);
    set_flag_is_negative(inter, inter.reg.x);
}

pub fn ldy(inter: &mut CPUInterface) {
    inter.reg.y = inter.mem.data();

    set_flag_is_zero(inter, inter.reg.y);
    set_flag_is_negative(inter, inter.reg.y);
}

pub fn sta(inter: &mut CPUInterface) {
    inter.mem.set_data(inter.reg.a);
}

pub fn sty(inter: &mut CPUInterface) {
    inter.mem.set_data(inter.reg.y);
}

pub fn stx(inter: &mut CPUInterface) {
    inter.mem.set_data(inter.reg.x);
}


/* #######################  Register Transfers  ####################### */
pub fn tax(inter: &mut CPUInterface) {
    inter.reg.x = inter.reg.a;

    set_flag_is_zero(inter, inter.reg.x);
    set_flag_is_negative(inter, inter.reg.x);
}

pub fn tay(inter: &mut CPUInterface) {
    inter.reg.y = inter.reg.a;

    set_flag_is_zero(inter, inter.reg.y);
    set_flag_is_negative(inter, inter.reg.y);
}

pub fn txa(inter: &mut CPUInterface) {
    inter.reg.a = inter.reg.x;

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn tya(inter: &mut CPUInterface) {
    inter.reg.a = inter.reg.y;

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}


/* #######################  Stack Operations  ####################### */
pub fn tsx(inter: &mut CPUInterface) {
    inter.reg.x = inter.reg.s;

    set_flag_is_zero(inter, inter.reg.x);
    set_flag_is_negative(inter, inter.reg.x);
}

pub fn txs(inter: &mut CPUInterface) {
    inter.reg.s = inter.reg.x;
}

pub fn pha(inter: &mut CPUInterface) {
    stack_push(inter, inter.reg.a);
}

pub fn php(inter: &mut CPUInterface) {
    stack_push(inter, inter.reg.p);
}

pub fn pla(inter: &mut CPUInterface) {
    let value = stack_pull(inter);
    inter.reg.a = value;

    set_flag_is_zero(inter, value);
    set_flag_is_negative(inter, value);
}

pub fn plp(inter: &mut CPUInterface) {
    inter.reg.p = stack_pull(inter);
}

/* #######################  Logical  ####################### */
pub fn and(inter: &mut CPUInterface) {
    inter.reg.a &= inter.mem.data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn eor(inter: &mut CPUInterface) {
    inter.reg.a ^= inter.mem.data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn ora(inter: &mut CPUInterface) {
    inter.reg.a |= inter.mem.data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn bit(inter: &mut CPUInterface) {
    let pattern = inter.reg.a;
    let value = inter.mem.data();

    let and_rs = value & pattern;
    set_flag_is_zero(inter, and_rs);

    let bit_6 = value & (1 << 6);
    set_flag_bool(inter, FlagPositionOffset::Overflow, bit_6 != 0);

    set_flag_is_negative(inter, value);
}

/* #######################  Arithmetic  ####################### */
pub fn adc(inter: &mut CPUInterface) {
    let val_1 = inter.reg.a as u16;
    let val_2 = inter.mem.data() as u16;

    let carry = ((inter.reg.p >> (FlagPositionOffset::Carry as u8)) & 0b1) as u16;

    let result = val_1 + val_2 + carry;

    inter.reg.alu = result as u8;
    inter.reg.a = inter.reg.alu;

    set_flag_is_zero(inter, inter.reg.alu);
    set_flag_is_negative(inter, inter.reg.alu);

    set_flag_bool(inter, FlagPositionOffset::Carry, (result & 0xFF00) != 0);
    set_flag_is_overflow(inter, val_1, val_2, result);
}

pub fn sbc(inter: &mut CPUInterface) {
    let val_1 = inter.reg.a as u16;
    let val_2 = inter.mem.data() as u16;

    let carry = ((inter.reg.p >> (FlagPositionOffset::Carry as u8)) & 0b1) as u16;

    let result = val_1 - val_2 - (1 - carry);

    inter.reg.alu = result as u8;
    inter.reg.a = inter.reg.alu;

    set_flag_is_zero(inter, inter.reg.alu);
    set_flag_is_negative(inter, inter.reg.alu);

    set_flag_bool(inter, FlagPositionOffset::Carry, (result & 0xFF00) == 0);
    set_flag_is_overflow(inter, val_1, val_2, result);
}

fn __generic_cmp(inter: &mut CPUInterface, reg: u8) {
    let val = inter.mem.data();

    set_flag_bool(inter, FlagPositionOffset::Carry, reg >= val);

    set_flag_bool(inter, FlagPositionOffset::Zero, reg == val);

    set_flag_is_negative(inter, reg - val);
}

pub fn cmp(inter: &mut CPUInterface) {
    __generic_cmp(inter, inter.reg.a);
}


pub fn cpx(inter: &mut CPUInterface) {
    __generic_cmp(inter, inter.reg.x);
}


pub fn cpy(inter: &mut CPUInterface) {
    __generic_cmp(inter, inter.reg.y);
}

/* #######################  Increments & Decrements  ####################### */
pub fn inc(inter: &mut CPUInterface) {
    let new_value = alu_add__flag_zn(inter, inter.mem.data(), 1);

    inter.mem.set_data(new_value);
}

pub fn inx(inter: &mut CPUInterface) {
    inter.reg.x = alu_add__flag_zn(inter, inter.reg.x, 1);
}

pub fn iny(inter: &mut CPUInterface) {
    inter.reg.y = alu_add__flag_zn(inter, inter.reg.y, 1);
}

pub fn dec(inter: &mut CPUInterface) {
    let new_value = alu_sub__flag_zn(inter, inter.mem.data(), 1);

    inter.mem.set_data(new_value);
}

pub fn dex(inter: &mut CPUInterface) {
    inter.reg.x = alu_sub__flag_zn(inter, inter.reg.x, 1);
}

pub fn dey(inter: &mut CPUInterface) {
    inter.reg.y = alu_sub__flag_zn(inter, inter.reg.y, 1);
}

/* #######################  Shifts  ####################### */
pub fn asl(inter: &mut CPUInterface) {
    let target = inter.target_mut();

    let old = *target;

    *target <<= 1;

    let target_val = *target; //copied as inter and target are both muts.
    set_flag_is_zero(inter, target_val);
    set_flag_is_negative(inter, target_val);

    set_flag_bool(inter, FlagPositionOffset::Carry, (old & (1 << 7)) != 0);
}

pub fn lsr(inter: &mut CPUInterface) {
    let target = inter.target_mut();

    let old = *target;

    *target >>= 1;

    let target_val = *target;  //copied as inter and target are both muts.
    set_flag_is_zero(inter, target_val);
    set_flag_is_negative(inter, target_val);

    set_flag_bool(inter, FlagPositionOffset::Carry, (old & 1) != 0);
}

pub fn rol(inter: &mut CPUInterface) {
    let old_carry = (inter.reg.p >> FlagPositionOffset::Carry as u8) & 0b1;

    asl(inter);

    let target = inter.target_mut();
    *target |= old_carry;
}

pub fn ror(inter: &mut CPUInterface) {
    let old_carry = (inter.reg.p >> FlagPositionOffset::Carry as u8) & 0b1;

    lsr(inter);

    let target = inter.target_mut();
    *target |= old_carry << 7;
}


/* #######################  Jumps & Calls  ####################### */
pub fn jmp(inter: &mut CPUInterface) {
    let mut new_addr: u16;

    new_addr = inter.reg.itr as u16;
    new_addr |= (inter.mem.data() as u16) << 8;

    inter.reg.pc = new_addr;
}

//TODO: impl
pub fn jsr(inter: &mut CPUInterface) {}


//TODO: impl
pub fn rts(inter: &mut CPUInterface) {}


/* #######################  Branches  ####################### */
#[inline]
fn __generic_branch_if_set(inter: &mut CPUInterface, flag: FlagPositionOffset) {
    if (inter.reg.p & (1 << flag as u8)) != 0 {
        inter.reg.itr = 1;
    }
}

#[inline]
fn __generic_branch_if_not_set(inter: &mut CPUInterface, flag: FlagPositionOffset) {
    if (inter.reg.p & (1 << flag as u8)) == 0 {
        inter.reg.itr = 1;
    }
}

pub fn bcc(inter: &mut CPUInterface) {
    __generic_branch_if_not_set(inter, FlagPositionOffset::Carry);
}


pub fn bcs(inter: &mut CPUInterface) {
    __generic_branch_if_set(inter, FlagPositionOffset::Carry);
}


pub fn beq(inter: &mut CPUInterface) {
    __generic_branch_if_set(inter, FlagPositionOffset::Zero);
}


pub fn bmi(inter: &mut CPUInterface) {
    __generic_branch_if_set(inter, FlagPositionOffset::Negative);
}

pub fn bne(inter: &mut CPUInterface) {
    __generic_branch_if_not_set(inter, FlagPositionOffset::Zero);
}

pub fn bpl(inter: &mut CPUInterface) {
    __generic_branch_if_not_set(inter, FlagPositionOffset::Negative);
}

pub fn bvc(inter: &mut CPUInterface) {
    __generic_branch_if_not_set(inter, FlagPositionOffset::Overflow);
}

pub fn bvs(inter: &mut CPUInterface) {
    __generic_branch_if_set(inter, FlagPositionOffset::Overflow);
}


/* #######################  Status Flag Changes  ####################### */
pub fn clc(inter: &mut CPUInterface) {
    clear_flag(inter, FlagPositionOffset::Carry);
}

pub fn cld(inter: &mut CPUInterface) {
    on_unimplemented_feature("Decimal Mode");
}

pub fn cli(inter: &mut CPUInterface) {
    on_unsupported_feature("Interrupts");
}

pub fn clv(inter: &mut CPUInterface) {
    clear_flag(inter, FlagPositionOffset::Overflow);
}

pub fn sec(inter: &mut CPUInterface) {
    set_flag(inter, FlagPositionOffset::Carry);
}

pub fn sed(_inter: &mut CPUInterface) {
    on_unimplemented_feature("Decimal Mode");
}

pub fn sei(inter: &mut CPUInterface) {
    on_unsupported_feature("Interrupts");
}

/* #######################  System Functions  ####################### */
//TODO: impl
pub fn brk(inter: &mut CPUInterface) {}

pub fn nop(_inter: &mut CPUInterface) {
    //do_nothing();
}

//TODO: impl
pub fn rti(inter: &mut CPUInterface) {}



