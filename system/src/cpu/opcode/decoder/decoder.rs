use super::{
    DECODE_TABLE,
    super::DecodedInstruction,
};

pub struct Decoder {}

impl Decoder {
    #[allow(unused_parens)]
    pub fn decode(op: u8) -> DecodedInstruction {
        DECODE_TABLE[op as usize]
    }
}