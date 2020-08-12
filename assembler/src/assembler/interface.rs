use std::collections::HashMap;

pub struct AssemblerInterface<'a> {
    rom: &'a mut [u8],
    write_ptr: u16,
    map: &'a mut HashMap<String, u16>,

    rom_start: u16,
}

impl<'a> AssemblerInterface<'a> {
    pub fn new(rom: &'a mut [u8], map: &'a mut HashMap<String, u16>, rom_start: u16) -> AssemblerInterface<'a> {
        AssemblerInterface {
            rom,
            write_ptr: 0,
            map,

            rom_start,
        }
    }

    pub fn write(&mut self, b: u8) {
        self.rom[self.write_ptr as usize] = b;

        self.write_ptr += 1;
    }

    /*pub fn contains_label(&self, k: &str) -> bool {
        self.map.contains_key(k)
    }*/

    pub fn get_label_value(&self, k: &str) -> Option<u16> {
        self.map.get(k)
            .cloned() //&u16 -> u16
    }

    pub fn insert_label(&mut self, k: &str, v: u16) {
        self.map.insert(String::from(k), v);
    }

    pub fn write_ptr(&self) -> u16 {
        self.write_ptr
    }

    pub fn increase_offset(&mut self, amm: u16) {
        self.write_ptr += amm;
    }

    pub fn rom_size(&self) -> u16 {
        self.rom.len() as u16
    }

    pub fn rom_start(&self) -> u16 {
        self.rom_start
    }

    pub fn reset_counter(&mut self) {
        self.write_ptr = 0;
    }
}
