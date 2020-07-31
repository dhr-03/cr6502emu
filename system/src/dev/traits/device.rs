use js_sys::Map;

pub trait DeviceTrait {
    /// This method is called on every system + cpu tick.
    fn tick(&mut self) {}

    /// This method is called when a system reset is requested.
    fn reset_system(&mut self);

    /// This method is called when a device hard reset is requested.
    ///
    /// After executing this, the device should look brand new, without any data on it.
    fn reset_hard(&mut self);

    /// Returns a JS Map containing the necessary data to update the device's widget if possible
    /// or a None (JS undefined).
    ///
    /// Every device passes the relevant data on it's own format, so a special function is needed
    /// for every type/class of devices to handle the update package.
    fn update_widget(&self) -> Option<Map>;
}
