pub enum ParseError {
    UnknownOpcode,
    UnknownMacro,
    UnknownIdentifier,
    UnknownPattern,

    UnknownAddressingMode,
    WrongAddressingMode,

    InvalidValue,
    ValueSize,

    SyntaxError,
}

pub type ParseResult<T> = Result<T, ParseError>;
