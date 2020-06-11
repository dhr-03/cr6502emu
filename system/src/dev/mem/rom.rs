use super::super::{DeviceTrait, AddressableDeviceTrait};

pub struct Rom {
    contents: Box<[u8]>
}

impl Rom {
    pub fn with_size(size: u16) -> Self {
        Rom {
            contents: vec![0_u8; size as usize].into_boxed_slice()
        }
    }

    pub fn contents_mut(&mut self) -> &mut [u8] {
        &mut self.contents
    }
}

impl DeviceTrait for Rom {
    fn size(&self) -> usize {
        self.contents.len()
    }

    fn reset_system(&mut self) {
        //do_nothing();
    }

    fn reset_hard(&mut self) {
        for val in &mut *self.contents {
            *val = 0;
        }
    }
}

impl AddressableDeviceTrait for Rom {
    fn read(&self, offset: u16) -> u8 {
        unsafe {
            *self.contents.get_unchecked(offset as usize)
        }
    }
}