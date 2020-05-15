use phf::{Map, phf_map};

use crate::parser::types::{ParseError};

pub struct CustomOpcodeParser {}

impl CustomOpcodeParser {
    pub fn from_str(line: &str) {
        let opcode_end = line.find(" ")
            .get_or_insert(line.len());

        let opcode = &line[0..opcode_end];
        let mut arg = line[opcode_end..line.len()];

        let arg_type: usize; //IMP, IMM, ABS

        if arg.starts_with("(") && arg.ends_with(")") {
            arg_type = 3;
            arg = &arg[1..arg.len() - 1];
        } else if arg.is {
            arg_type = 2;
            arg_is_imm = true;
        }

        let opcode_values = CUSTOM_OPCODES.get(opcode)
            .ok_or(ParseError::UnknownOpcode)?;

        let opcode_val = opcode_values.get()

        Ok()
    }
}



const OPCODE_NONE: u8 = 0xFF;

static CUSTOM_OPCODES: Map<&'static str, [u8; 3]> = phf_map! {
               //  IMP   IMM   ABS
        "CAR" => [NONE, NONE, NONE]
        "ALM" => [NONE, NONE, NONE],
        "SUM" => [NONE, NONE, NONE],
        "RES" => [NONE, NONE, NONE],
        "MUL" => [NONE, NONE, NONE],
        "DIV" => [NONE, NONE, NONE],
        "LEE" => [NONE, NONE, NONE],
        "IMP" => [NONE, NONE, NONE],
        "FIN" => [NONE, NONE, NONE],

        "IR" => [NONE, NONE, NONE],
        "SI0" => [NONE, NONE, NONE],
        "SIME" => [NONE, NONE, NONE],
        "SUB" => [NONE, NONE, NONE],
        "RET" => [NONE, NONE, NONE],
};

