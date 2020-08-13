use crate::assembler::{AssemblerInterface};
use crate::parser::types::{ParseResult, ParseError, ValueMode};

use super::common::CodeItemTrait;

use crate::js_logger::{err_code};
use crate::lang::assembler as lang;
use crate::parser::Parser;

pub struct MacroDefine {
    name: String,
    value: u16,
}

impl MacroDefine {
    pub fn from_str(line: &str) -> ParseResult<Self> {
        let args: Vec<&str> = line.splitn(3, " ").collect();

        let name = *args.get(1).ok_or(ParseError::SyntaxError)?;
        let value_str = *args.get(2).ok_or(ParseError::SyntaxError)?;

        //todo: duplicated fragments: this, self.process (Label::from_str)
        if name.len() > 15 {
            err_code(lang::ERR_LBL_LONG_1, name, lang::ERR_LBL_LONG_2);

            Err(ParseError::ValueSize)
        } else if name.len() < 3 {
            err_code(lang::ERR_LBL_SHORT_1, name, lang::ERR_LBL_SHORT_2);

            Err(ParseError::ValueSize)
        } else{

            let value = Parser::parse_addr_value(value_str, 0, false)?;

            match value {
                ValueMode::U8(n) => {
                    let number = n as u16;

                    Ok(MacroDefine{
                        name: String::from(name),
                        value: number,
                    })
                }

                ValueMode::U16(number) => {
                    Ok(MacroDefine{
                        name: String::from(name),
                        value: number,
                    })
                }

                _ => Err(ParseError::SyntaxError)
            }
        }
    }
}

impl CodeItemTrait for MacroDefine {
    fn get_size(&self) -> usize {
        0
    }

    fn process(&self, asm: &mut AssemblerInterface) -> (bool, bool) {
        let ok;

        if let Some(_) = asm.get_label_value(self.name.as_str()) {
            err_code(lang::ERR_LBL_RE_DEF_1, self.name.as_str(), lang::ERR_LBL_RE_DEF_2);

            ok = false;
        } else {
            asm.insert_label(&self.name, self.value);

            ok = true;
        }

        (ok, false)
    }

    fn execute(&self, _: &mut AssemblerInterface) -> ParseResult<()> {
        #[cfg(debug_assertions)]
        panic!("define.execute");

        #[allow(unreachable_code)]
        Ok(())
    }
}
