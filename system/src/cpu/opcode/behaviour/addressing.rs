use super::super::AddressingFn;
use super::addressing_internal::*;

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

//TODO: implement, rmw
pub const ZPX: [AddressingFn; 0] = [];

//TODO: implement
pub const ZPY: [AddressingFn; 0] = [];

//TODO: implement
pub const REL: [AddressingFn; 0] = [];

//TODO: implement
pub const ABS: [AddressingFn; 0] = [];

//TODO: implement, rmw
pub const ABX: [AddressingFn; 0] = [];

//TODO: implement
pub const ABY: [AddressingFn; 0] = [];

//TODO: implement
pub const IND: [AddressingFn; 0] = [];

//TODO: implement
pub const IDX: [AddressingFn; 0] = [];

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
