use wasm_bindgen::prelude::wasm_bindgen;

use crate::parser::types::{AddressingMode, ParseError};

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum LoggerMessage {
    // Parser
    PrsErrNumParse,
    PrsErrNumParseI8,

    PrsErrExpectedZP,

    // Assembler
    AsmErrLblNeverDef,
    AsmErrLblReDef,
    AsmErrLblShort,
    AsmErrLblLong,

    AsmErrAsmFailed,
    AsmErrRomTooSmall,

    AsmErrUnknownOpcode,
    AsmErrAddrMode,
    AsmErrTargetTooFar,

    AsmErrEmptyInput,

    AsmInfoAsmSuccess,

    // Macros
    McrErrNonAscii,
    McrErrNumParse,

    // Parse Error
    PreUnknownOpcode,
    PreUnknownMacro,
    PreUnknownPattern,
    PreUnknownIdentifier,

    PreUnknownAddressingMode,
    PreWrongAddressingMode,

    PreInvalidValue,
    PreValueSize,

    PreSyntaxError,
}

impl ParseError {
    pub fn to_logger_msg(&self) -> LoggerMessage {
        use ParseError::*;
        use LoggerMessage::*;

        match self {
            UnknownOpcode => PreUnknownOpcode,
            UnknownMacro => PreUnknownMacro,
            UnknownPattern => PreUnknownPattern,
            UnknownIdentifier => PreUnknownIdentifier,

            UnknownAddressingMode => PreUnknownAddressingMode,
            WrongAddressingMode => PreWrongAddressingMode,

            InvalidValue => PreInvalidValue,
            ValueSize => PreValueSize,

            SyntaxError => PreSyntaxError,
        }
    }
}

impl AddressingMode {
    pub fn to_str(&self) -> &'static str {
        use AddressingMode::*;
        match self {
            Implicit => "Implicit",
            Immediate => "Immediate",

            ZeroPage => "ZeroPage",
            ZeroPageX => "ZeroPageX",
            ZeroPageY => "ZeroPageY",

            RelativeOffset => "RelativeOffset",
            RelativeTarget => "RelativeTarget",

            Absolute => "Absolute",
            AbsoluteX => "AbsoluteX",
            AbsoluteY => "AbsoluteY",

            Indirect => "Indirect",

            IndexedIndirect => "IndexedIndirect",
            IndirectIndexed => "IndirectIndexed",
        }
    }
}
