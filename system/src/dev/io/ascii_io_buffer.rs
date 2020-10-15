use wasm_bindgen::JsValue;
use js_sys::{Map, Array};

use super::super::{DeviceTrait, AddressableDeviceTrait, DeviceId};

pub struct AsciiIOBuffer {
    ascii_in: Array,

    // This should be a shared string but JsStrings don't appear to have any mutation methods on the Rust side.
    // A char array is the most "efficient" way of doing it that i have found reading the api docs.
    ascii_out: Array,
}

impl AsciiIOBuffer {
    pub fn new() -> Self{
        AsciiIOBuffer {
            ascii_in: Array::new(),
            ascii_out: Array::new(),
        }
    }
}

impl DeviceTrait for AsciiIOBuffer {
    fn reset_system(&mut self) {
        while self.ascii_in.length() > 0 {
            self.ascii_in.pop();
        }

        while self.ascii_out.length() > 0 {
            self.ascii_out.pop();
        }
    }

    fn reset_hard(&mut self) {
        self.reset_system();
    }

    fn setup_widget(&mut self, _data: &Map) -> Option<Map> {
        let pkg = Map::new();

        pkg.set(&JsValue::from_str("in"), &self.ascii_in);
        pkg.set(&JsValue::from_str("out"), &self.ascii_out);

        Some(pkg)
    }

    fn update_widget(&mut self) -> Option<Map> {
        None
    }

    fn device_id(&self) -> DeviceId {
        DeviceId::AsciiIOBuffer
    }
}

impl AddressableDeviceTrait for AsciiIOBuffer {
    fn size(&self) -> u16 {
        1
    }

    fn read_unchecked(&self, _offset: u16) -> u8 {
       self.ascii_in.pop()
           .as_f64()
           .map_or(0, |val| val as u8)
    }

    fn write_unchecked(&mut self, _offset: u16, value: u8) {
        let char_arr = [value];
        let char_str = &std::str::from_utf8(&char_arr);

        if let Ok(char) = char_str {
            let char_js = JsValue::from_str(char);

            self.ascii_out.push(&char_js);
        }
    }
}
