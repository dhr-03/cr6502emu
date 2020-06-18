use super::super::{AnnotatedOpcode, AddressingModifier};
use super::operations_internal::*;

/* #######################  Load/Store Operations  ####################### */
pub const LDA: AnnotatedOpcode = (lda, AddressingModifier::Read);

pub const LDX: AnnotatedOpcode = (ldx, AddressingModifier::Read);

pub const LDY: AnnotatedOpcode = (ldy, AddressingModifier::Read);

pub const STA: AnnotatedOpcode = (sta, AddressingModifier::Write);

pub const STY: AnnotatedOpcode = (sty, AddressingModifier::Write);

pub const STX: AnnotatedOpcode = (stx, AddressingModifier::Write);


/* #######################  Register Transfers  ####################### */

pub const TAX: AnnotatedOpcode = (tax, AddressingModifier::None);

pub const TAY: AnnotatedOpcode = (tay, AddressingModifier::None);

pub const TXA: AnnotatedOpcode = (txa, AddressingModifier::None);

pub const TYA: AnnotatedOpcode = (tya, AddressingModifier::None);


/* #######################  Stack Operations  ####################### */

pub const TSX: AnnotatedOpcode = (tsx, AddressingModifier::PlaceHolder);

pub const TXS: AnnotatedOpcode = (txs, AddressingModifier::PlaceHolder);

pub const PHA: AnnotatedOpcode = (pha, AddressingModifier::PlaceHolder);

pub const PHP: AnnotatedOpcode = (php, AddressingModifier::PlaceHolder);

pub const PLA: AnnotatedOpcode = (pla, AddressingModifier::PlaceHolder);

pub const PLP: AnnotatedOpcode = (plp, AddressingModifier::PlaceHolder);


/* #######################  Logical  ####################### */

pub const AND: AnnotatedOpcode = (and, AddressingModifier::Read);

pub const EOR: AnnotatedOpcode = (eor, AddressingModifier::Read);

pub const ORA: AnnotatedOpcode = (ora, AddressingModifier::Read);

pub const BIT: AnnotatedOpcode = (bit, AddressingModifier::Read);


/* #######################  Arithmetic  ####################### */

pub const ADC: AnnotatedOpcode = (adc, AddressingModifier::Read);

pub const SBC: AnnotatedOpcode = (sbc, AddressingModifier::Read);

pub const CMP: AnnotatedOpcode = (cmp, AddressingModifier::Read);

pub const CPX: AnnotatedOpcode = (cpx, AddressingModifier::Read);

pub const CPY: AnnotatedOpcode = (cpy, AddressingModifier::Read);


/* #######################  Increments & Decrements  ####################### */

pub const INC: AnnotatedOpcode = (inc, AddressingModifier::RMW);

pub const INX: AnnotatedOpcode = (inx, AddressingModifier::None);

pub const INY: AnnotatedOpcode = (iny, AddressingModifier::None);

pub const DEC: AnnotatedOpcode = (dec, AddressingModifier::RMW);

pub const DEX: AnnotatedOpcode = (dex, AddressingModifier::None);

pub const DEY: AnnotatedOpcode = (dey, AddressingModifier::None);


/* #######################  Shifts  ####################### */

pub const ASL: AnnotatedOpcode = (asl, AddressingModifier::RMW);

pub const LSR: AnnotatedOpcode = (lsr, AddressingModifier::RMW);

pub const ROL: AnnotatedOpcode = (rol, AddressingModifier::RMW);

pub const ROR: AnnotatedOpcode = (ror, AddressingModifier::RMW);


/* #######################  Jumps & Calls  ####################### */

pub const JMP: AnnotatedOpcode = (jmp, AddressingModifier::None);

pub const JSR: AnnotatedOpcode = (jsr, AddressingModifier::PlaceHolder);

pub const RTS: AnnotatedOpcode = (rts, AddressingModifier::PlaceHolder);


/* #######################  Branches  ####################### */

pub const BCC: AnnotatedOpcode = (bcc, AddressingModifier::None);

pub const BCS: AnnotatedOpcode = (bcs, AddressingModifier::None);

pub const BEQ: AnnotatedOpcode = (beq, AddressingModifier::None);

pub const BMI: AnnotatedOpcode = (bmi, AddressingModifier::None);

pub const BNE: AnnotatedOpcode = (bne, AddressingModifier::None);

pub const BPL: AnnotatedOpcode = (bpl, AddressingModifier::None);

pub const BVC: AnnotatedOpcode = (bvc, AddressingModifier::None);

pub const BVS: AnnotatedOpcode = (bvs, AddressingModifier::None);


/* #######################  Status Flag Changes  ####################### */

pub const CLC: AnnotatedOpcode = (clc, AddressingModifier::None);

pub const CLD: AnnotatedOpcode = (cld, AddressingModifier::None);

pub const CLI: AnnotatedOpcode = (cli, AddressingModifier::None);

pub const CLV: AnnotatedOpcode = (clv, AddressingModifier::None);

pub const SEC: AnnotatedOpcode = (sec, AddressingModifier::None);

pub const SED: AnnotatedOpcode = (sed, AddressingModifier::None);

pub const SEI: AnnotatedOpcode = (sei, AddressingModifier::None);


/* #######################  System Functions  ####################### */

pub const BRK: AnnotatedOpcode = (brk, AddressingModifier::PlaceHolder);

pub const NOP: AnnotatedOpcode = (nop, AddressingModifier::None);

pub const RTI: AnnotatedOpcode = (rti, AddressingModifier::PlaceHolder);
