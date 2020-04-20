use wasm_bindgen::__rt::core::hint::unreachable_unchecked;

use super::{ParseResult, ParseError, ParsedAddress, NumberBase,
            ParsedU8, ParsedU16, ParsedI8,
            IdentifierMap};

use super::re_patterns::{RE_NORMAL_ADDRESSING, RE_INDEXED_ADDRESSING};

pub struct Parser {}

impl Parser {
    //TODO: I8

    // for simplicity, lets just return a u16, but inform if the value is actually a [u/i]8
    fn parse_re_addr_common(re_result: &[&str], offset: usize, map: &IdentifierMap) -> ParseResult<(ParsedU16, bool)> {
        let base: NumberBase;
        let is_zp: bool; //zero-page OR only 1 byte of data (instead of 2)

        let str_value = re_result[offset + 1];
        let mut value: u16;

        let parse_value = |base: u32|
            u16::from_str_radix(str_value, base).map_err(|_| ParseError::SyntaxError);

        unsafe {
            match re_result[offset + 0] {
                "" => {
                    base = NumberBase::DEC;
                    value = parse_value(10)?;
                    is_zp = value <= 0xFF;
                }

                "$" => {
                    base = NumberBase::HEX;
                    is_zp = str_value.len() <= 2;
                    value = parse_value(16)?;
                }

                "b" => {
                    base = NumberBase::BIN;
                    is_zp = str_value.len() <= 4;
                    value = parse_value(2)?;
                }

                label_type => {
                    base = NumberBase::DEC;
                    value = *map.get(str_value).ok_or(ParseError::UnknownIdentifier)?;

                    is_zp = match label_type {
                        "%" => false,
                        "lo " => true,
                        "hi " => {
                            value >>= 8;
                            true
                        }
                        _ => unreachable_unchecked() //panic!("regex returned an invalid value")
                    }
                }
            }
        }


        Ok(
            (ParsedU16 {
                is_address: true,
                value,
                base,
            },
             is_zp
            )
        )
    }

    pub fn parse_addr_normal(address: &str, map: &IdentifierMap) -> ParseResult<ParsedAddress> {
        let re_nrm = Parser::regex_normal_addressing(address)?;

        let is_addr = re_nrm[0] != "#";

        let (parsed_number, is_zp) =
            Parser::parse_re_addr_common(&re_nrm, 1, map)?;


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

            //immediate only accepts 1 byte
            [false, false] => Err(ParseError::ValueTooBig)
        }
    }


    pub fn parse_addr_indexed(address: &str, map: &IdentifierMap) -> ParseResult<ParsedAddress> {
        let re_inx = Parser::regex_indexed_addressing(address)?;

        let (parsed_number, is_zp) =
            Parser::parse_re_addr_common(&re_inx, 1, map)?;

        //inline
        let zp_or_err = |rs: ParsedAddress| {
            if is_zp {
                Ok(rs)
            } else {
                Err(ParseError::ValueTooBig)
            }
        };

        //inline
        let to_u8 = || {
            ParsedU8 {
                is_address: true,
                value: *(&parsed_number.value) as u8,
                base: parsed_number.base,
            }
        };

        //some values have already been validated by parse_re_addr_common and
        //the regex itself, we can ignore those or assume they are valid
        match re_inx {
            ["", _, _, "", "X", ""] => match is_zp {
                true => Ok(ParsedAddress::ZeroPageX(to_u8())),
                false => Ok(ParsedAddress::AbsoluteX(parsed_number))
            },

            ["", _, _, "", "Y", ""] => match is_zp {
                true => Ok(ParsedAddress::ZeroPageY(to_u8())),
                false => Ok(ParsedAddress::AbsoluteY(parsed_number)),
            }

            ["(", _, _, "", "X", ")"] => zp_or_err(ParsedAddress::IndexedIndirect(to_u8())),

            ["(", _, _, ")", "Y", ""] => zp_or_err(ParsedAddress::IndirectIndexed(to_u8())),

            ["(", _, _, ")", "", ""] => Ok(ParsedAddress::Indirect(parsed_number)),

            ["*", _, _, "", "", ""] => Ok(ParsedAddress::RelativeOffset(ParsedI8 {
                is_address: true,
                value: parsed_number.value as i8,
                base: parsed_number.base,
            })),

            ["&", _, _, "", "", ""] => Ok(ParsedAddress::RelativeTarget(parsed_number)),

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

    pub fn clean_input(input: &str) -> impl Iterator<Item=&str> {
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
            Err(ParseError::UnknownAddressingMode)
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
            Err(ParseError::UnknownAddressingMode)
        }
    }
}

// line type
impl Parser {
    #[inline(always)]
    pub fn is_macro(line: &str) -> bool {
        line.starts_with("#")
    }

    #[inline(always)]
    pub fn is_label(line: &str) -> bool {
        line.ends_with(":") &&
            (&line[..line.len() - 2]).chars().all(char::is_alphanumeric)
    }
}