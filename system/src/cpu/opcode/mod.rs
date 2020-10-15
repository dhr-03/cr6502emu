mod addressing_actions;
mod operations_actions;

mod decoder;
mod addr_modifier;

mod shared;

pub use addressing_actions::export as addressing;
pub use operations_actions::export as operations;

pub use decoder::Decoder;

pub use addr_modifier::AddressingModifier;

// ###################
pub type InstructionFn = fn(inter: &mut super::CPUInterface);
pub type AddressingFn = fn(inter: &mut super::CPUInterface, op_fn: InstructionFn, op_mod: AddressingModifier);

pub type AnnotatedOpcode = (InstructionFn, AddressingModifier);
pub type AddressingActions = [AddressingFn];

pub type DecodedInstruction = (&'static AddressingActions, AnnotatedOpcode);
