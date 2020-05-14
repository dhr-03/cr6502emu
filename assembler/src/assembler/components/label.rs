use crate::assembler::{AssemblerInterface};
use crate::parser::{ParseResult, ParseError};

use super::common::{CodeItemTrait, to_boxed_result};

pub struct Label {
    name: String
}

impl Label {
    pub fn from_str(line: &str) -> ParseResult<Self> {
        let name = &line[0..line.len() - 1];

        //TODO pick size, min size: const
        if name.len() <= 20 {
            Ok(Label {
                name: name.to_string()
            })
        } else {
            Err(ParseError::ValueTooBig)
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

    fn process(&self, asm: &mut AssemblerInterface) -> bool {
        if let Some(_) = asm.get_label_value(self.name.as_str()) {
            panic!("label redef");
        } else {
            asm.insert_label(self.name.as_str(), asm.offset)
        }


        false
    }

    fn execute(&self, _: &mut AssemblerInterface) -> ParseResult<()> {
        #[cfg(debug_assertions)]
        panic!("label.execute");

        Ok(())
    }
}
