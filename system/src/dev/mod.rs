mod traits;

pub mod mem;
pub mod io;

mod holder;

mod id;
mod js_representation;
mod factory;

pub mod utils;

pub use traits::{DeviceTrait, AddressableDeviceTrait};

pub use holder::DeviceHolder;

pub use id::DeviceId;
pub use js_representation::DeviceRepresentation;
pub use factory::DeviceFactory;

pub type BoxedDev = Box<dyn AddressableDeviceTrait>;
