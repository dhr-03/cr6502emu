use js_sys::Map;

use super::super::DeviceId;

pub trait DeviceTrait {
    /// This method is called on every system + cpu tick.
    fn tick(&mut self) {}

    /// This method is called when a system reset is requested.
    fn reset_system(&mut self);

    /// This method is called when a device hard reset is requested.
    ///
    /// After executing this, the device should look brand new, without any data on it.
    fn reset_hard(&mut self);

    /// Takes a JS Map and uses that data to setup the device and/or widget.
    ///
    /// After that, updates the Map with relevant information and an update package.
    fn setup_widget(&mut self, _pkg: &Map) {
    }

    /// Updates the JS Map given with the necessary information.
    ///
    /// Every device passes the relevant data on it's own format, so a special function is needed
    /// for every type/class of devices to handle the update package.
    fn update_widget(&mut self, _pkg: &Map) {
    }

    fn device_id(&self) -> DeviceId;
}
