use wasm_bindgen::prelude::*;

use super::{Parser, ParseResult, ParsedAddress, ParseError};


#[wasm_bindgen]
pub struct Assembler {}

//public api
#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Assembler {
        Assembler {}
    }

    pub fn assemble(&self, prg: &str) { //TODO: result
        //for line in Parser::filter_input(prg) {
        //TODO: check max length
        //      check type: instruction, label, macro
        // }
    }
}

//parsers, struct members so they can emit warnings
#[wasm_bindgen]
impl Assembler {
    pub fn parse_instruction(&self, line: &str) { //TODO: result
        let space_i = *line.find(' ').get_or_insert(line.len());
        let opcode = &line[..space_i];
        let data = *line.get(space_i + 1..).get_or_insert("");

    }


    fn parse_macro(&self, line: &str) {
        unimplemented!()
    }


    fn parse_address(&self, address: &str) -> ParseResult<ParsedAddress> {
        if address.is_empty() || address == "A" {
            Ok(ParsedAddress::Implicit) //accumulator

        } else {

            Parser::parse_addr_normal(address)
                .or_else(|err| {
                    if let ParseError::UnknownAddressMode = err {
                        Parser::parse_addr_indexed(address)
                    } else {
                        Err(err)
                    }
                })
        }
    }
}

