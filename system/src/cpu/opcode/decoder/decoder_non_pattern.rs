use super::Decoder;

use super::super::{DecodedInstruction};
use super::super::behaviour::{operations as o, addressing as a};


impl Decoder {
    pub fn decode_non_pattern(op: u8) -> DecodedInstruction {
        match op {
            0x20 => (
                &a::ASB,
                &o::JSR,
            ),

            0x6D => (
                &a::IND,
                &o::JSR
            ),

            0x00 => (
                &a::SBK,
                &o::BRK
            ),

            //0x92 | 0xB2 |
            _ => (
                &a::IMP,
                &o::NOP
            )
        }
    }

    pub fn is_non_pattern(op: u8) -> bool {
        match op {
            0x20 => true, //JSR
            0x6D => true, //JMP ()

            0x92 | 0xB2 => true, //NOP

            0x00 => true, //BRK

            _ => false
        }
    }
}