use core::hint::unreachable_unchecked;

use super::{ParseResult, ParseError,
            AddressingMode};

use super::js_regex::{js_re_nrm, js_re_inx};
use crate::assembler::{ValueMode, ParsedValue};

pub struct Parser {}

impl Parser {
    fn parse_re_addr_common(re_result: &[&str], offset: usize, is_i8: bool) -> ParseResult<ValueMode> {
        let str_value = re_result[offset + 1];

        let parse_value = |base: u32| {
            u16::from_str_radix(str_value, base)
                .map_err(|_| ParseError::SyntaxError)
        };

        let parse_value_i8 = |base: u32| {
            i8::from_str_radix(str_value, base)
                .map_err(|_| ParseError::SyntaxError)
        };


        let parse_to_valuemode = |base| {
            let value;

            if is_i8 {
                value = parse_value_i8(base)
                    .map(|v| ValueMode::I8(v));
            } else {
                value = parse_value(base)
                    .map(|v| match v <= 0xFF {
                        true => ValueMode::U8(v as u8),
                        false => ValueMode::U16(v)
                    })
            }

            value
        };

        unsafe {
            Ok(
                match re_result[offset + 0] {
                    "" => parse_to_valuemode(10)?,

                    "$" => parse_to_valuemode(16)?,

                    "b" => parse_to_valuemode(2)?,

                    "%" => ValueMode::Label(str_value.into()),

                    "lo " => ValueMode::LabelLo(str_value.into()),

                    "hi " => ValueMode::LabelHi(str_value.into()),

                    _ => unreachable_unchecked()
                }
            )
        }
    }

    pub fn parse_addr_normal(address: &str) -> ParseResult<ParsedValue> {
        let re_nrm = Parser::regex_normal_addressing(address)?;

        let is_addr = re_nrm[0] != "#";

        let value = Parser::parse_re_addr_common(&re_nrm, 1, false)?;
        let is_zp = value.is_zp();

        match [is_addr, is_zp] {
            [true, true] => Ok(
                ParsedValue::new(AddressingMode::ZeroPage, value, true)
            ),

            [true, false] => Ok(
                ParsedValue::new(AddressingMode::Absolute, value, true)
            ),

            [false, true] => Ok(
                ParsedValue::new(AddressingMode::Immediate, value, true)
            ),

            //immediate only accepts 1 byte
            [false, false] => Err(ParseError::ValueTooBig)
        }
    }


    fn __indexed_zp_or_err(addr_mode: AddressingMode, value: ValueMode, is_zp: bool) -> ParseResult<ParsedValue> {
        if is_zp {
            Ok(
                ParsedValue::new(addr_mode, value, true)
            )
        } else {
            Err(ParseError::ValueTooBig)
        }
    }

    pub fn parse_addr_indexed(address: &str) -> ParseResult<ParsedValue> {
        let re_inx = Parser::regex_indexed_addressing(address)?;

        let should_be_i8 = match re_inx[0].get(0..1) {
            Some("*") => true,
            _ => false
        };

        let value = Parser::parse_re_addr_common(&re_inx, 1, should_be_i8)?;
        let is_zp = value.is_zp();


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

            ["(", _, _, "", "X", ")"] =>
                Parser::__indexed_zp_or_err(AddressingMode::IndexedIndirect, value, is_zp),

            ["(", _, _, ")", "Y", ""] =>
                Parser::__indexed_zp_or_err(AddressingMode::IndirectIndexed, value, is_zp),

            ["(", _, _, ")", "", ""] => Ok(
                ParsedValue::new(AddressingMode::Indirect, value, true)
            ),

            ["*", _, _, "", "", ""] => match value.is_i8() {
                true => Ok(
                    ParsedValue::new(AddressingMode::RelativeOffset, value, false)
                ),
                false => Err(
                    ParseError::InvalidValue
                )
            }

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
        line.ends_with(":") && line.len() > 3 &&
            (&line[..line.len() - 1])
                .chars()
                .all(|c| char::is_alphanumeric(c) || c == '_')
    }
}