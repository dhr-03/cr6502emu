use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::ptr;

use super::{Parser, ParseResult, ParsedAddress, ParseError};
use crate::opcodes::{OPCODES_MAP, NONE as OPCODE_NONE};

//TODO: messages

pub type IdentifierMap = HashMap<String, u16>;

#[wasm_bindgen]
pub struct Assembler {
    rom_offset: u16,
    identifiers: IdentifierMap,

    test_tmp: [u8; 30],
    offset: u16,
}

//public api
#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen(constructor)]
    pub fn new(rom_start: u16) -> Assembler {
        Assembler {
            rom_offset: rom_start,
            identifiers: HashMap::new(),

            test_tmp: [0; 30],
            offset: 0,
        }
    }

    pub fn assemble(&mut self, prg: &str) -> *const u8 {
        let mut success = true;
        let mut lines = Parser::clean_input(prg);

        while let (_, Some(line)) = (success, lines.next()) {
            if Parser::is_macro(line) {
                success = self.macro_behaviour(line)
            } else if Parser::is_label(line) {
                success = self.label_behaviour(line)
            } else {
                success = self.instruction_behaviour(line)
            }
        }

        self.identifiers.clear();

        if success {
            &self.test_tmp[0] //get ptr
        } else {
            0 as *const u8
        }
    }
}

impl Assembler {
    #[inline(always)]
    fn macro_behaviour(&mut self, line: &str) -> bool {
        self.parse_macro(line).is_ok()
    }

    #[inline(always)]
    fn label_behaviour(&mut self, line: &str) -> bool {
        let name = &line[..line.len() - 1];

        if !self.identifiers.contains_key(name) {
            self.identifiers.insert(name.into(), self.offset + self.rom_offset);
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn instruction_behaviour(&mut self, line: &str) -> bool {

        match self.parse_instruction(line) {
            Ok((opcode, addr)) => {
                self.write_rom(opcode);

                {
                    use ParsedAddress::*;
                    match addr {
                        Implicit => (),

                        Immediate(v) | ZeroPage(v) | ZeroPageX(v) |
                        ZeroPageY(v) | IndexedIndirect(v) |
                        IndirectIndexed(v) => self.write_rom(v.value),

                        RelativeTarget(v) => self.write_rom_u16(v.value),
                        RelativeOffset(v) => {
                            let target: u16 = v.value as u16 + self.offset - 1;
                            self.write_rom_u16(target);
                        }

                        Absolute(v) | AbsoluteX(v) | AbsoluteY(v) |
                        Indirect(v) => {
                            self.write_rom_u16(v.value);
                        }
                    }
                }

                true
            }

            Err(_) => false
        }
    }
}

//parsers, struct members so they can emit warnings
impl Assembler {
    fn write_rom(&mut self, byte: u8) { //TODO: safe
        self.test_tmp[self.offset as usize] = byte;
        self.offset += 1;
    }

    fn write_rom_u16(&mut self, bytes: u16) {
        self.write_rom(bytes as u8);
        self.write_rom((bytes >> 8) as u8);
    }

    fn parse_instruction(&mut self, line: &str) -> ParseResult<(u8, ParsedAddress)> {
        let space_i = *line.find(' ').get_or_insert(line.len());
        let opcode = &line[..space_i];
        let data = *line.get((space_i + 1)..).get_or_insert("");

        let parsed_addr = self.parse_address(data)?;

        let opcode_val = OPCODES_MAP.get(opcode)
            .ok_or(ParseError::UnknownOpcode)?;

        let index = usize::from(&parsed_addr);

        opcode_val.get(index)
            .ok_or(ParseError::UnknownOpcode)
            .and_then(|v| {
                if *v != OPCODE_NONE {
                    Ok((*v, parsed_addr))
                } else {
                    Err(ParseError::WrongAddressingMode)
                }
            })
    }


    fn parse_macro(&self, _line: &str) -> ParseResult<()> {
        Err(ParseError::UnknownMacro)
    }


    fn parse_address(&self, address: &str) -> ParseResult<ParsedAddress> {
        if address.is_empty() || address == "A" {
            Ok(ParsedAddress::Implicit) //accumulator
        } else {
            Parser::parse_addr_normal(address, &self.identifiers)
                .or_else(|err| {
                    if let ParseError::UnknownAddressingMode = err {
                        Parser::parse_addr_indexed(address, &self.identifiers)
                    } else {
                        Err(err)
                    }
                })
        }
    }
}

