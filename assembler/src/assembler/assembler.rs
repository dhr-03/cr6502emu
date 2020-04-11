use super::{ParseResult, ParsedAddress, NumberBase};
use super::re_patterns::{RE_NORMAL_ADDRESSING, RE_INDEXED_ADRRESING};
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


//use web_sys::console::{info_2};
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

                _ => ()
            }
        }
    }


    fn parse_macro(&self, line: &str) {}

    fn parse_address(&self, address: &str) -> ParsedAddress { //TODO: result
        if address.is_empty() || address == "A" {
            return ParsedAddress::Implicit; //accumulator
        }

        /* try normal_addressing */ {
            let re_nrm = Assembler::regex_normal_addressing(address);
            if !re_nrm[2].is_empty() {
                let is_addr = re_nrm[0] != "#";
                let mut base: NumberBase;
                let mut is_zp = false; //zero-page OR only 1 byte of data (instead of 2)

                let val = re_nrm[2];

                match re_nrm[1] {
                    "" => {
                        //TODO: check for overflows and is_zp based on the actual value
                        base = NumberBase::DEC;
                        is_zp = val.len() <= 3;
                    }

                    "$" => {
                        base = NumberBase::HEX;
                        is_zp = val.len() <= 2;
                    }

                    "b" => {
                        base = NumberBase::BIN;
                        is_zp = val.len() <= 4;
                    }

                    "%" | "lo " | "hi " => unimplemented!(), // TODO: labels

                    _ => unreachable!() //panic!("regex returned an invalid num type")
                };

                match [is_addr, is_zp] {
                    [true, true] => return ParsedAddress::ZeroPage(ParsedU8 {
                        is_address: true,
                        value: u8::from_str_radix(val, base as u32).unwrap(),
                        base,
                    }),

                    [true, false] => return ParsedAddress::Absolute(ParsedU16 {
                        is_address: true,
                        value: u16::from_str_radix(val, base as u32).unwrap(),
                        base,
                    }),

                    [false, true] => return ParsedAddress::Immediate(ParsedU8 {
                        is_address: false,
                        value: u8::from_str_radix(val, base as u32).unwrap(),
                        base,
                    }),

                    [false, false] => panic!("invalid: !is_addr + !is_zp")
                };
            }
        };

        /* try indexed_addressing */ {};


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
            matches.next().get_or_insert(""),
            matches.next().get_or_insert(""),
            matches.next().get_or_insert("")
        ];

        rt
    }

    fn regex_indexed_addressing(input: &str) -> [&str; 6] {
        unimplemented!()
    }
}
