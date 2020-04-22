use core::hint::unreachable_unchecked;
use std::collections::HashMap;

pub struct LabelValue {
    pub value: Option<u16>,
    pub usages_lo: Vec<u16>,
    pub usages_hi: Vec<u16>,
}

impl LabelValue {
    fn new(value: u16) -> LabelValue {
        LabelValue {
            value: Some(value),
            usages_lo: Vec::new(),
            usages_hi: Vec::new(),
        }
    }

    fn without_value() -> LabelValue {
        LabelValue {
            value: None,
            usages_lo: Vec::new(),
            usages_hi: Vec::new(),
        }
    }
}

pub struct LabelManager {
    pub map: HashMap<String, LabelValue>,

}

impl LabelManager {
    pub fn new() -> LabelManager {
        LabelManager {
            map: HashMap::new()
        }
    }


    #[inline(always)]
    pub fn insert(&mut self, name: &str, value: u16) {
        self.map.insert(name.to_string(), LabelValue::new(value));
    }


    fn get_or_insert_mut(&mut self, name: &str) -> &mut LabelValue {
        if !self.map.contains_key(name) {
            self.map.insert(name.to_owned(), LabelValue::without_value());
        }

        unsafe {
            match self.map.get_mut(name) {
                Some(v) => v,
                None => unreachable_unchecked()
            }
        }
    }


    pub fn get_or_sched_lo(&mut self, name: &str, addr: u16) -> Option<u8> {
        let item = self.get_or_insert_mut(name);

        if let Some(v) = item.value {
            Some(v as u8)
        } else {
            item.usages_lo.push(addr);
            None
        }
    }

    pub fn get_or_sched_hi(&mut self, name: &str, addr: u16) -> Option<u8> {
        let item = self.get_or_insert_mut(name);

        if let Some(v) = item.value {
            Some((v >> 8) as u8)
        } else {
            item.usages_hi.push(addr);
            None
        }
    }


    pub fn get_or_sched(&mut self, name: &str, addr: u16) -> Option<u16> {
        let item = self.get_or_insert_mut(name);

        if let Some(v) = item.value {
            Some(v)
        } else {
            item.usages_lo.push(addr);
            item.usages_hi.push(addr + 1);
            None
        }
    }
}