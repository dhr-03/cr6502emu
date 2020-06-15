use super::super::{CycleRef, AddressingCycle};
use super::operations_internal as operation;

pub const IMP: [&AddressingCycle; 1] = [
    &[
        CycleRef::OpHardRef(0),
    ]
];

pub const A__: [&AddressingCycle; 1] = [
    &[
        CycleRef::Fn(operation::set_target_a),
        CycleRef::OpHardRef(0),
    ]
];

pub const IMM: [&AddressingCycle; 1] = [
    &[
        CycleRef::Fn(operation::read_at_pc),
        CycleRef::OpHardRef(0),
    ]
];


//TODO: implement, rmw
pub const ZP_: [&AddressingCycle; 0] = [];

//TODO: implement, rmw
pub const ZPX: [&AddressingCycle; 0] = [];

//TODO: implement
pub const ZPY: [&AddressingCycle; 0] = [];

//TODO: implement
pub const REL: [&AddressingCycle; 0] = [];

//TODO: implement
pub const ABS: [&AddressingCycle; 0] = [];

//TODO: implement, rmw
pub const ABX: [&AddressingCycle; 0] = [];

//TODO: implement
pub const ABY: [&AddressingCycle; 0] = [];

//TODO: implement
pub const IND: [&AddressingCycle; 0] = [];

//TODO: implement
pub const IDX: [&AddressingCycle; 0] = [];

//TODO: implement
pub const IXD: [&AddressingCycle; 0] = [];

//TODO: implement
pub const ASB: [&AddressingCycle; 0] = []; //absolute JSR

//TODO: implement
pub const SPH: [&AddressingCycle; 0] = []; //stack push

//TODO: implement
pub const SPL: [&AddressingCycle; 0] = []; //stack pull

//TODO: implement
pub const SRT: [&AddressingCycle; 0] = []; //stack RTS

//TODO: implement
pub const SBK: [&AddressingCycle; 0] = []; //stack BRK
