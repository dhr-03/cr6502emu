use core::hint::unreachable_unchecked;

use super::{ParseResult, ParseError,
            AddressingMode};

use super::js_regex::{js_re_nrm, js_re_inx};
use crate::assembler::{ValueMode, ParsedValue};

pub struct Parser {}

impl Parser {
    //TODO: I8

    // for simplicity, lets just return a u16, but inform if the value is actually a [u/i]8
    fn parse_re_addr_common(re_result: &[&str], offset: usize) -> ParseResult<(ValueMode, bool)> {
        let value_mode: ValueMode;
        let is_zp: bool; //zero-page OR only 1 byte of data

        let str_value = re_result[offset + 1];

        let parse_value = |base: u32|
            u16::from_str_radix(str_value, base).map_err(|_| ParseError::SyntaxError);

        unsafe {
            match re_result[offset + 0] {
                "" => {
                    let parsed_value = parse_value(10)?;

                    value_mode = ValueMode::U16(parsed_value);
                    is_zp = parsed_value <= 0xFF;
                }

                "$" => {
                    value_mode = ValueMode::U16(
                        parse_value(16)?
                    );
                    is_zp = str_value.len() <= 2;
                }

                "b" => {
                    value_mode = ValueMode::U16(
                        parse_value(2)?
                    );
                    is_zp = str_value.len() <= 4;
                }

                "%" => {
                    value_mode = ValueMode::Label(str_value.into());
                    is_zp = false;
                }

                "lo " => {
                    value_mode = ValueMode::LabelLo(str_value.into());
                    is_zp = true;
                }

                "hi " => {
                    value_mode = ValueMode::LabelHi(str_value.into());
                    is_zp = true;
                }

                _ => unreachable_unchecked()
            }
        }


        Ok(
            (value_mode, is_zp)
        )
    }

    pub fn parse_addr_normal(address: &str) -> ParseResult<ParsedValue> {
        let re_nrm = Parser::regex_normal_addressing(address)?;

        let is_addr = re_nrm[0] != "#";

        let (value, is_zp) =
            Parser::parse_re_addr_common(&re_nrm, 1)?;


        match [is_addr, is_zp] {
            [true, true] => Ok(
                ParsedValue::new(AddressingMode::ZeroPage, value, true)
            ),

            [true, false] => Ok(
                ParsedValue::new(AddressingMode::Absolute, value, true)
            ),

            [false, true] => Ok(
                ParsedValue::new(AddressingMode::Immediate,value.to_u8_or_dft(), false)
            ),

            //immediate only accepts 1 byte
            [false, false] => Err(ParseError::ValueTooBig)
        }
    }


    pub fn parse_addr_indexed(address: &str) -> ParseResult<ParsedValue> {
        let re_inx = Parser::regex_indexed_addressing(address)?;

        let (value, is_zp) =
            Parser::parse_re_addr_common(&re_inx, 1)?;


        //unsafe closure to keep the unsafe block small
        let number_value = (|| unsafe {
            match value {
                ValueMode::U16(v) => v,
                _ => unreachable_unchecked()
            }
        })();

        let zp_or_err = |addr_mode: AddressingMode| {
            if is_zp {
                Ok(
                    ParsedValue::new(addr_mode, ValueMode::U8(number_value as u8), true)
                )
            } else {
                Err(ParseError::ValueTooBig)
            }
        };


        //some values have already been validated by parse_re_addr_common and
        //the regex itself, we can ignore those or assume they are valid
        match re_inx {
            ["", _, _, "", "X", ""] => Ok(match is_zp {
                true => ParsedValue::new(AddressingMode::ZeroPageX, value, true),
                false => ParsedValue::new(AddressingMode::AbsoluteX, value, true)
            }),

            ["", _, _, "", "Y", ""] => Ok(match is_zp {
                true => ParsedValue::new(AddressingMode::ZeroPageY, value, true),
                false => ParsedValue::new(AddressingMode::AbsoluteY, value, true)
            }),

            ["(", _, _, "", "X", ")"] => zp_or_err(AddressingMode::IndexedIndirect),
            ["(", _, _, ")", "Y", ""] => zp_or_err(AddressingMode::IndirectIndexed),

            ["(", _, _, ")", "", ""] => Ok(
                ParsedValue::new(AddressingMode::Indirect, value, true)
            ),

            ["*", _, _, "", "", ""] => Ok(
                ParsedValue::new(AddressingMode::RelativeOffset,
                                 ValueMode::I8(number_value as i8),
                                 true,
                )
            ),

            ["&", _, _, "", "", ""] => Ok(
                ParsedValue::new(AddressingMode::RelativeTarget, value, true)
            ),

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

    #[inline(always)]
    pub fn clean_input(input: &str) -> impl Iterator<Item=&str> {
        input.lines()
            .map(|l| Parser::sanitize_line(l))
            .filter(|l| !l.is_empty())
    }
}

impl Parser {
    fn substr_or_empty(txt: &str, from: usize, len: usize) -> &str {
        if len != 0 {
            &txt[from..len]
        } else {
            &""
        }
    }

    fn regex_normal_addressing(input: &str) -> ParseResult<[&str; 3]> {
        let mut bounds = [0; 6];
        js_re_nrm(input, &mut bounds);

        if bounds[5] != 0 {
            Ok([
                Parser::substr_or_empty(input, bounds[0], bounds[1]),
                Parser::substr_or_empty(input, bounds[2], bounds[3]),
                Parser::substr_or_empty(input, bounds[4], bounds[5])
            ])
        } else {
            Err(ParseError::UnknownAddressingMode)
        }
    }

    fn regex_indexed_addressing(input: &str) -> ParseResult<[&str; 6]> {
        let mut bounds = [0; 12];
        js_re_inx(input, &mut bounds);

        if bounds[5] != 0 {
            Ok([
                Parser::substr_or_empty(input, bounds[0], bounds[1]),
                Parser::substr_or_empty(input, bounds[2], bounds[3]),
                Parser::substr_or_empty(input, bounds[4], bounds[5]),
                Parser::substr_or_empty(input, bounds[6], bounds[7]),
                Parser::substr_or_empty(input, bounds[8], bounds[9]),
                Parser::substr_or_empty(input, bounds[10], bounds[11])
            ])
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
        line.ends_with(":") & &
            (&line[..line.len() - 2]).chars().all(char::is_alphanumeric)
    }
}