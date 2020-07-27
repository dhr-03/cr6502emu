pub trait DeviceTrait {
    /// This method is called on every system + cpu tick.
    fn tick(&mut self) {}

    /// This method is called when a system reset is requested.
    fn reset_system(&mut self);

    /// This method is called when a device hard reset is requested.
    ///
    /// After executing this, the device should look brand new, without any data on it.
    fn reset_hard(&mut self);
}
