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

//This should be only be used when a feature such as decimal mode is not yet implemented,
//but might be in the future.
fn on_unimplemented_feature(name: &str) {
    alert_2("This feature is not implemented: ", name);
}

// ############### Tools ###############

pub fn inc_pc(inter: &mut CPUInterface) {
    inter.reg.pc += 1;
}
pub fn dec_pc(inter: &mut CPUInterface) {
    inter.reg.pc -= 1;
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
fn set_flag_is_zero(inter: &mut CPUInterface, value: u8) {
    if value == 0 {
        set_flag(inter, FlagPositionOffset::Zero);
    } else {
        clear_flag(inter, FlagPositionOffset::Zero);
    }
}

#[inline]
fn set_flag_is_negative(inter: &mut CPUInterface, value: u8) {
    // if the most significant bit is set, the number is considered negative
    inter.reg.p |= (value & (1 << 7))
}

//TODO
#[inline]
fn set_flag_is_carry(inter: &mut CPUInterface, val_1: u8, val_2: u8) {}

//TODO
#[inline]
fn set_flag_is_overflow(inter: &mut CPUInterface, val_1: u8, val_2: u8) {}

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

//############### Transfers ###############
pub fn pc_bdata(inter: &mut CPUInterface) {
    inter.mem.set_addr(
        inter.reg.pc
    );

    inter.mem.read_at_addr();
}


/* #######################  Load/Store Operations  ####################### */
pub fn lda(inter: &mut CPUInterface) {
    inter.reg.a = inter.mem.get_data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn ldx(inter: &mut CPUInterface) {
    inter.reg.x = inter.mem.get_data();
}

pub fn ldy(inter: &mut CPUInterface) {
    inter.reg.y = inter.mem.get_data();
}

//TODO: impl
pub fn sta(inter: &mut CPUInterface) {}


//TODO: impl
pub fn sty(inter: &mut CPUInterface) {}


//TODO: impl
pub fn stx(inter: &mut CPUInterface) {}


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

//TODO: impl
pub fn pha(inter: &mut CPUInterface) {}


//TODO: impl
pub fn php(inter: &mut CPUInterface) {}


//TODO: impl
pub fn pla(inter: &mut CPUInterface) {}


//TODO: impl
pub fn plp(inter: &mut CPUInterface) {}


/* #######################  Logical  ####################### */
pub fn and(inter: &mut CPUInterface) {
    inter.reg.a &= inter.mem.get_data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn eor(inter: &mut CPUInterface) {
    inter.reg.a ^= inter.mem.get_data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn ora(inter: &mut CPUInterface) {
    inter.reg.a |= inter.mem.get_data();

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);
}

pub fn bit(inter: &mut CPUInterface) {}

/* #######################  Arithmetic  ####################### */
pub fn adc(inter: &mut CPUInterface) {
    let val_1 = inter.reg.a;
    let val_2 = inter.mem.get_data();

    inter.reg.a = alu_add__flag_zn(inter, val_1, val_2);

    set_flag_is_carry(inter, val_1, val_2);
    set_flag_is_overflow(inter, val_1, val_2);
}

pub fn sbc(inter: &mut CPUInterface) {
    let val_1 = inter.reg.a;
    let val_2 = inter.mem.get_data();

    inter.reg.a = alu_sub__flag_zn(inter, val_1, val_2);

    set_flag_is_carry(inter, val_1, val_2);
    set_flag_is_overflow(inter, val_1, val_2);
}

fn __generic_cmp(inter: &mut CPUInterface, reg: u8) {
    let val = inter.mem.get_data();

    //TODO: alu sbc?

    if reg >= val {
        set_flag(inter, FlagPositionOffset::Carry);
    } else {
        clear_flag(inter, FlagPositionOffset::Carry);
    }

    if reg == val {
        set_flag(inter, FlagPositionOffset::Zero);

        //TODO: n ok?
        clear_flag(inter, FlagPositionOffset::Negative);
    } else {
        clear_flag(inter, FlagPositionOffset::Zero);

        set_flag_is_negative(inter, reg);
    }
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
//TODO: impl
pub fn inc(inter: &mut CPUInterface) {}

pub fn inx(inter: &mut CPUInterface) {
    inter.reg.x = alu_add__flag_zn(inter, inter.reg.x, 1)
}

pub fn iny(inter: &mut CPUInterface) {
    inter.reg.y = alu_add__flag_zn(inter, inter.reg.y, 1);
}

//TODO: impl
pub fn dec(inter: &mut CPUInterface) {}

pub fn dex(inter: &mut CPUInterface) {
    inter.reg.x = alu_sub__flag_zn(inter, inter.reg.x, 1);
}

pub fn dey(inter: &mut CPUInterface) {
    inter.reg.y = alu_sub__flag_zn(inter, inter.reg.y, 1);
}

/* #######################  Shifts  ####################### */
pub fn asl(inter: &mut CPUInterface) {
    let old = inter.reg.a;

    inter.reg.a <<= 1;

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);

    if (old & (1 << 7)) != 0 { //set to old bit
        set_flag(inter, FlagPositionOffset::Carry);
    } else {
        clear_flag(inter, FlagPositionOffset::Carry);
    }
}

pub fn lsr(inter: &mut CPUInterface) {
    let old = inter.reg.a;

    inter.reg.a >>= 1;

    set_flag_is_zero(inter, inter.reg.a);
    set_flag_is_negative(inter, inter.reg.a);

    if (old & 1) != 0 { //set to old bit
        set_flag(inter, FlagPositionOffset::Carry);
    } else {
        clear_flag(inter, FlagPositionOffset::Carry);
    }
}

//TODO: impl
pub fn rol(inter: &mut CPUInterface) {}

//TODO: impl
pub fn ror(inter: &mut CPUInterface) {}


/* #######################  Jumps & Calls  ####################### */
//TODO: impl
pub fn jmp(inter: &mut CPUInterface) {}


//TODO: impl
pub fn jsr(inter: &mut CPUInterface) {}


//TODO: impl
pub fn rts(inter: &mut CPUInterface) {}


/* #######################  Branches  ####################### */
//TODO: impl
pub fn bcc(inter: &mut CPUInterface) {}


//TODO: impl
pub fn bcs(inter: &mut CPUInterface) {}


//TODO: impl
pub fn beq(inter: &mut CPUInterface) {}


//TODO: impl
pub fn bmi(inter: &mut CPUInterface) {}


//TODO: impl
pub fn bne(inter: &mut CPUInterface) {}


//TODO: impl
pub fn bpl(inter: &mut CPUInterface) {}


//TODO: impl
pub fn bvc(inter: &mut CPUInterface) {}


//TODO: impl
pub fn bvs(inter: &mut CPUInterface) {}


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

pub fn sed(inter: &mut CPUInterface) {
    on_unimplemented_feature("Decimal Mode");
}

pub fn sei(inter: &mut CPUInterface) {
    on_unsupported_feature("Interrupts");
}

/* #######################  System Functions  ####################### */
//TODO: impl
pub fn brk(inter: &mut CPUInterface) {}

pub fn nop(inter: &mut CPUInterface) {
    //do_nothing();
}

//TODO: impl
pub fn rti(inter: &mut CPUInterface) {}



