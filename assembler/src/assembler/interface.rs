use std::collections::HashMap;

pub struct AssemblerInterface<'a> {
    pub rom: &'a mut [u8],
    pub offset: u16,
    pub map: &'a mut HashMap<String, u16>,
}

impl AssemblerInterface<'_> {
    pub fn write(&mut self, b: u8) {
        self.rom[self.offset as usize] = b;

        self.offset += 1;
    }

    pub fn contains_label(&self, k: &str) -> bool {
        self.map.contains_key(k)
    }

    pub fn get_label_value(&self, k: &str) -> Option<u16> {
        self.map.get(k)
            .cloned() //&u16 -> u16
    }

    pub fn insert_label(&mut self, k: &str, v: u16) {
        self.map.insert(String::from(k), v);
    }

    pub fn increase_offset(&mut self, amm: u16) {
        self.offset += amm;
    }

    pub fn reset_counter(&mut self) {
        self.offset = 0;
    }
}
