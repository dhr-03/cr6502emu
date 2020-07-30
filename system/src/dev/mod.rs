mod traits;

pub mod mem;
pub mod io;

mod holder;

mod id;
mod representation;
mod factory;

pub use traits::{DeviceTrait, AddressableDeviceTrait};

pub use holder::DeviceHolder;

pub use id::DeviceId;
pub use representation::DeviceRepresentation;
pub use factory::DeviceFactory;

pub type BoxedDev = Box<dyn AddressableDeviceTrait>;
