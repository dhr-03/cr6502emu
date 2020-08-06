use js_sys::Map;

use super::super::{DeviceTrait, AddressableDeviceTrait, DeviceId, utils};

pub struct Ram {
    contents: Box<[u8]>,

    widget_update: bool,
}

impl Ram {
    pub fn with_size(size: u16) -> Self {
        Ram {
            contents: vec![0_u8; size as usize].into_boxed_slice(),

            widget_update: true,
        }
    }
}

impl DeviceTrait for Ram {
    fn reset_system(&mut self) {
        for val in &mut *self.contents {
            *val = 0;
        }

        self.widget_update = true;
    }

    fn reset_hard(&mut self) {
        self.reset_system();
    }

    fn update_widget(&mut self) -> Option<Map> {
        let pkg = Map::new();

        utils::js_map_add_entry_bool(&pkg, "update", self.widget_update);

        self.widget_update = false;

        Some(pkg)
    }

    fn device_id(&self) -> DeviceId {
        DeviceId::Ram
    }
}

impl AddressableDeviceTrait for Ram {
    fn size(&self) -> u16 {
        self.contents.len() as u16
    }

    fn read_unchecked(&self, offset: u16) -> u8 {
        unsafe {
            *self.contents.get_unchecked(offset as usize)
        }
    }

    fn write_unchecked(&mut self, offset: u16, value: u8) {
        self.widget_update = true;

        unsafe {
            *self.contents.get_unchecked_mut(offset as usize) = value;
        }
    }

    fn data_ptr(&mut self) -> *const u8 {
        self.widget_update = true;

        self.contents.as_ptr()
    }
}
