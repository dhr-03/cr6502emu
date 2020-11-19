use crate::parser::types::{ParseResult, ParseError};

use super::common::{CodeItemTrait, to_boxed_result};

use crate::js_logger::Logger;
use crate::lang::LoggerMessage;

use super::{MacroWrite, MacroDefine};

pub struct MacroFactory {}

impl MacroFactory {
    pub fn from_str_boxed(line: &str) -> ParseResult<Box<dyn CodeItemTrait>> {
        if line.starts_with("store") {
            to_boxed_result(Self::macro_write(line))
        } else if line.starts_with("define") {
            to_boxed_result(MacroDefine::from_str(line))
        } else {
            Err(ParseError::UnknownMacro)
        }
    }

    //todo: this shouldn't be here
    fn macro_write(line: &str) -> ParseResult<MacroWrite> {
        let args: Vec<&str> = line.splitn(3, " ").collect();

        if args.len() == 3 {
            match args[1] {
                "asciiz" => {
                    if args[2].is_ascii() {
                        Ok(MacroWrite::new_str(args[2]))
                    } else {
                        Logger::err_msg(LoggerMessage::McrErrNonAscii);

                        Err(ParseError::InvalidValue)
                    }
                }

                "byte" => {
                    let val: u8 = parse_or_log(args[2])?;

                    Ok(MacroWrite::new_zp(val))
                }

                "bytex2" => {
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
            Logger::explained_err(LoggerMessage::McrErrNumParse, txt);

            ParseError::InvalidValue
        })
}

