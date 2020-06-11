mod generic;
mod container;

pub use generic::GenericRegister;

pub type DataRegister = GenericRegister<u8>;
pub type AddrRegister = GenericRegister<u16>;

pub use container::RegisterContainer;

