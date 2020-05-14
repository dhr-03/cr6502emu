use crate::assembler::{AssemblerInterface};
use crate::parser::{ParseResult, ParseError};

pub trait CodeItemTrait {
    fn get_size(&self) -> usize;

    fn process(&self, asm: &mut AssemblerInterface) -> bool;
    fn execute(&self, asm: &mut AssemblerInterface) -> ParseResult<()>;
}

pub fn to_boxed<T>(what: T) -> Box<dyn CodeItemTrait> where T: CodeItemTrait + 'static {
    let boxed: Box<dyn CodeItemTrait> = Box::new(what);

    boxed
}

pub fn to_boxed_result<T>(result: ParseResult<T>) -> ParseResult<Box<dyn CodeItemTrait>> where T: CodeItemTrait + 'static {
    result.map(|v| to_boxed(v))
}