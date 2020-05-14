use crate::assembler::{AssemblerInterface};
use crate::parser::{Parser, types::*};

use super::common::{CodeItemTrait, to_boxed_result};

pub struct Instruction {
    opcode: u8,
    value: ParsedValue,
}

impl Instruction {
    pub fn from_str(line: &str) -> ParseResult<Self> {
        let (opcode, value) = Parser::parse_instruction(line)?;

        Ok(Instruction {
            opcode,
            value,
        })
    }

    pub fn from_str_boxed(line: &str) -> ParseResult<Box<dyn CodeItemTrait>> {
        to_boxed_result(
            Self::from_str(line)
        )
    }
}

impl CodeItemTrait for Instruction {
    fn get_size(&self) -> usize {
        self.value.value().get_size() + 1 //opcode
    }

    fn process(&self, _: &mut AssemblerInterface) -> (bool, bool) {
        (true, true)
    }

    fn execute(&self, asm: &mut AssemblerInterface) -> ParseResult<()> {
        asm.write(self.opcode);

        let val_size = self.value.value().get_size();
        let value = self.value.resolve(asm);

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

            None => Err(ParseError::UnknownIdentifier)
        }
    }
}