use crate::dev::BoxedDev;
use std::ops::Range;

pub struct DeviceHolder {
    range: Range<u16>,

    device: BoxedDev,

    uid: u16,
}

impl DeviceHolder {
    pub fn new(device: BoxedDev, start: u16, end: u16, uid: u16) -> Self {
        DeviceHolder {
            range: Range {
                start,
                end,
            },

            device,
            uid,
        }
    }

    pub fn device_mut(&mut self) -> &mut BoxedDev {
        &mut self.device
    }

    pub fn device(&self) -> &BoxedDev {
        &self.device
    }

    pub fn range(&self) -> &Range<u16> {
        &self.range
    }

    pub fn uid(&self) -> u16 {
        self.uid
    }
}

