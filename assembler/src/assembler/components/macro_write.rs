use super::common::{CodeItemTrait};
use crate::parser::types::ParseResult;
use crate::assembler::AssemblerInterface;

enum DataContainer {
    Str(String),
    ABS(u16),
    ZP(u8),
}

pub struct MacroWrite {
    data: DataContainer
}

impl MacroWrite {
    pub fn new_str(str: &str) -> Self {
        MacroWrite {
            data: DataContainer::Str(String::from(str))
        }
    }

    pub fn new_abs(num: u16) -> Self {
        MacroWrite {
            data: DataContainer::ABS(num)
        }
    }

    pub fn new_zp(num: u8) -> Self {
        MacroWrite {
            data: DataContainer::ZP(num)
        }
    }
}

impl CodeItemTrait for MacroWrite {
    fn get_size(&self) -> usize {
        use DataContainer::*;
        match &self.data {
            Str(s) => s.len() + 1,
            ABS(_) => 2,
            ZP(_) => 1
        }
    }

    fn process(&self, _asm: &mut AssemblerInterface) -> (bool, bool) {
        (true, true)
    }

    fn execute(&self, asm: &mut AssemblerInterface) -> ParseResult<()> {
        use DataContainer::*;
        match &self.data {
            Str(s) => {
                for char in s.as_bytes() {
                    asm.write(*char);
                }

                asm.write(0);
            },
            ABS(v) => {
                asm.write(*v as u8);
                asm.write((*v >> 8) as u8);
            },
            ZP(v) => {
                asm.write(*v)
            }
        }

        Ok(())
    }
}