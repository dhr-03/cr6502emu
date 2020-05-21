use phf::{Map, phf_map};

pub const OPCODE_NONE: u8 = 0xFF;


// SRC: http://www.obelisk.me.uk/6502/instructions.html
use OPCODE_NONE as NONE;
pub static OPCODES_MAP: Map<&'static str, [u8; 12]> = phf_map! {

    // crate::parser::types::AddressingMode
    //        IMP    IMM   ZP   ZP,X  ZP,Y  REL   ABS   ABS,X ABS,Y  IND  INXD  INDX

    /* #######################  Load/Store Operations  ####################### */
    "LDA" => [NONE, 0xA9, 0xA5, 0xB5, NONE, NONE, 0xAD, 0xBD, 0xB9, NONE, 0xA1, 0xB1],
    "LDX" => [NONE, 0xA2, 0xA6, NONE, 0xB6, NONE, 0xAE, NONE, 0xBE, NONE, NONE, NONE],
    "LDY" => [NONE, 0xA0, 0xA4, 0xB4, NONE, NONE, 0xAC, 0xBC, NONE, NONE, NONE, NONE],

    "STA" => [NONE, NONE, 0x85, 0x95, NONE, NONE, 0x8D, 0x9D, 0x99, NONE, 0x81, 0x91],
    "STY" => [NONE, NONE, 0x84, 0x94, NONE, NONE, 0x8C, NONE, NONE, NONE, NONE, NONE],
    "STX" => [NONE, NONE, 0x86, NONE, 0x96, NONE, 0x8E, NONE, NONE, NONE, NONE, NONE],

    /* #######################  Register Transfers  ####################### */
    "TAX" => [0xAA, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "TAY" => [0xA8, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    "TXA" => [0x8A, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "TYA" => [0x98, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

     /* #######################  Stack Operations  ####################### */
    "TSX" => [0xBA, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "TXS" => [0x9A, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    "PHA" => [0x48, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "PHP" => [0x08, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    "PLA" => [0x68, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "PLP" => [0x28, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    /* #######################  Logical  ####################### */
    "AND" => [NONE, 0x29, 0x25, 0x35, NONE, NONE, 0x2D, 0x3D, 0x39, NONE, 0x21, 0x31],
    "EOR" => [NONE, 0x49, 0x45, 0x55, NONE, NONE, 0x4D, 0x5D, 0x59, NONE, 0x41, 0x51],
    "ORA" => [NONE, 0x09, 0x05, 0x15, NONE, NONE, 0x0D, 0x1D, 0x19, NONE, 0x01, 0x11],
    "BIT" => [NONE, NONE, 0x24, NONE, NONE, NONE, 0x2C, NONE, NONE, NONE, NONE, NONE],

    /* #######################  Arithmetic  ####################### */
    "ADC" => [NONE, 0x69, 0x65, 0x75, NONE, NONE, 0x6D, 0x7D, 0x79, NONE, 0x61, 0x71],
    "SBC" => [NONE, 0xE9, 0xE5, 0xF5, NONE, NONE, 0xED, 0xFD, 0xF9, NONE, 0xE1, 0xF1],

    "CMP" => [NONE, 0xC9, 0xC5, 0xD5, NONE, NONE, 0xCD, 0xDD, 0xD9, NONE, 0xC1, 0xD1],
    "CPX" => [NONE, 0xE0, 0xE4, NONE, NONE, NONE, 0xEC, NONE, NONE, NONE, NONE, NONE],
    "CPY" => [NONE, 0xC0, 0xC4, NONE, NONE, NONE, 0xCC, NONE, NONE, NONE, NONE, NONE],

    /* #######################  Increments & Decrements  ####################### */
    "INC" => [NONE, NONE, 0xE6, 0xF6, NONE, NONE, 0xEE, 0xFE, NONE, NONE, NONE, NONE],
    "INX" => [0xE8, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "INY" => [0xC8, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    "DEC" => [NONE, NONE, 0xC6, 0xD6, NONE, NONE, 0xCE, 0xDE, NONE, NONE, NONE, NONE],
    "DEX" => [0xCA, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "DEY" => [0x88, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    /* #######################  Shifts  ####################### */
    "ASL" => [0x0A, NONE, 0x06, 0x16, NONE, NONE, 0x0E, 0x1E, NONE, NONE, NONE, NONE],
    "LSR" => [0x4A, NONE, 0x46, 0x56, NONE, NONE, 0x4E, 0x5E, NONE, NONE, NONE, NONE],

    "ROL" => [0x2A, NONE, 0x26, 0x36, NONE, NONE, 0x2E, 0x3E, NONE, NONE, NONE, NONE],
    "ROR" => [0x6A, NONE, 0x66, 0x76, NONE, NONE, 0x6E, 0x7E, NONE, NONE, NONE, NONE],

    /* #######################  Jumps & Calls  ####################### */
    "JMP" => [NONE, NONE, NONE, NONE, NONE, NONE, 0x4C, NONE, NONE, 0x6C, NONE, NONE],
    "JSR" => [NONE, NONE, NONE, NONE, NONE, NONE, 0x20, NONE, NONE, NONE, NONE, NONE],
    "RTS" => [0x60, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    /* #######################  Branches  ####################### */
    "BCC" => [NONE, NONE, NONE, NONE, NONE, 0x90, NONE, NONE, NONE, NONE, NONE, NONE],
    "BCS" => [NONE, NONE, NONE, NONE, NONE, 0xB0, NONE, NONE, NONE, NONE, NONE, NONE],
    "BEQ" => [NONE, NONE, NONE, NONE, NONE, 0xF0, NONE, NONE, NONE, NONE, NONE, NONE],
    "BMI" => [NONE, NONE, NONE, NONE, NONE, 0x30, NONE, NONE, NONE, NONE, NONE, NONE],
    "BNE" => [NONE, NONE, NONE, NONE, NONE, 0xD0, NONE, NONE, NONE, NONE, NONE, NONE],
    "BPL" => [NONE, NONE, NONE, NONE, NONE, 0x10, NONE, NONE, NONE, NONE, NONE, NONE],
    "BVC" => [NONE, NONE, NONE, NONE, NONE, 0x50, NONE, NONE, NONE, NONE, NONE, NONE],
    "BVS" => [NONE, NONE, NONE, NONE, NONE, 0x70, NONE, NONE, NONE, NONE, NONE, NONE],

    /* #######################  Status Flag Changes  ####################### */
    "CLC" => [0x18, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "CLD" => [0xD8, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "CLI" => [0x58, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "CLV" => [0xB8, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    "SEC" => [0x38, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "SED" => [0xF8, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "SEI" => [0x78, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

    /* #######################  System Functions  ####################### */
    "BRK" => [0x00, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "NOP" => [0xEA, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],
    "RTI" => [0x40, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE],

};
