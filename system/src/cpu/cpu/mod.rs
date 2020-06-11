mod cpu;
mod interface;

mod current_opcode;

pub use cpu::CPU;
pub use interface::CPUInterface;
use current_opcode::CurrentOpcode;