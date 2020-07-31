use super::{BoxedDev, DeviceId};
use crate::dev::{mem, io};

// We cant have constructors in traits.
pub struct DeviceFactory {}

impl DeviceFactory {
    /// Creates a new device in the form of `Result<BoxedDevice>` (`Box<dyn AddressableDeviceTrait>`)
    ///  with size `size`
    ///
    /// If the device has a fixed size, `size` must be a 0, and the final size can be checked with `self.size()`.
    ///
    /// # Returns
    ///  Returns `Ok(...)` if the device size is valid and `Err(())` if not.
    ///
    /// ## Err type
    /// In the future, `E` might contain some kind of error, but for now, the only error is an invalid size.
    pub fn with_size(dev_type: DeviceId, size: u16) -> Result<BoxedDev, ()> {
        if size == 0 && !Self::dev_has_fixed_size(&dev_type) {
            Err(())
        } else {
            match dev_type {
                DeviceId::CPU => Err(()),

                DeviceId::Rom => {
                    Ok(Box::new(
                        mem::Rom::with_size(size)
                    ))
                }

                DeviceId::Ram => {
                    Ok(Box::new(
                        mem::Ram::with_size(size)
                    ))
                }
            }
        }
    }

    fn dev_has_fixed_size(dev_id: &DeviceId) -> bool {
        dev_id.fixed_size_of().is_some()
    }
}
