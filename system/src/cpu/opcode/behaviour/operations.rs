use super::super::InstructionFn;
use super::operations_internal::*;

/* #######################  Load/Store Operations  ####################### */
pub const LDA: [InstructionFn; 1] = [
    lda
];

pub const LDX: [InstructionFn; 1] = [
    ldx
];

pub const LDY: [InstructionFn; 1] = [
    ldy
];

//TODO: impl
pub const STA: [InstructionFn; 0] = [];

//TODO: impl
pub const STY: [InstructionFn; 0] = [];

//TODO: impl
pub const STX: [InstructionFn; 0] = [];

/* #######################  Register Transfers  ####################### */

pub const TAX: [InstructionFn; 1] = [
    tax
];

pub const TAY: [InstructionFn; 1] = [
    tay
];

pub const TXA: [InstructionFn; 1] = [
    txa
];

pub const TYA: [InstructionFn; 1] = [
    tya
];

/* #######################  Stack Operations  ####################### */

pub const TSX: [InstructionFn; 1] = [
    tsx
];

pub const TXS: [InstructionFn; 1] = [
    txs
];

//TODO: impl
pub const PHA: [InstructionFn; 0] = [];

//TODO: impl
pub const PHP: [InstructionFn; 0] = [];

//TODO: impl
pub const PLA: [InstructionFn; 0] = [];

//TODO: impl
pub const PLP: [InstructionFn; 0] = [];

/* #######################  Logical  ####################### */

pub const AND: [InstructionFn; 1] = [
    and
];

pub const EOR: [InstructionFn; 1] = [
    eor
];

pub const ORA: [InstructionFn; 1] = [
    ora
];

pub const BIT: [InstructionFn; 1] = [
    bit
];

/* #######################  Arithmetic  ####################### */

pub const ADC: [InstructionFn; 1] = [
    adc
];

pub const SBC: [InstructionFn; 1] = [
    sbc
];

pub const CMP: [InstructionFn; 1] = [
    cmp
];

pub const CPX: [InstructionFn; 1] = [
    cpx
];

pub const CPY: [InstructionFn; 1] = [
    cpy
];


/* #######################  Increments & Decrements  ####################### */


//TODO: impl
pub const INC: [InstructionFn; 0] = [];

pub const INX: [InstructionFn; 1] = [
    inx
];

pub const INY: [InstructionFn; 1] = [
    iny
];

//TODO: impl
pub const DEC: [InstructionFn; 0] = [];

pub const DEX: [InstructionFn; 1] = [
    dex
];

pub const DEY: [InstructionFn; 1] = [
    dey
];

/* #######################  Shifts  ####################### */

pub const ASL: [InstructionFn; 1] = [
    asl
];

pub const LSR: [InstructionFn; 1] = [
    lsr
];


//TODO: impl
pub const ROL: [InstructionFn; 0] = [];

//TODO: impl
pub const ROR: [InstructionFn; 0] = [];

/* #######################  Jumps & Calls  ####################### */


//TODO: impl
pub const JMP: [InstructionFn; 0] = [];

//TODO: impl
pub const JSR: [InstructionFn; 0] = [];

//TODO: impl
pub const RTS: [InstructionFn; 0] = [];

/* #######################  Branches  ####################### */


//TODO: impl
pub const BCC: [InstructionFn; 0] = [];

//TODO: impl
pub const BCS: [InstructionFn; 0] = [];

//TODO: impl
pub const BEQ: [InstructionFn; 0] = [];

//TODO: impl
pub const BMI: [InstructionFn; 0] = [];

//TODO: impl
pub const BNE: [InstructionFn; 0] = [];

//TODO: impl
pub const BPL: [InstructionFn; 0] = [];

//TODO: impl
pub const BVC: [InstructionFn; 0] = [];

//TODO: impl
pub const BVS: [InstructionFn; 0] = [];

/* #######################  Status Flag Changes  ####################### */

pub const CLC: [InstructionFn; 1] = [
    clc
];

pub const CLD: [InstructionFn; 1] = [
    cld
];

pub const CLI: [InstructionFn; 1] = [
    cli
];

pub const CLV: [InstructionFn; 1] = [
    clv
];

pub const SEC: [InstructionFn; 1] = [
    sec
];

pub const SED: [InstructionFn; 1] = [
    sed
];

pub const SEI: [InstructionFn; 1] = [
    sei
];
/* #######################  System Functions  ####################### */


//TODO: impl
pub const BRK: [InstructionFn; 0] = [];

pub const NOP: [InstructionFn; 1] = [
    nop
];

//TODO: impl
pub const RTI: [InstructionFn; 0] = [];
