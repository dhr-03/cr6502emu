use super::super::DecodedInstruction;

use super::decoder_non_pattern;

pub struct Decoder {}

impl Decoder {
    #[allow(unused_parens)]
    pub fn decode(op: u8) -> DecodedInstruction {
        /*
        The 6502 instruction table is laid out according to a pattern a-b-c, where
        a and b are an octal number each, followed by a group of two binary digits c,
        as in the bit-vector "aaabbbcc".

        ...

        Text copyright: https://www.masswerk.at
        Source:         https://www.masswerk.at/6502/6502_instruction_set.html
        */

        let a = (op >> 5);
        let b = (op >> 2) & 0b111;
        let c = (op >> 0) & 0b11;


        if Self::is_rel(a, b, c) {
            Self::decode_rel(a)
        } else if Self::is_non_pattern(op) {
            Self::decode_non_pattern(op)
        } else if Self::is_imp(a, b, c) {
            Self::decode_imp(a, b, c)
        } else {
            Self::decode_c(a, b, c)
        }
    }
}