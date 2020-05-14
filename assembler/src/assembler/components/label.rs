use crate::assembler::{AssemblerInterface};
use crate::parser::types::{ParseResult, ParseError};

use super::common::{CodeItemTrait, to_boxed_result};

use crate::js_logger::{err_code};
use crate::lang::assembler as lang;

pub struct Label {
    name: String
}

impl Label {
    pub fn from_str(line: &str) -> ParseResult<Self> {
        let name = &line[0..line.len() - 1];

        if name.len() > 15 {
            err_code(lang::ERR_LBL_LONG_1, name, lang::ERR_LBL_LONG_2);

            Err(ParseError::ValueSize)
        } else if name.len() < 3 {
            err_code(lang::ERR_LBL_SHORT_1, name, lang::ERR_LBL_SHORT_2);

            Err(ParseError::ValueSize)
        } else{
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
            err_code(lang::ERR_LBL_RE_DEF_1, self.name.as_str(), lang::ERR_LBL_RE_DEF_2);

            ok = false;
        } else {
            asm.insert_label(self.name.as_str(), asm.offset());

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
