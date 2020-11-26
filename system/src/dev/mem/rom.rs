use js_sys::Map;

use super::super::{DeviceTrait, AddressableDeviceTrait, DeviceId, utils};

pub struct Rom {
    contents: Box<[u8]>,

    widget_update: bool,
}

impl Rom {
    pub fn with_size(size: u16) -> Self {
        Rom {
            contents: vec![0_u8; size as usize].into_boxed_slice(),

            widget_update: true,
        }
    }
}

impl DeviceTrait for Rom {
    fn reset_system(&mut self) {
        //do_nothing();
    }

    fn reset_hard(&mut self) {
        for val in &mut *self.contents {
            *val = 0;
        }

        self.widget_update = true;
    }

    fn update_widget(&mut self, pkg: &Map) {
        utils::js_map_add_entry_bool(pkg, "update", self.widget_update);

        self.widget_update = false;
    }

    fn device_id(&self) -> DeviceId {
        DeviceId::Rom
    }
}

impl AddressableDeviceTrait for Rom {
    fn size(&self) -> u16 {
        self.contents.len() as u16
    }

    fn read_unchecked(&self, offset: u16) -> u8 {
        unsafe {
            *self.contents.get_unchecked(offset as usize)
        }
    }

    fn data_ptr(&mut self) -> *const u8 {
        self.widget_update = true;

        self.contents.as_ptr()
    }
}
