use wasm_bindgen::prelude::*;

use super::{Parser, ParseResult, ParsedAddress, ParseError,
            ParsedU8, ParsedI8, ParsedU16};
use crate::opcodes::{OPCODES_MAP, NONE as OPCODE_NONE};

use crate::alert;

#[wasm_bindgen]
pub struct Assembler {
    test_tmp: [u8; 30],
    offset: usize
}

//public api
#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Assembler {
        Assembler {
            test_tmp: [0; 30],
            offset: 0
        }
    }

    pub fn assemble(&mut self, prg: &str) -> *const u8 { //TODO: result
        for line in Parser::clean_input(prg) {
        //TODO: check max length
        //      check type: instruction, label, macro

            if let Err(_) = self.parse_instruction(line) {
                alert("err");
                break
            }
        }

        &self.test_tmp[0]
    }
}

//parsers, struct members so they can emit warnings
impl Assembler {
    fn write_rom(&mut self, byte: u8) { //TODO: safe
        self.test_tmp[self.offset] = byte;
        self.offset += 1;
    }

    fn parse_instruction(&mut self, line: &str,) -> ParseResult<()> {
        let space_i = *line.find(' ').get_or_insert(line.len());
        let opcode = &line[..space_i];
        let data = *line.get((space_i + 1)..).get_or_insert("");

        let parsed_addr = self.parse_address(data)?;

        let opcode_val = OPCODES_MAP.get(opcode)
            .ok_or(ParseError::UnknownOpcode)?;

        let opcode_val = opcode_val.get(usize::from(&parsed_addr))
            .ok_or(ParseError::UnknownOpcode)
            .and_then(|v| {
                if *v != OPCODE_NONE {
                    Ok(v)
                } else {
                    Err(ParseError::WrongAddressMode)
                }
            })?;

        self.write_rom(*opcode_val);

        {
            use ParsedAddress::*;
            match parsed_addr {
                Implicit => (),

                Immediate(v) | ZeroPage(v) | ZeroPageX(v) |
                ZeroPageY(v) | IndexedIndirect(v) |
                IndirectIndexed(v) => self.write_rom(v.value),

                RelativeTarget(v) => {}
                RelativeOffset(v) => {}

                Absolute(v) | AbsoluteX(v) | AbsoluteY(v) |
                Indirect(v) => {
                    self.write_rom(v.value as u8);
                    self.write_rom((v.value >> 8) as u8);
                }
            }
        }

        Ok(())
    }


    fn parse_macro(&self, line: &str) {
        unimplemented!()
    }


    fn parse_address(&self, address: &str) -> ParseResult<ParsedAddress> {
        if address.is_empty() || address == "A" {
            Ok(ParsedAddress::Implicit) //accumulator
        } else {
            Parser::parse_addr_normal(address)
                .or_else(|err| {
                    if let ParseError::UnknownAddressMode = err {
                        Parser::parse_addr_indexed(address)
                    } else {
                        Err(err)
                    }
                })
        }
    }
}

