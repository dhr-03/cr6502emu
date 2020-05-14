use crate::parser::types::{ParseResult, ParseError};

use super::common::{CodeItemTrait, to_boxed_result};

use crate::js_logger::{err_msg, err_code};
use crate::lang::macros as lang;

use super::MacroWrite;

pub struct MacroFactory {}

impl MacroFactory {
    pub fn from_str_boxed(line: &str) -> ParseResult<Box<dyn CodeItemTrait>> {
        to_boxed_result(
            if line.starts_with("store") {
                Self::macro_write(line)
            } else {
                Err(ParseError::UnknownMacro)
            }
        )
    }

    fn macro_write(line: &str) -> ParseResult<MacroWrite> {
        let args: Vec<&str> = line.splitn(3, " ").collect();

        if args.len() == 3 {
            match args[1] {
                "asciiz" => {
                    if args[2].is_ascii() {
                        Ok(MacroWrite::new_str(args[2]))
                    } else {
                        err_msg(lang::ERR_WRT_NON_ASCII);

                        Err(ParseError::InvalidValue)
                    }
                }

                "u8" => {
                    let val: u8 = parse_or_log(args[2])?;

                    Ok(MacroWrite::new_zp(val))
                }

                "u16" => {
                    let val: u16 = parse_or_log(args[2])?;

                    Ok(MacroWrite::new_abs(val))
                }

                _ => Err(ParseError::SyntaxError)
            }
        } else {
            Err(ParseError::SyntaxError)
        }
    }
}

fn parse_or_log<T: std::str::FromStr>(txt: &str) -> ParseResult<T> {
    T::from_str(txt)
        .map_err(|_| {
            err_code(lang::ERR_WRT_NUM_PARSE_1, txt, lang::ERR_WRT_NUM_PARSE_2);

            ParseError::InvalidValue
        })
}

