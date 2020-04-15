use super::{ParseResult, ParsedAddress, NumberBase,
            ParsedU8, ParsedU16, ParsedI8};
use super::re_patterns::{RE_NORMAL_ADDRESSING, RE_INDEXED_ADDRESSING};
use crate::assembler::ParseError;

pub struct Parser {}

impl Parser {
    //TODO: I8
    // for simplicity, lets just return a u16, but inform if the value is actually a [u/i]8
    fn parse_re_addr_common(re_result: &[&str], offset: usize) -> ParseResult<(ParsedU16, bool)> {
        let base: NumberBase;
        let is_zp: bool; //zero-page OR only 1 byte of data (instead of 2)

        let str_value = re_result[offset + 1];

        match re_result[offset + 0] {
            "" => {
                //TODO: check based on the actual value
                base = NumberBase::DEC;
                is_zp = str_value.len() <= 3;
            }

            "$" => {
                base = NumberBase::HEX;
                is_zp = str_value.len() <= 2;
            }

            "b" => {
                base = NumberBase::BIN;
                is_zp = str_value.len() <= 4;
            }

            //"%" | "lo " | "hi " => todo!(), // TODO: labels

            _ => unreachable!() //panic!("regex returned an invalid num type")
        }


        Ok(
            (ParsedU16 {
                is_address: true,
                value: u16::from_str_radix(str_value, base as u32).unwrap(),
                base,
            },
             is_zp
            )
        )
    }

    pub fn parse_addr_normal(address: &str) -> ParseResult<ParsedAddress> {
        let re_nrm = Parser::regex_normal_addressing(address)?;

        let is_addr = re_nrm[0] != "#";

        let (parsed_number, is_zp) =
            Parser::parse_re_addr_common(&re_nrm, 1)?;


        match [is_addr, is_zp] {
            [true, true] => Ok(ParsedAddress::ZeroPage(ParsedU8 {
                is_address: true,
                value: parsed_number.value as u8,
                base: parsed_number.base,
            })),

            [true, false] => Ok(ParsedAddress::Absolute(ParsedU16 {
                is_address: true,
                value: parsed_number.value,
                base: parsed_number.base,
            })),

            [false, true] => Ok(ParsedAddress::Immediate(ParsedU8 {
                is_address: false,
                value: parsed_number.value as u8,
                base: parsed_number.base,
            })),

            [false, false] => Err(ParseError::ValueTooBig)
        }
    }


    pub fn parse_addr_indexed(address: &str) -> ParseResult<ParsedAddress> {
        let re_inx = Parser::regex_indexed_addressing(address)?;

        let (parsed_number, is_zp) =
            Parser::parse_re_addr_common(&re_inx, 1)?;


        let zp_or_err = |val: ParsedAddress| {
            if is_zp {
                Ok(val)
            } else {
                Err(ParseError::ValueTooBig)
            }
        };

        //some values have already been validated by parse_re_addr_common and
        //the regex itself, we can ignore those or assume they are valid
        match re_inx {
            ["", _, _, "", "X", ""] => match is_zp {
                true => Ok(ParsedAddress::ZeroPageX(ParsedU8 {
                    is_address: true,
                    value: parsed_number.value as u8,
                    base: parsed_number.base,
                })),
                false => Ok(ParsedAddress::AbsoluteX(ParsedU16 {
                    is_address: true,
                    value: parsed_number.value,
                    base: parsed_number.base,
                })),
            },

            ["", _, _, "", "Y", ""] => match is_zp {
                true => Ok(ParsedAddress::ZeroPageY(ParsedU8 {
                    is_address: true,
                    value: parsed_number.value as u8,
                    base: parsed_number.base,
                })),
                false => Ok(ParsedAddress::AbsoluteY(ParsedU16 {
                    is_address: true,
                    value: parsed_number.value,
                    base: parsed_number.base,
                })),
            }

            ["(", _, _, "", "X", ")"] => zp_or_err(ParsedAddress::IndexedIndirect(ParsedU8 {
                is_address: true,
                value: parsed_number.value as u8,
                base: parsed_number.base,
            })),

            ["(", _, _, ")", "Y", ""] => zp_or_err(ParsedAddress::IndirectIndexed(ParsedU8 {
                is_address: true,
                value: parsed_number.value as u8,
                base: parsed_number.base,
            })),

            ["(", _, _, ")", "", ""] => Ok(ParsedAddress::Indirect(ParsedU16 {
                is_address: true,
                value: parsed_number.value,
                base: parsed_number.base,
            })),

            //FIXME: conflicts with Normal: ABS, ZP
            ["", _, _, "", "", ""] => zp_or_err(ParsedAddress::Relative(ParsedI8 {
                is_address: true,
                value: parsed_number.value as i8,
                base: parsed_number.base,
            })),

            _ => Err(ParseError::SyntaxError)
        }
    }
}


impl Parser {
    fn sanitize_line(line: &str) -> &str {
        match line.find(';') {
            Some(i) => &line[..i], //if found, remove comment
            None => line
        }.trim() //remove blank chars, such as [space] \t \n \r ...
    }

    fn clean_input(input: &str) -> impl Iterator<Item=&str> {
        input.lines()
            .map(|l| Parser::sanitize_line(l))
            .filter(|l| !l.is_empty())
    }
}


impl Parser {
    //TODO: use js regex
    fn regex_normal_addressing(input: &str) -> ParseResult<[&str; 3]> {
        let matches = RE_NORMAL_ADDRESSING.captures(input);

        if let Some(matches) = matches {
            let mut matches = matches.iter()
                .map(|opt| {
                    match opt {
                        Some(m) => m.as_str(),
                        None => ""
                    }
                });

            matches.next(); //first match == whole match
            let rt: [&str; 3] = [
                matches.next().unwrap_or(""),
                matches.next().unwrap_or(""),
                matches.next().unwrap_or("")
            ];

            Ok(rt)
        } else {
            Err(ParseError::UnknownAddressMode)
        }
    }

    fn regex_indexed_addressing(input: &str) -> ParseResult<[&str; 6]> {
        let matches = RE_INDEXED_ADDRESSING.captures(input);

        if let Some(matches) = matches {
            let mut matches = matches.iter()
                .map(|opt| {
                    match opt {
                        Some(m) => m.as_str(),
                        None => ""
                    }
                });

            matches.next(); //first match == whole match
            let rt: [&str; 6] = [
                matches.next().unwrap_or(""),
                matches.next().unwrap_or(""),
                matches.next().unwrap_or(""),
                matches.next().unwrap_or(""),
                matches.next().unwrap_or(""),
                matches.next().unwrap_or(""),
            ];

            Ok(rt)
        } else {
            Err(ParseError::UnknownAddressMode)
        }
    }
}