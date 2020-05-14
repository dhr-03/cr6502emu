mod parser;
mod parser_asm;
mod parser_addr;

mod types_result;
mod types_value;

pub use parser::Parser;
pub mod types {
    pub use super::types_result::*;
    pub use super::types_value::*;
}