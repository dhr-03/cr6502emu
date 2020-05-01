use crate::assembler::{ParseError, AddressingMode};

type STR = &'static str;
/*
    Format: [ERR|WARN|INF]_{DESC}(_[1..])?
*/


// ###################################################################################
//                                      Parser
// ###################################################################################

pub mod parser {
    use super::STR;

    pub const ERR_NUM_PARSE_1: STR = "Failed to parse value";
    pub const ERR_NUM_PARSE_2: STR = "as";
}

// ###################################################################################
//                                     Assembler
// ###################################################################################

pub mod assembler {
    use super::STR;

    pub const ERR_LBL_NEVER_DEF_1: STR = "Label";
    pub const ERR_LBL_NEVER_DEF_2: STR = "is never defined";

    pub const ERR_LBL_RE_DEF_1: STR = ERR_LBL_NEVER_DEF_1;
    pub const ERR_LBL_RE_DEF_2: STR = "has already been defined";

    pub const ERR_LBL_SHORT_1: STR = ERR_LBL_NEVER_DEF_1;
    pub const ERR_LBL_SHORT_2: STR = "it's too short";

    pub const ERR_ASM_FAILED: STR = "Assemble failed";

    pub const ERR_ROM_TOO_SMALL: STR = "The program ROM is too small";

    pub const ERR_ADDR_MODE_1: STR = "Opcode:";
    pub const ERR_ADDR_MODE_2: STR = "is incompatible with";

    pub const ERR_UNKNOWN_OPCODE: STR = "Unknown opcode";

    pub const ERR_MACROS_TODO: STR = "Macros are not implemented yet";

    pub const ERR_EMPTY_INPUT: STR = "Nothing to assemble";


    pub const WARN_REL_OOB: STR = "Target it's outside of the address range of the current ROM";

    pub const INFO_ASM_SUCCESS_1: STR = "Assembled into";
    pub const INFO_ASM_SUCCESS_2: STR = "bytes";
}

// ###################################################################################
//                                     to_str
// ###################################################################################

impl ParseError {
    pub fn to_str(&self) -> STR {
        use ParseError::*;
        match self {
            UnknownOpcode => "UnknownOpcode",
            UnknownMacro => "UnknownMacro",

            UnknownAddressingMode => "UnknownAddressingMode",
            WrongAddressingMode => "WrongAddressingMode",

            InvalidValue => "InvalidValue",
            ValueTooBig => "ValueTooBig",

            SyntaxError => "SyntaxError",
        }
    }
}

impl AddressingMode {
    pub fn to_str(&self) -> STR {
        use AddressingMode::*;
        match self {
            Implicit => "Implicit",
            Immediate => "Immediate",

            ZeroPage => "ZeroPage",
            ZeroPageX => "ZeroPageX",
            ZeroPageY => "ZeroPageY",

            RelativeOffset => "RelativeOffset",

            Absolute => "Absolute",
            AbsoluteX => "AbsoluteX",
            AbsoluteY => "AbsoluteY",

            Indirect => "Indirect",

            IndexedIndirect => "IndexedIndirect",
            IndirectIndexed => "IndirectIndexed",
        }
    }
}