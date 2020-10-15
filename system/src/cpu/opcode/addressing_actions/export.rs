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

pub const IND: [AddressingFn; 5] = [
    ind_1,
    ind_2,
    ind_3,
    ind_4,
    ind_5,
];

pub const IDX: [AddressingFn; 4] = [
    idx_1,
    idx_2,
    idx_3,
    idx_4,
];

pub const IXD: [AddressingFn; 4] = [
    ixd_1,
    ixd_2,
    ixd_3,
    ixd_4,
];

pub const ASB: [AddressingFn; 5] = [ //absolute JSR
    asb_1,
    asb_2,
    asb_3,
    asb_4,
    asb_5,
];

pub const SPH: [AddressingFn; 2] = [ //stack push
    sph_1,
    sph_2,
];

pub const SPL: [AddressingFn; 3] = [ //stack pull
    spl_1,
    spl_2,
    spl_3,
];

pub const SRT: [AddressingFn; 5] = [//stack RTS
    srt_1,
    srt_2,
    srt_3,
    srt_4,
    srt_5,
];

//TODO: implement
pub const SBK: [AddressingFn; 0] = []; //stack BRK
