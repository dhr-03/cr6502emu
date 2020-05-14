use wasm_bindgen::prelude::wasm_bindgen;

use std::collections::HashMap;
use core::hint::unreachable_unchecked;

use std::ptr;

use crate::assembler::{AssemblerInterface};
use crate::assembler::components::{CodeItemTrait, Instruction, Label, MacroFactory};

use crate::parser::{Parser, ParsedValue, ParseError, ParseResult, AddressingMode, ValueMode};

use crate::js_logger::{Logger,
                       err_code, info_code_i32, warn_code,
                       err_msg, warn_msg};

use crate::lang::assembler as lang;
use crate::lang::assembler::ERR_ASM_FAILED;


#[wasm_bindgen]
pub struct Assembler {
    rom_offset: u16,
    //hashmap doesn't deallocate all the memory after deleting items, keeping it as a member can save a few os calls
    identifiers: HashMap<String, u16>,

    test_tmp: [u8; 30],
    write_offset: u16,
}

//public api
#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen(constructor)]
    pub fn new(rom_start: u16) -> Assembler {
        Assembler {
            rom_offset: rom_start,
            identifiers: HashMap::new(),

            test_tmp: [0; 30],
            write_offset: 0,
        }
    }

    pub fn assemble(&mut self, lines: &str) -> *const u8 {
        let mut interface = AssemblerInterface {
            rom: &mut self.test_tmp,
            offset: 0,
            map: &mut self.identifiers,
        };

        //Stage 1: parse into structs
        let stage_1 = Parser::clean_input(lines).map(|(num, line)| {
            if Parser::is_instruction(line) {
                Instruction::from_str_boxed(line)
            } else if Parser::is_label(line) {
                Label::from_str_boxed(line)
            } else if Parser::is_macro(line) {
                MacroFactory::from_str_boxed("")
            } else {
                Err(ParseError::UnknownPattern)
            }
        });

        //Stage 2: build and remove labels, log errs/warns/infs if necessary
        let mut st2_ok = true;

        let stage_2 = stage_1.filter(|item| {
            if let Ok(item) = item {
                let item_size = item.get_size() as u16;
                interface.increase_offset(item_size);

                item.process(&mut interface) //-> bool, keep?
            } else {
                if let Err(e) = item {
                    //TODO log

                    st2_ok = false;
                }

                false
            }
        });

        //Stage 3.0: Iterators are lazy evaluated, so we need to execute it an collect it before continuing
        let stage_3: Vec<Box<dyn CodeItemTrait>> = stage_2
            .map(|i| i.unwrap_or_else(|_| unsafe { unreachable_unchecked() })) //filtered in stg 2
            .collect();


        //Stage 3.1: Now we know the final ROM size, lets check if it fits and if everything is ok
        let mut rsv_write_ok = false;

        if interface.offset < 1 {
            err_msg(lang::ERR_EMPTY_INPUT);
        } else if (interface.offset as usize) >= interface.rom.len() {
            err_msg(lang::ERR_ROM_TOO_SMALL);
        } else if st2_ok {
            //Stage 4.1: Write

            interface.reset_counter();

            rsv_write_ok = true; //resolve and write

            for item in stage_3 {
                if let Err(e) = item.execute(&mut interface) {
                    //TODO log

                    rsv_write_ok = false;

                    break;
                }
            }
        }


        Logger::set_current_line_str("EOL");

        if rsv_write_ok {
            info_code_i32(lang::INFO_ASM_SUCCESS_1,
                          interface.offset as i32,
                          lang::INFO_ASM_SUCCESS_2);

            Self::clear_unused_rom(&mut interface);

            self.identifiers.clear();

            &self.test_tmp[0]
        } else {
            err_msg(lang::ERR_ASM_FAILED);

            self.identifiers.clear();

            ptr::null()
        }
    }

    fn clear_unused_rom(asm: &mut AssemblerInterface) {
        for _ in asm.offset as usize..asm.rom.len() {
            asm.write(0); //BRK
        }
    }
}


