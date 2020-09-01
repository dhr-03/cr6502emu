use super::super::AddressingFn;
use super::internal::*;

pub const IMP: [AddressingFn; 1] = [
    imp
];

pub const A__: [AddressingFn; 1] = [
    a__
];

pub const IMM: [AddressingFn; 1] = [
    imm
];

pub const ZP_: [AddressingFn; 2] = [
    zp_1,
    zp_2,
];

pub const ZPX: [AddressingFn; 3] = [
    zpx_1,
    zpx_2,
    zpx_3,
];

pub const ZPY: [AddressingFn; 3] = [
    zpy_1,
    zpy_2,
    zpy_3,
];

pub const REL: [AddressingFn; 1] = [
    rel
];

pub const ABS: [AddressingFn; 3] = [
    abs_1,
    abs_2,
    abs_3,
];

pub const ABX: [AddressingFn; 3] = [
    abx_1,
    abx_2,
    abx_3,
];

pub const ABY: [AddressingFn; 3] = [
    aby_1,
    aby_2,
    aby_3,
];

//TODO: implement
pub const IND: [AddressingFn; 0] = [];

pub const IDX: [AddressingFn; 4] = [
    idx_1,
    idx_2,
    idx_3,
    idx_4,
];

//TODO: implement
pub const IXD: [AddressingFn; 0] = [];

//TODO: implement
pub const ASB: [AddressingFn; 0] = []; //absolute JSR

//TODO: implement
pub const SPH: [AddressingFn; 0] = []; //stack push

//TODO: implement
pub const SPL: [AddressingFn; 0] = []; //stack pull

//TODO: implement
pub const SRT: [AddressingFn; 0] = []; //stack RTS

//TODO: implement
pub const SBK: [AddressingFn; 0] = []; //stack BRK
