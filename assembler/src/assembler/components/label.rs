use crate::assembler::{AssemblerInterface};
use crate::parser::types::{ParseResult, ParseError};

use super::common::{CodeItemTrait, to_boxed_result};

use crate::js_logger::Logger;
use crate::lang::LoggerMessage;

pub struct Label {
    name: String
}

impl Label {
    pub fn from_str(line: &str) -> ParseResult<Self> {
        let name = &line[0..line.len() - 1];

        if name.len() > 15 {
            Logger::explained_err(LoggerMessage::AsmErrLblLong, name);

            Err(ParseError::ValueSize)
        } else if name.len() < 3 {
            Logger::explained_err(LoggerMessage::AsmErrLblShort, name);

            Err(ParseError::ValueSize)
        } else {
            Ok(Label {
                name: name.to_string()
            })
        }
    }

    pub fn from_str_boxed(line: &str) -> ParseResult<Box<dyn CodeItemTrait>> {
        to_boxed_result(
            Self::from_str(line)
        )
    }
}

impl CodeItemTrait for Label {
    fn get_size(&self) -> usize {
        0
    }

    fn process(&self, asm: &mut AssemblerInterface) -> (bool, bool) {
        let ok;

        if let Some(_) = asm.get_label_value(self.name.as_str()) {
            Logger::explained_err(LoggerMessage::AsmErrLblReDef, self.name.as_str());

            ok = false;
        } else {
            asm.insert_label(
                self.name.as_str(),
                asm.rom_start() + asm.write_ptr(),
            );

            ok = true;
        }


        (ok, false)
    }

    fn execute(&self, _: &mut AssemblerInterface) -> ParseResult<()> {
        #[cfg(debug_assertions)]
        panic!("label.execute");

        #[allow(unreachable_code)]
            Ok(())
    }
}
