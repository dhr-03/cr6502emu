mod system;
mod bus;
mod mem_manager;


pub use system::System;

use bus::Bus;
pub use mem_manager::MemManager;

type DevHolderVec = Vec<crate::dev::DeviceHolder>;