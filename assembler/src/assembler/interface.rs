use std::collections::HashMap;

pub struct AssemblerInterface<'a> {
    rom: &'a mut [u8],
    offset: u16,
    map: &'a mut HashMap<String, u16>,
}

impl<'a> AssemblerInterface<'a> {
    pub fn new(rom: &'a mut [u8], map: &'a mut HashMap<String, u16>) -> AssemblerInterface<'a> {
        AssemblerInterface {
            rom,
            offset: 0,
            map
        }
    }

    pub fn write(&mut self, b: u8) {
        self.rom[self.offset as usize] = b;

        self.offset += 1;
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

    pub fn offset(&self) -> u16 {
        self.offset
    }

    pub fn increase_offset(&mut self, amm: u16) {
        self.offset += amm;
    }

    pub fn rom_size(&self) -> u16 {
        self.rom.len() as u16
    }

    pub fn reset_counter(&mut self) {
        self.offset = 0;
    }
}
