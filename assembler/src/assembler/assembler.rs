use wasm_bindgen::prelude::wasm_bindgen;

use super::{Parser, ParseResult, ParsedValue, ParseError,
            ValueMode,
            LabelManager};

use crate::opcodes::{OPCODES_MAP, NONE as OPCODE_NONE};
use super::AddressingMode;

use super::js_logger::{Logger,
                       err_code, info_code_i32, warn_code,
                       err_msg, warn_msg};
use crate::lang::assembler as lang;

#[wasm_bindgen]
pub struct Assembler {
    rom_offset: u16,
    identifiers: LabelManager,

    test_tmp: [u8; 30],
    offset: u16,
}

//public api
#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen(constructor)]
    pub fn new(rom_start: u16) -> Assembler {
        Logger::setup(&"div.demo");

        Assembler {
            rom_offset: rom_start,
            identifiers: LabelManager::new(),

            test_tmp: [0; 30],
            offset: 0,
        }
    }

    pub fn assemble(&mut self, prg: &str) -> *const u8 {
        self.offset = 0;

        let mut success = true;

        let mut lines = Parser::clean_input(prg);
        while let (true, Some(line)) = (success, lines.next()) {
            if Parser::is_macro(line) {
                success = self.macro_behaviour(line);

            } else if Parser::is_label(line) {
                success = self.label_behaviour(line);

            } else {
                success = self.instruction_behaviour(line);
            }
        }

        // write unordered labels
        let keys: Vec<String> = self.identifiers.map.keys()
            .map(|k| k.clone())
            .collect();

        let mut keys_iter = keys.iter();

        while let (true, Some(key)) = (success, keys_iter.next()) {
            let label = self.identifiers.map.remove_entry(key).unwrap().1;

            if let Some(value) = label.value {
                let w_val = value as u8;
                for addr in label.usages_lo.iter() {
                    success = self.write_rom_at(w_val as u8, *addr);
                }

                let w_val = (value >> 8) as u8;
                for addr in &label.usages_hi {
                    success = self.write_rom_at(w_val, *addr);
                }
            } else {
                err_code(lang::ERR_LBL_NEVER_DEF_1, key.as_str(), lang::ERR_LBL_NEVER_DEF_2);
                success = false; //undefined label
            }
        }

        self.identifiers.map.clear();

        if success && self.offset < 1 {
            err_msg(lang::ERR_EMPTY_INPUT);

            success = false;
        }

        if success {
            self.clear_unused_rom();

            info_code_i32(lang::INFO_ASM_SUCCESS_1, self.offset as i32, lang::INFO_ASM_SUCCESS_2);

            &self.test_tmp[0] //get ptr
        } else {
            err_msg(lang::ERR_ASM_FAILED);

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

        if let Some(label) = self.identifiers.map.get_mut(name) {
            if let Some(_) = label.value { //if the value is already defined

                err_code(lang::ERR_LBL_RE_DEF_1, name, lang::ERR_LBL_RE_DEF_2);
                false
            } else {
                label.value = Some(self.rom_offset + self.offset);
                true
            }
        } else {
            self.identifiers.insert(name.into(), self.offset + self.rom_offset);
            true
        }
    }

    #[inline(always)]
    fn instruction_behaviour(&mut self, line: &str) -> bool {
        let mut rt: bool;

        match self.parse_instruction(line) {
            Ok((opcode, addr)) => {
                rt = self.write_rom(opcode);

                match addr.value() {
                    ValueMode::None => (),

                    ValueMode::U8(v) => rt = self.write_rom(*v),

                    ValueMode::U16(v) => rt = self.write_rom_u16(*v),

                    ValueMode::I8(offset) => {
                        let write_offset_sg = self.offset as i32;
                        let offset_32 = *offset as i32;

                        let target = write_offset_sg + offset_32;

                        if target < 0 && target < write_offset_sg ||
                            target >= self.test_tmp.len() as i32{
                            warn_msg(lang::WARN_REL_OOB)
                        }

                        rt = self.write_rom(*offset as u8)
                    },

                    ValueMode::Label(name) => {
                        let data = self.identifiers.get_or_sched(name, self.offset);
                        if let Some(bytes) = data {
                            rt = self.write_rom_u16(bytes);
                        } else {
                            self.offset += 2;
                        }
                    }

                    ValueMode::LabelLo(name) => {
                        let data = self.identifiers.get_or_sched_lo(name, self.offset);
                        if let Some(byte) = data {
                            rt = self.write_rom(byte);
                        } else {
                            self.offset += 1;
                        }
                    }

                    ValueMode::LabelHi(name) => {
                        let data = self.identifiers.get_or_sched_hi(name, self.offset);
                        if let Some(byte) = data {
                            rt = self.write_rom(byte);
                        } else {
                            self.offset += 1;
                        }
                    }
                }
                rt
            }

            Err(err) => {
                Logger::begin_err();
                Logger::write_code(err.to_str());
                Logger::end_msg();

                false
            }
        }
    }
}

impl Assembler {
    #[inline(always)]
    fn write_rom_at(&mut self, byte: u8, addr: u16) -> bool {
        if (addr as usize) < self.test_tmp.len() {
            self.test_tmp[addr as usize] = byte;
            true
        } else {
            err_msg(lang::ERR_ROM_TOO_SMALL);

            false
        }
    }

    fn write_rom(&mut self, byte: u8) -> bool {
        let write_ok = self.write_rom_at(byte, self.offset);
        self.offset += 1;

        write_ok
    }

    fn write_rom_u16(&mut self, bytes: u16) -> bool {
        self.write_rom(bytes as u8) &&
            self.write_rom((bytes >> 8) as u8)
    }

    fn clear_unused_rom(&mut self) {
        for i in self.offset..self.test_tmp.len() as u16 {
            self.test_tmp[i as usize] = 0;
        }
    }

    fn parse_instruction(&mut self, line: &str) -> ParseResult<(u8, ParsedValue)> {
        let space_i = *line.find(' ').get_or_insert(line.len());
        let opcode = &line[..space_i];
        let data = *line.get((space_i + 1)..).get_or_insert("");

        let parsed_addr = self.parse_address(data)?;

        let opcode_val = OPCODES_MAP.get(opcode)
            .ok_or_else(|| {
                Logger::begin_err();
                Logger::write_str(lang::ERR_UNKNOWN_OPCODE);
                Logger::write_code(opcode);
                Logger::end_msg();

                ParseError::UnknownOpcode
            })?;

        let index = usize::from(parsed_addr.addr_mode());

        opcode_val.get(index)
            .ok_or(ParseError::UnknownOpcode)
            .and_then(|v| {
                if *v != OPCODE_NONE {
                    Ok((*v, parsed_addr))
                } else {
                    Logger::begin_err();
                    Logger::write_str(lang::ERR_ADDR_MODE_1);
                    Logger::write_code(opcode);
                    Logger::write_str(lang::ERR_ADDR_MODE_2);
                    Logger::write_code(parsed_addr.addr_mode().to_str());
                    Logger::end_msg();

                    Err(ParseError::WrongAddressingMode)
                }
            })
    }


    fn parse_macro(&self, _line: &str) -> ParseResult<()> {
        err_msg(lang::ERR_MACROS_TODO);

        Err(ParseError::UnknownMacro)
    }


    fn parse_address(&self, address: &str) -> ParseResult<ParsedValue> {
        if address.is_empty() || address == "A" { //accumulator
            Ok(
                ParsedValue::new(AddressingMode::Implicit, ValueMode::None, false)
            )
        } else {
            Parser::parse_addr_normal(address)
                .or_else(|err| {
                    if let ParseError::UnknownAddressingMode = err {
                        Parser::parse_addr_indexed(address)
                    } else {
                        Err(err)
                    }
                })
        }
    }
}

