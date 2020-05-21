use crate::parser::Parser;

use crate::parser::types::*;

use crate::opcodes::{OPCODES_MAP, OPCODE_NONE};

use crate::js_logger::{Logger, err_code};
use crate::lang::assembler as lang;

impl Parser {
    pub fn parse_instruction(line: &str) -> ParseResult<(u8, ParsedValue)> {
        let space_i = *line.find(' ').get_or_insert(line.len());
        let opcode = &line[..space_i];
        let data = *line.get((space_i + 1)..).get_or_insert("");

        let parsed_addr = Self::parse_address(data)?;

        let opcode_val = OPCODES_MAP.get(opcode)
            .ok_or_else(|| {
                err_code(lang::ERR_UNKNOWN_OPCODE, opcode, "");

                ParseError::UnknownOpcode
            })?;

        let index = parsed_addr.addr_mode().to_table_index();

        opcode_val.get(index)
            .ok_or(ParseError::UnknownOpcode)
            .and_then(|v| {
                if *v != OPCODE_NONE {
                    Ok((*v, parsed_addr))
                } else {
                    Logger::begin_err();
                    Logger::write_str(lang::ERR_ADDR_MODE_1);
                    Logger::write_code(opcode);
                    Logger::write_str(lang::ERR_ADDR_MODE_2);
                    Logger::write_code(parsed_addr.addr_mode().to_str());
                    Logger::end_msg();

                    Err(ParseError::WrongAddressingMode)
                }
            })
    }


    fn parse_address(address: &str) -> ParseResult<ParsedValue> {
        if address.is_empty() || address == "A" { //accumulator
            Ok(
                ParsedValue::new(AddressingMode::Implicit, ValueMode::None, false)
            )
        } else {
            Parser::parse_addr_normal(address)
                .or_else(|err| {
                    if let ParseError::UnknownAddressingMode = err {
                        Parser::parse_addr_indexed(address)
                    } else {
                        Err(err)
                    }
                })
        }
    }
}

impl Parser {
    pub fn sanitize_line(line: &str) -> &str {
        match line.find(';') {
            Some(i) => &line[..i], //if found, remove comment
            None => line
        }.trim() //remove blank chars, such as [space] \t \n \r ...
    }

    #[inline(always)]
    pub fn clean_input(input: &str) -> impl Iterator<Item=(usize, &str)> {
        input.lines()
            .enumerate()
            .map(|(n, l)| (n, Parser::sanitize_line(l)))
            .filter(|(_, l)| !l.is_empty())
    }

    #[inline(always)]
    pub fn is_macro(line: &str) -> bool {
        line.starts_with("#")
    }

    #[inline(always)]
    pub fn is_label(line: &str) -> bool {
        line.ends_with(":") &&
            (&line[..line.len() - 1])
                .chars()
                .all(|c| char::is_alphanumeric(c) || c == '_')
    }

    pub fn is_instruction(line: &str) -> bool {
        //opcode must be 3 chars long, and alphabetic
        line.get(0..3).map_or(false, |op| {
            op.chars().all(char::is_alphabetic) &&
                //fourth char must be [None] or a space
                line.get(3..4).map_or(true, |c| c == " ")
        })
    }
}