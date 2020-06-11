pub trait DeviceTrait {
    fn tick(&mut self);

    fn reset(&mut self);
}

pub trait AddressableDeviceTrait: DeviceTrait {
    fn read(&self, offset: u16) -> u8;

    fn write(&mut self, offset: u16, value: u8) -> u8;
}