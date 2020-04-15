use super::{ParseResult, ParsedAddress, NumberBase};
use super::re_patterns::{RE_NORMAL_ADDRESSING, RE_INDEXED_ADDRESSING};
use super::{ParsedU8, ParsedU16, ParsedI8};

//TODO: use Result

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Assembler {
    //output:
}

//public api
#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Assembler {
        Assembler {}
    }

    pub fn assemble(&self, prg: &str) { //TODO: result
        for line in Assembler::filter_input(prg) {
            //TODO: check max length
            //      check type: instruction, label, macro
        }
    }
}

use crate::alert;

//parsers, struct members so they can emit warnings
#[wasm_bindgen]
impl Assembler {
    pub fn parse_instruction(&self, line: &str) { //TODO: result

        //TODO: TMP
        let line = Assembler::sanitize_line(line);

        let space_i = *line.find(' ').get_or_insert(line.len());
        let opcode = &line[..space_i];
        let data = *line.get(space_i + 1..).get_or_insert("");


        /* REMOVE */ {
            let a = self.parse_address(data);
            match a {
                ParsedAddress::Implicit => {
                    alert(&format!("{} imp", opcode));
                }
                ParsedAddress::Immediate(d) => {
                    alert(&format!("inm\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address));
                }

                ParsedAddress::Absolute(d) => {
                    alert(&format!("abs\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address));
                }

                ParsedAddress::ZeroPage(d) => {
                    alert(&format!("zp\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address));
                }

                ParsedAddress::IndexedIndirect(d) => {
                    alert(&format!("indexed ind\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address));
                }

                ParsedAddress::IndirectIndexed(d) => {
                    alert(&format!("ind indexed\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address));
                }

                ParsedAddress::Relative(d) => {
                    alert(&format!("rel\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address));
                }

                ParsedAddress::Indirect(d) => {
                    alert(&format!("ind\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address));
                }

                ParsedAddress::ZeroPageX(d) => alert(&format!("ZP X\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address)),
                ParsedAddress::ZeroPageY(d) => alert(&format!("ZP Y\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address)),
                ParsedAddress::AbsoluteX(d) => alert(&format!("ABS X\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address)),
                ParsedAddress::AbsoluteY(d) => alert(&format!("ABS Y\n{}\nbase:{}\nval:{}\naddr:{}", opcode, d.base as i32, d.value, d.is_address)),


                _ => alert("unk")
            }
        }
    }


    fn parse_macro(&self, line: &str) {
        unimplemented!()
    }


    // for simplicity, lets just return a u16, but inform if the value is actually a [u/i]8
    //TODO: I8
    fn parse_re_addr_common(&self, re_result: &[&str], offset: usize) -> (ParsedU16, bool) {
        let mut base: NumberBase;
        let mut is_zp: bool; //zero-page OR only 1 byte of data (instead of 2)

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

            "%" | "lo " | "hi " => unimplemented!(), // TODO: labels

            _ => unreachable!() //panic!("regex returned an invalid num type")
        };


        (ParsedU16 {
            is_address: true,
            value: u16::from_str_radix(str_value, base as u32).unwrap(),
            base,
        },
         is_zp
        )
    }

    fn parse_address(&self, address: &str) -> ParsedAddress { //TODO: result
        if address.is_empty() || address == "A" {
            return ParsedAddress::Implicit; //accumulator
        }

        /* try normal_addressing */

        //FIXME: panics if nothing is found. commenting it until a Result based api is made. \
        //       continuing with indexed_addressing

        /* {
            let re_nrm = Assembler::regex_normal_addressing(address);
            if !re_nrm[2].is_empty() {
                let is_addr = re_nrm[0] != "#";

                let (parsed_number, is_zp) =
                    self.parse_re_addr_common(&re_nrm, 1);


                match [is_addr, is_zp] {
                    [true, true] => return ParsedAddress::ZeroPage(ParsedU8 {
                        is_address: true,
                        value: parsed_number.value as u8,
                        base: parsed_number.base,
                    }),

                    /*[true, false] => return ParsedAddress::Absolute(ParsedU16 {
                        is_address: true,
                        value: parsed_number.value,
                        base: parsed_number.base,
                    }),*/

                    [true, false] => panic!("done by indexed_addressing")

                    [false, true] => return ParsedAddress::Immediate(ParsedU8 {
                        is_address: false,
                        value: parsed_number.value as u8,
                        base: parsed_number.base,
                    }),

                    [false, false] => panic!("invalid: !is_addr + !is_zp")
                };
            }
        }*/


        /* try indexed_addressing */{
            let re_inx = Assembler::regex_indexed_addressing(address);
            if !re_inx[2].is_empty() {
                let (parsed_number, is_zp) =
                    self.parse_re_addr_common(&re_inx, 1);


                //TODO: maybe use Future or something so we avoid creating possibly unnecessary instances ?
                let zp_or_err = |val: ParsedAddress| {
                    if !is_zp {
                        panic!("only zp")
                    }

                    val
                };

                //let zp_or = |val1: ParsedAddress, val|

                //some values have already been validated by parse_re_addr_common and
                //the regex itself, we can ignore those or assume they are valid
                match re_inx { //rel
                    // ["(", "$", "FF", ")", "X", ")"]
                    ["", _, _, "", "X", ""] => return match is_zp {
                        true => ParsedAddress::ZeroPageX(ParsedU8 {
                            is_address: true,
                            value: parsed_number.value as u8,
                            base: parsed_number.base,
                        }),
                        false => ParsedAddress::AbsoluteX(ParsedU16 {
                            is_address: true,
                            value: parsed_number.value,
                            base: parsed_number.base,
                        }),
                    },

                    ["", _, _, "", "Y", ""] => {
                        match is_zp {
                            true => return ParsedAddress::ZeroPageY(ParsedU8 {
                                is_address: true,
                                value: parsed_number.value as u8,
                                base: parsed_number.base,
                            }),
                            false => return ParsedAddress::AbsoluteY(ParsedU16 {
                                is_address: true,
                                value: parsed_number.value,
                                base: parsed_number.base,
                            }),
                        }
                    }

                    ["(", _, _, "", "X", ")"] => return zp_or_err(ParsedAddress::IndexedIndirect(ParsedU8 {
                        is_address: true,
                        value: parsed_number.value as u8,
                        base: parsed_number.base,
                    })),

                    ["(", _, _, ")", "Y", ""] => return zp_or_err(ParsedAddress::IndirectIndexed(ParsedU8 {
                        is_address: true,
                        value: parsed_number.value as u8,
                        base: parsed_number.base,
                    })),

                    ["(", _, _, ")", "", ""] => return ParsedAddress::Indirect(ParsedU16 {
                        is_address: true,
                        value: parsed_number.value,
                        base: parsed_number.base,
                    }),


                    //FIXME: conflicts with Normal: ABS, ZP
                    ["", _, _, "", "", ""] => return zp_or_err(ParsedAddress::Relative(ParsedI8 {
                        is_address: true,
                        value: parsed_number.value as i8,
                        base: parsed_number.base,
                    })),

                    _ => panic!("unknown mode")
                };
            }
        }


        panic!("err");
    }
}

//parse tools, mostly static
impl Assembler {
    fn sanitize_line(line: &str) -> &str {
        match line.find(';') {
            Some(i) => &line[..i], //if found, remove comment
            None => line
        }.trim() //remove blank chars, such as [space] \t \n \r ...
    }

    fn filter_input(input: &str) -> impl Iterator<Item=&str> {
        input.lines()
            .map(|l| Assembler::sanitize_line(l))

            //check if anything is left after sanitizing, skipping empty and only indented lines
            .filter(|l| {
                match l.chars().next() {
                    Some(ch) => true,
                    None => false
                }
            })
    }

    fn regex_normal_addressing(input: &str) -> [&str; 3] {
        let mut matches = RE_NORMAL_ADDRESSING.captures(input).expect("no re match");
        let mut matches = matches.iter()
            .map(|opt| {
                match opt {
                    Some(m) => m.as_str(),
                    None => ""
                }
            });

        matches.next(); //first match == whole match
        let mut rt: [&str; 3] = [
            matches.next().unwrap_or(""),
            matches.next().unwrap_or(""),
            matches.next().unwrap_or("")
        ];

        rt
    }

    fn regex_indexed_addressing(input: &str) -> [&str; 6] {
        let mut matches = RE_INDEXED_ADDRESSING.captures(input).expect("no re match");
        let mut matches = matches.iter()
            .map(|opt| {
                match opt {
                    Some(m) => m.as_str(),
                    None => ""
                }
            });

        matches.next(); //first match == whole match
        let mut rt: [&str; 6] = [
            matches.next().unwrap_or(""),
            matches.next().unwrap_or(""),
            matches.next().unwrap_or(""),
            matches.next().unwrap_or(""),
            matches.next().unwrap_or(""),
            matches.next().unwrap_or(""),
        ];

        rt
    }
}
