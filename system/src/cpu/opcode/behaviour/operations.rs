use super::super::{AnnotatedOpcode, AddressingModifier};
use super::operations_internal::*;

/* #######################  Load/Store Operations  ####################### */
pub const LDA: AnnotatedOpcode = (lda, AddressingModifier::PlaceHolder);

pub const LDX: AnnotatedOpcode = (ldx, AddressingModifier::PlaceHolder);

pub const LDY: AnnotatedOpcode = (ldy, AddressingModifier::PlaceHolder);

pub const STA: AnnotatedOpcode = (sta, AddressingModifier::PlaceHolder);

pub const STY: AnnotatedOpcode = (sty, AddressingModifier::PlaceHolder);

pub const STX: AnnotatedOpcode = (stx, AddressingModifier::PlaceHolder);


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

pub const AND: AnnotatedOpcode = (and, AddressingModifier::PlaceHolder);

pub const EOR: AnnotatedOpcode = (eor, AddressingModifier::PlaceHolder);

pub const ORA: AnnotatedOpcode = (ora, AddressingModifier::PlaceHolder);

pub const BIT: AnnotatedOpcode = (bit, AddressingModifier::PlaceHolder);


/* #######################  Arithmetic  ####################### */

pub const ADC: AnnotatedOpcode = (adc, AddressingModifier::PlaceHolder);

pub const SBC: AnnotatedOpcode = (sbc, AddressingModifier::PlaceHolder);

pub const CMP: AnnotatedOpcode = (cmp, AddressingModifier::PlaceHolder);

pub const CPX: AnnotatedOpcode = (cpx, AddressingModifier::PlaceHolder);

pub const CPY: AnnotatedOpcode = (cpy, AddressingModifier::PlaceHolder);


/* #######################  Increments & Decrements  ####################### */

pub const INC: AnnotatedOpcode = (inc, AddressingModifier::PlaceHolder);

pub const INX: AnnotatedOpcode = (inx, AddressingModifier::None);

pub const INY: AnnotatedOpcode = (iny, AddressingModifier::None);

pub const DEC: AnnotatedOpcode = (dec, AddressingModifier::PlaceHolder);

pub const DEX: AnnotatedOpcode = (dex, AddressingModifier::None);

pub const DEY: AnnotatedOpcode = (dey, AddressingModifier::None);


/* #######################  Shifts  ####################### */

pub const ASL: AnnotatedOpcode = (asl, AddressingModifier::PlaceHolder);

pub const LSR: AnnotatedOpcode = (lsr, AddressingModifier::PlaceHolder);

pub const ROL: AnnotatedOpcode = (rol, AddressingModifier::PlaceHolder);

pub const ROR: AnnotatedOpcode = (ror, AddressingModifier::PlaceHolder);


/* #######################  Jumps & Calls  ####################### */

pub const JMP: AnnotatedOpcode = (jmp, AddressingModifier::PlaceHolder);

pub const JSR: AnnotatedOpcode = (jsr, AddressingModifier::PlaceHolder);

pub const RTS: AnnotatedOpcode = (rts, AddressingModifier::PlaceHolder);


/* #######################  Branches  ####################### */

pub const BCC: AnnotatedOpcode = (bcc, AddressingModifier::PlaceHolder);

pub const BCS: AnnotatedOpcode = (bcs, AddressingModifier::PlaceHolder);

pub const BEQ: AnnotatedOpcode = (beq, AddressingModifier::PlaceHolder);

pub const BMI: AnnotatedOpcode = (bmi, AddressingModifier::PlaceHolder);

pub const BNE: AnnotatedOpcode = (bne, AddressingModifier::PlaceHolder);

pub const BPL: AnnotatedOpcode = (bpl, AddressingModifier::PlaceHolder);

pub const BVC: AnnotatedOpcode = (bvc, AddressingModifier::PlaceHolder);

pub const BVS: AnnotatedOpcode = (bvs, AddressingModifier::PlaceHolder);


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
