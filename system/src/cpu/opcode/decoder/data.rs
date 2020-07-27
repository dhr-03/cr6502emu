use super::super::{
    DecodedInstruction,
    behaviour::{operations, addressing},
    InstructionFn, AddressingModifier, AddressingFn, AnnotatedOpcode, AddressingActions,
};

mod __table {
    use super::{addressing, operations, DecodedInstruction};

    use addressing::*;
    use operations::*;
    // --------------------------------------------------------------------------------------------------------------------------------------------------------------
    const INVALID_OPC: DecodedInstruction = (&IMP, NOP);
    pub static DECODE_TABLE: [DecodedInstruction; 256] = [
        (&IMP, BRK), (&IXD, ORA), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZP_, ORA), (&ZP_, ASL), INVALID_OPC,
        (&IMP, PHP), (&IMM, ORA), (&A__, ASL), INVALID_OPC, INVALID_OPC, (&ABS, ORA), (&ABS, ASL), INVALID_OPC,
        //
        (&REL, BPL), (&IDX, ORA), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZPX, ORA), (&ZPX, ASL), INVALID_OPC,
        (&IMP, CLC), (&ABY, ORA), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ABX, ORA), (&ABX, ASL), INVALID_OPC,
        //
        (&ASB, JSR), (&IXD, AND), INVALID_OPC, INVALID_OPC, (&ZP_, BIT), (&ZP_, AND), (&ZP_, ROL), INVALID_OPC,
        (&IMP, PLP), (&IMM, AND), (&A__, ROL), INVALID_OPC, (&ABS, BIT), (&ABS, AND), (&ABS, ROL), INVALID_OPC,
        //
        (&REL, BMI), (&IDX, AND), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZPX, AND), (&ZPX, ROL), INVALID_OPC,
        (&IMP, SEC), (&ABY, AND), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ABX, AND), (&ABX, ROL), INVALID_OPC,
        //
        (&SRT, RTI), (&IXD, EOR), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZP_, EOR), (&ZP_, LSR), INVALID_OPC,
        (&IMP, PHA), (&IMM, EOR), (&A__, LSR), INVALID_OPC, (&ABS, JMP), (&ABS, EOR), (&ABS, LSR), INVALID_OPC,
        //
        (&REL, BVC), (&IDX, EOR), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZPX, EOR), (&ZPX, LSR), INVALID_OPC,
        (&IMP, CLI), (&ABY, EOR), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ABX, EOR), (&ABX, LSR), INVALID_OPC,
        //
        (&SRT, RTS), (&IXD, ADC), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZP_, ADC), (&ZP_, ROR), INVALID_OPC,
        (&IMP, PLA), (&IMM, ADC), (&A__, ROR), INVALID_OPC, (&IND, JMP), (&ABS, ADC), (&ABS, ROR), INVALID_OPC,
        //
        (&REL, BVS), (&IDX, ADC), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZPX, ADC), (&ZPX, ROR), INVALID_OPC,
        (&IMP, SEI), (&ABY, ADC), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ABX, ADC), (&ABX, ROR), INVALID_OPC,
        //
        INVALID_OPC, (&IXD, STA), INVALID_OPC, INVALID_OPC, (&ZP_, STY), (&ZP_, STA), (&ZP_, STX), INVALID_OPC,
        (&IMP, DEY), INVALID_OPC, (&IMP, TXA), INVALID_OPC, (&ABS, STY), (&ABS, STA), (&ABS, STX), INVALID_OPC,
        //
        (&REL, BCC), (&IDX, STA), INVALID_OPC, INVALID_OPC, (&ZPX, STY), (&ZPX, STA), (&ZPY, STX), INVALID_OPC,
        (&IMP, TYA), (&ABY, STA), (&IMP, TXS), INVALID_OPC, INVALID_OPC, (&ABX, STA), INVALID_OPC, INVALID_OPC,
        //
        (&IMM, LDY), (&IXD, LDA), (&IMM, LDX), INVALID_OPC, (&ZP_, LDY), (&ZP_, LDA), (&ZP_, LDX), INVALID_OPC,
        (&IMP, TAY), (&IMM, LDA), (&IMP, TAX), INVALID_OPC, (&ABS, LDY), (&ABS, LDA), (&ABS, LDX), INVALID_OPC,
        //
        (&REL, BCS), (&IDX, LDA), INVALID_OPC, INVALID_OPC, (&ZPX, LDY), (&ZPX, LDA), (&ZPY, LDX), INVALID_OPC,
        (&IMP, CLV), (&ABY, LDA), (&IMP, TSX), INVALID_OPC, (&ABX, LDY), (&ABX, LDA), (&ABY, LDX), INVALID_OPC,
        //
        (&IMM, CPY), (&IXD, CMP), INVALID_OPC, INVALID_OPC, (&ZP_, CPY), (&ZP_, CMP), (&ZP_, DEC), INVALID_OPC,
        (&IMP, INY), (&IMM, CMP), (&IMP, DEX), INVALID_OPC, (&ABS, CPY), (&ABS, CMP), (&ABS, DEC), INVALID_OPC,
        //
        (&REL, BNE), (&IDX, CMP), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZPX, CMP), (&ZPX, DEC), INVALID_OPC,
        (&IMP, CLD), (&ABY, CMP), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ABX, CMP), (&ABX, DEC), INVALID_OPC,
        //
        (&IMM, CPX), (&IXD, SBC), INVALID_OPC, INVALID_OPC, (&ZP_, CPX), (&ZP_, SBC), (&ZP_, INC), INVALID_OPC,
        (&IMP, INX), (&IMM, SBC), (&IMP, NOP), INVALID_OPC, (&ABS, CPX), (&ABS, SBC), (&ABS, INC), INVALID_OPC,
        //
        (&REL, BEQ), (&IDX, SBC), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ZPX, SBC), (&ZPX, INC), INVALID_OPC,
        (&IMP, SED), (&ABY, SBC), INVALID_OPC, INVALID_OPC, INVALID_OPC, (&ABX, SBC), (&ABX, INC), INVALID_OPC,
    ];
}

pub use __table::DECODE_TABLE;
