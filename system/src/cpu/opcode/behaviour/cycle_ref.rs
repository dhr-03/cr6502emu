use super::super::InstructionFn;

pub enum CycleRef {
    Fn(InstructionFn),
    OpHardRef(u8),
    OpSoftRef(u8),
}