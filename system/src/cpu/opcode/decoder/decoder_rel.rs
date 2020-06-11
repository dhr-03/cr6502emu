use super::Decoder;

use super::super::{InstructionActions, DecodedInstruction};
use super::super::behaviour::{operations::*, addressing};

const OPCODES_REL_A: [&InstructionActions; 8] = [
    &BPL, &BMI, &BVC, &BVS, &BCC, &BCS, &BNE, &BEQ
];

impl Decoder {
    pub fn decode_rel(a: u8) -> DecodedInstruction {
        (
            &addressing::REL,
            OPCODES_REL_A[a as usize] //a == 0bXXX
        )
    }

    pub fn is_rel(_: u8, b: u8, c: u8) -> bool {
        b == 4 && c == 0
    }
}