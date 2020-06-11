mod system;
mod device_holder;

mod bus;
mod mem_manager;


pub use system::System;
use device_holder::DeviceHolder;

use bus::Bus;
pub use mem_manager::MemManager;

type BoxedDev = Box<dyn crate::dev::AddressableDeviceTrait>;
type DevHolderVec = Vec<DeviceHolder>;