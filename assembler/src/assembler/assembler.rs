use wasm_bindgen::prelude::wasm_bindgen;

use std::collections::HashMap;
use core::hint::unreachable_unchecked;

use crate::assembler::{AssemblerInterface};
use crate::assembler::components::{CodeItemTrait, Instruction, Label, MacroFactory};

use crate::parser::Parser;
use crate::parser::types::ParseError;

use crate::js_logger::Logger;

use crate::lang::LoggerMessage;


#[wasm_bindgen]
pub struct Assembler {
    //hashmaps dont deallocate all the memory after deleting items, keeping it as a member can save a few os calls
    identifiers: HashMap<String, u16>
}

//public api
#[wasm_bindgen]
impl Assembler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Assembler {
        Assembler {
            identifiers: HashMap::new(),
        }
    }

    pub fn assemble(&mut self, lines: &str, rom: &mut [u8], rom_start: u16) -> bool {
        let mut interface = AssemblerInterface::new(
            rom,
            &mut self.identifiers,
            rom_start,
        );

        //Stage 1: parse into structs
        let stage_1 = Parser::clean_input(lines).map(|(num, line)| {
            Logger::set_current_line(num + 1);

            let rs;

            if Parser::is_instruction(line) {
                rs = Instruction::from_str_boxed(line)
            } else if Parser::is_label(line) {
                rs = Label::from_str_boxed(line)
            } else if Parser::is_macro(line) {
                rs = MacroFactory::from_str_boxed(&line[1..line.len()])
            } else {
                rs = Err(ParseError::UnknownPattern)
            }

            (num, rs)
        });

        //Stage 2: build and remove labels, log errs/warns/infs if necessary
        let mut st2_ok = true;

        Logger::reset();
        let stage_2 = stage_1.filter(|(num, item)| {
            Logger::set_current_line(*num + 1);

            if let Ok(item) = item {
                let item_size = item.get_size() as u16;
                interface.increase_offset(item_size);

                let (ok, keep) = item.process(&mut interface);

                if !ok {
                    st2_ok = false;
                }

                keep
            } else {
                if let Err(e) = item {
                    if !Logger::msg_handled() {
                        Logger::err_msg(e.to_logger_msg());
                    }

                    st2_ok = false;
                }

                false
            }
        });

        //Stage 3.0: Iterators are lazy evaluated, so we need to execute it an collect it before continuing
        let stage_3: Vec<(usize, Box<dyn CodeItemTrait>)> = stage_2
            .map(|(num, i)| {
                (num, i.unwrap_or_else(|_| unsafe { unreachable_unchecked() })) //filtered in stg 2
            })
            .collect();


        //Stage 3.1: Now we know the final ROM size, lets check if it fits and if everything is ok
        let mut rsv_write_ok = false;

        Logger::set_current_line_str("EVAL");

        if interface.write_ptr() < 1 {
            if st2_ok {
                Logger::err_msg(LoggerMessage::AsmErrEmptyInput);
            }
        } else if interface.write_ptr() > interface.rom_size() {
            Logger::err_msg(LoggerMessage::AsmErrRomTooSmall);
        } else if st2_ok {
            //Stage 4: Write

            interface.reset_counter();

            rsv_write_ok = true; //resolve and write

            for (num, item) in stage_3 {
                Logger::set_current_line(num + 1);

                if let Err(e) = item.execute(&mut interface) {
                    if !Logger::msg_handled() {
                        Logger::err_msg(e.to_logger_msg());
                    }

                    rsv_write_ok = false;
                }
            }
        }


        Logger::set_current_line_str("EOF");

        if rsv_write_ok {
            Logger::explained_info_i32(LoggerMessage::AsmInfoAsmSuccess, interface.write_ptr() as i32);

            Self::clear_unused_rom(&mut interface);

            self.identifiers.clear();

            true
        } else {
            Logger::err_msg(LoggerMessage::AsmErrAsmFailed);

            self.identifiers.clear();

            false
        }
    }

    fn clear_unused_rom(asm: &mut AssemblerInterface) {
        for _ in asm.write_ptr()..asm.rom_size() {
            asm.write(0xEA); //NOP
        }
    }
}


