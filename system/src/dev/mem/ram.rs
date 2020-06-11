use super::super::{DeviceTrait, AddressableDeviceTrait};

pub struct Ram {
    contents: Box<[u8]>
}

impl Ram {
    pub fn with_size(size: u16) -> Self {
        Ram {
            contents: vec![0_u8; size as usize].into_boxed_slice()
        }
    }
}

impl DeviceTrait for Ram {
    fn size(&self) -> usize {
        self.contents.len()
    }

    fn reset_system(&mut self) {
        for val in &mut *self.contents {
            *val = 0;
        }
    }

    fn reset_hard(&mut self) {
        self.reset_system();
    }
}

impl AddressableDeviceTrait for Ram {
    fn read(&self, offset: u16) -> u8 {
        unsafe {
            *self.contents.get_unchecked(offset as usize)
        }
    }

    fn write(&mut self, offset: u16, value: u8) {
        unsafe {
            *self.contents.get_unchecked_mut(offset as usize) = value;
        }
    }
}