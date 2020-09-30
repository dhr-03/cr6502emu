use super::{DataRegister, AddrRegister};
use crate::utils::Unsigned;

pub struct RegisterContainer {
    //Some comments: Copyright 1981-2004 by The Western Design Center, Inc. W65C02S Data Sheet

    /// Arithmetic and Logic Unit (ALU)
    /// --
    /// All arithmetic and logic operations take place within the ALU. In addition to data operations, the ALU also calculates the effective address for relative and indexed addressing modes. The result of a data operation is stored in either memory or an internal register. Carry, Negative, Overflow and Zero flags are updated following the ALU data operation.
    pub alu: DataRegister,

    /// Accumulator Register (A)
    /// --
    /// The Accumulator Register (A) is an 8-bit general purpose register which holds one of the operands and the result of arithmetic and logical operations. Reconfigured versions of this processor family could have additional accumulators.
    pub a: DataRegister,

    /// Index Registers (X and Y)
    /// --
    /// There are two 8-bit Index Registers (X and Y) which may be used as general purpose registers or to provide an index value for calculation of the effective address. When executing an instruction with indexed addressing, the microprocessor fetches the OpCode and the base address, and then modifies the address by adding the Index Register contents to the address prior to performing the desired operation.
    pub x: DataRegister,

    /// Index Registers (X and Y)
    /// --
    /// There are two 8-bit Index Registers (X and Y) which may be used as general purpose registers or to provide an index value for calculation of the effective address. When executing an instruction with indexed addressing, the microprocessor fetches the OpCode and the base address, and then modifies the address by adding the Index Register contents to the address prior to performing the desired operation.
    pub y: DataRegister,

    /// Processor Status Register (P)
    /// --
    /// The 8-bit Processor Status Register (P) contains status flags and mode select bits. The Carry (C), Negative (N), Overflow (V) and Zero (Z) status flags serve to report the status of ALU operations. These status flags are tested with Conditional Branch instructions. The Decimal (D) and IRQB disable (I) are used as mode select flags. These flags are set by the program to change microprocessor operations. Bit 5 is available for a user status or mode bit.
    pub p: DataRegister,

    /// Program Counter Register (PC)
    /// --
    /// The 16-bit Program Counter Register (PC) provides the addresses which are used to step the microprocessor through sequential program instructions. This register is incremented each time an instruction or operand is fetched from program memory.
    pub pc: AddrRegister,

    /// Stack Pointer Register (S)
    /// --
    /// The Stack Pointer Register (S) is an 8-bit register which is used to indicate the next available location in the stack memory area. It serves as the effective address in stack addressing modes as well as subroutine and interrupt processing.
    pub s: DataRegister,

    /// Internal (custom) Temp Register
    /// ---
    /// Used by the CPU to store values temporally.
    pub itr: DataRegister,
}

impl RegisterContainer {
    pub fn new() -> Self {
        let mut tmp = RegisterContainer {
            alu: DataRegister::zero(),

            a: DataRegister::zero(),
            x: DataRegister::zero(),
            y: DataRegister::zero(),

            p: DataRegister::zero(),

            pc: AddrRegister::zero(),
            s: DataRegister::zero(),

            itr: DataRegister::zero(),
        };

        tmp.reset();

        tmp
    }

    //TODO reset vector
    pub fn reset(&mut self) {
        self.alu = 0;

        self.a = 0;
        self.x = 0;
        self.y = 0;

        self.p = 0;

        self.pc = 0;
        self.s = 0;

        self.itr = 0;
    }
}
