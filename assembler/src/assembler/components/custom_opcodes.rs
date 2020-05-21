use phf::{Map, phf_map};

use crate::parser::Parser;
use crate::parser::types::{ParseError, ParseResult,
                           AddressingMode, ParsedValue, ValueMode};
use super::{CodeItemTrait, Instruction, common::to_boxed};

use crate::js_logger::{Logger, warn_msg, err_msg};
use crate::lang;

pub struct CustomOpcodeParser {}

impl CustomOpcodeParser {
    pub fn from_str(line: &str) -> ParseResult<Box<dyn CodeItemTrait>> {

        //XXX: needs refactoring

        //TODO: do and remove
        warn_msg("Custom opcodes are partially unimplemented");

        let opcode_end = *line.find(" ")
            .get_or_insert(line.len());

        let opcode = &line[0..opcode_end];
        let arg = *line.get((opcode_end + 1)..line.len())
            .get_or_insert("");

        let addr_mode: AddressingMode;
        let addr_value: ParseResult<ValueMode>;

        use AddressingMode::{Absolute, Immediate, Implicit};

        if arg.is_empty() {
            addr_mode = Implicit;
            addr_value = Ok(ValueMode::None)
        } else if arg.starts_with("(") && arg.ends_with(")") {
            addr_mode = Absolute;

            addr_value = Parser::parse_addr_value(
                &arg[1..arg.len() - 1], 0, false,
            ).and_then(|v| v.into_abs().map_err(|_| ParseError::InvalidValue));

        } else {
            addr_mode = Immediate;

            addr_value = Parser::parse_addr_value(
                arg, 0, false,
            )
        }

        let addr_value = addr_value?;
        match addr_mode {
            Immediate => {
                if !addr_value.is_zp() {
                    err_msg(lang::parser::ERR_EXPECTED_ZP);

                    return Err(ParseError::ValueSize);
                }
            }

            _ => ()
        }

        let opcode_modes = CUSTOM_OPCODES.get(opcode)
            .ok_or(ParseError::UnknownOpcode)?;

        let opcode_val = opcode_modes.get(
            Self::addr_mode_cust_index(&addr_mode)
        ).ok_or(ParseError::UnknownOpcode)?;

        // ############## _TMP ##################
        //TODO: do and remove
        if *opcode_val == _TMP {
            warn_msg("Using placeholder")
        }

        // ############## /_TMP ##################

        if *opcode_val != OPCODE_NONE {
            let is_addr = match addr_mode {
                Implicit => false,
                Immediate => false,
                _ => true
            };

            let addr = ParsedValue::new(addr_mode, addr_value, is_addr);

            let boxed = to_boxed(Instruction::new(
                *opcode_val,
                addr,
            ));

            Ok(boxed)
        } else {
            Logger::begin_err();
            Logger::write_str(lang::assembler::ERR_ADDR_MODE_1);
            Logger::write_code(opcode);
            Logger::write_str(lang::assembler::ERR_ADDR_MODE_2);
            Logger::write_code(addr_mode.to_str());
            Logger::end_msg();

            Err(ParseError::WrongAddressingMode)
        }
    }

    fn addr_mode_cust_index(adr_mode: &AddressingMode) -> usize {
        use AddressingMode::*;
        use std::usize;

        match adr_mode {
            Implicit => 0,
            Immediate => 1,
            Absolute => 2,

            _ => usize::MAX
        }
    }

    pub fn is_cust_instruction(line: &str) -> bool {
        let opcode_end = *line.find(" ")
            .get_or_insert(line.len());

        (&line[0..opcode_end]).chars().all(char::is_alphanumeric) &&
            line.get(opcode_end..(opcode_end + 1))
                // is space or EOL
                .map_or(true, |c| c == " ")
    }
}


use crate::opcodes::OPCODE_NONE;
use OPCODE_NONE as NONE;

const _TMP: u8 =  0xFE; //placehodler
//TODO: add actual opcodes
static CUSTOM_OPCODES: Map<&'static str, [u8; 3]> = phf_map! {
        //         IMP   IMM   ABS
        "CAR" =>  [NONE, 0xA9, 0xAD],
        "ALM" =>  [NONE, NONE, 0x8D],
        "SUM" =>  [NONE, 0x69, 0x6D],
        "RES" =>  [NONE, 0xE9, 0xED],
        "MUL" =>  [NONE, _TMP, _TMP],
        "DIV" =>  [NONE, _TMP, _TMP],
        "LEE" =>  [NONE, NONE, _TMP],
        "IMP" =>  [NONE, NONE, _TMP],
        "FIN" =>  [_TMP, NONE, NONE],

        "IR" =>   [NONE, NONE, 0x4C],
        "SI0" =>  [NONE, NONE, _TMP],
        "SIME" => [NONE, NONE, _TMP],
        "SUB" =>  [NONE, NONE, 0x20],
        "RET" =>  [0x60, NONE, NONE],
        };

