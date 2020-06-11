mod behaviour;
mod decoder;

pub use behaviour::{addressing, operations};

pub use behaviour::cycle_ref::CycleRef;

pub use decoder::Decoder;

// ###################
//TODO: check code for non type references
pub type CycleFn = fn(inter: &mut super::CPUInterface);

pub type AddressingCycle = [CycleRef];
pub type AddressingCycleRef = &'static [CycleRef];
pub type AddressingActions = [&'static AddressingCycle];

pub type InstructionFn = CycleFn;
pub type InstructionActions = [InstructionFn];

pub type DecodedInstruction = (&'static AddressingActions, &'static InstructionActions);