use super::Decoder;

use super::super::{InstructionActions, DecodedInstruction, AddressingActions};
use super::super::behaviour::{operations::*, addressing as addr};

const OPCODES_IMP: [&InstructionActions; 30] = [
    &NOP, &NOP, &RTI, &RTS, &TXS, &TSX,
    &PHP, &PLP, &PHA, &PLA, &DEY, &TAY, &INY, &INX,
    &ASL, &ROL, &LSR, &ROR, &TXA, &TAX, &DEX, &NOP,
    &CLC, &SEC, &CLI, &SEI, &TYA, &CLV, &CLD, &SED,
];

//XXX: this is messy
impl Decoder {
    pub fn decode_imp(a: u8, b: u8, c: u8) -> DecodedInstruction {
        let offset: isize;

        if b == 2 {
            offset = 6; //second row
        } else if b == 6 {
            if c == 0 {
                offset = 22; //third row
            } else {
                offset = -8; //c=2 to c=0, ignore b. Map b=6,c=2 to b=0,c=0 keeping a (TXS, TSX)
            }
        } else {
            offset = 0;
        }

        let index = offset + (a + c * 4) as isize;

        let opcode = match OPCODES_IMP.get(index as usize) {
            Some(v) => *v,
            None => &NOP
        };

        (
            &addr::IMP as &AddressingActions,
            opcode

        )
    }

    pub fn is_imp(a: u8, b: u8, c: u8) -> bool {
        c != 3 && (
            (b == 0 && c == 0 && a < 4) || //OPCODES_IMP row 0
                (b == 2 && c != 1) || //OPCODES_IMP row 2 and 3
                (b == 6 && (c == 0 || (c == 2 && (a == 4 || a == 5)))) //TXS TSX
        )
    }
}