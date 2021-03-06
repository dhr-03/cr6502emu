use crate::assembler::{AssemblerInterface};
use crate::parser::{Parser, types::*};

use super::common::{CodeItemTrait, to_boxed_result};

use crate::js_logger::Logger;
use crate::lang::LoggerMessage;

pub struct Instruction {
    opcode: u8,
    value: ParsedValue,
}

impl Instruction {
    pub fn new(opcode: u8, value: ParsedValue) -> Self {
        Instruction {
            opcode,
            value,
        }
    }

    pub fn from_str(line: &str) -> ParseResult<Self> {
        let (opcode, value) = Parser::parse_instruction(line)?;

        Ok(Self::new(
            opcode,
            value,
        ))
    }

    pub fn from_str_boxed(line: &str) -> ParseResult<Box<dyn CodeItemTrait>> {
        to_boxed_result(
            Self::from_str(line)
        )
    }

    pub fn get_map_value(&self, asm: &AssemblerInterface) -> ParseResult<ParsedValue> {
        if let AddressingMode::RelativeTarget = self.value.addr_mode() {
            let position = asm.write_ptr() as i32;
            let target = self.value.resolve(asm)
                .ok_or(ParseError::UnknownIdentifier)? as i32;

            let offset = target - (asm.rom_start() as i32 + position + 1);

            if offset > 127 || offset < -128 {
                Logger::err_msg(LoggerMessage::AsmErrTargetTooFar);

                Err(ParseError::ValueSize)
            } else {
                Ok(ParsedValue::new(
                    AddressingMode::RelativeOffset,
                    ValueMode::I8(offset as i8),
                    false,
                ))
            }
        } else {
            Ok(self.value.clone())
        }
    }
}

impl CodeItemTrait for Instruction {
    fn get_size(&self) -> usize {
        let data_size;

        if let AddressingMode::RelativeTarget = self.value.addr_mode() {
            data_size = 1;
        } else {
            data_size = self.value.value().get_size();
        }

        data_size + 1 //opcode
    }

    fn process(&self, _: &mut AssemblerInterface) -> (bool, bool) {
        (true, true)
    }

    fn execute(&self, asm: &mut AssemblerInterface) -> ParseResult<()> {
        asm.write(self.opcode);

        let final_value = self.get_map_value(asm)?;

        let val_size = final_value.value().get_size();
        let value = final_value.resolve(asm);

        match value {
            Some(v) => {
                if val_size > 0 {
                    asm.write(v as u8);
                    if val_size > 1 {
                        asm.write((v >> 8) as u8);
                    }
                }

                Ok(())
            }

            None => {
                Logger::explained_err(
                    LoggerMessage::AsmErrLblNeverDef,
                    self.value.label_name().get_or_insert(""),
                );

                Err(ParseError::UnknownIdentifier)
            }
        }
    }
}
