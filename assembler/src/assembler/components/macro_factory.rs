use crate::assembler::{AssemblerInterface};
use crate::parser::{ParseResult, ParseError};

use super::common::{CodeItemTrait, to_boxed_result};

pub struct MacroFactory {
    //TODO
}

impl MacroFactory {
    pub fn from_str(_: &str) -> ParseResult<Self> {
        Ok(MacroFactory {})
    }


    pub fn from_str_boxed(line: &str) -> ParseResult<Box<dyn CodeItemTrait>> {
        to_boxed_result(
            Self::from_str(line)
        )
    }
}

impl CodeItemTrait for MacroFactory {
    fn get_size(&self) -> usize {
        0
    }

    fn process(&self, _: &mut AssemblerInterface) -> bool {
        false
    }

    fn execute(&self, _: &mut AssemblerInterface) -> ParseResult<()> {Ok(())}

}
