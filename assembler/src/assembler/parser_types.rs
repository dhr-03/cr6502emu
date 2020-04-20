pub enum ParseError {
    UnknownOpcode,
    UnknownMacro,
    UnknownIdentifier,

    UnknownAddressingMode,
    WrongAddressingMode,

    ValueTooBig,

    SyntaxError,
}

pub type ParseResult<T> = Result<T, ParseError>;


#[derive(Copy, Clone)]
pub enum NumberBase {
    HEX = 16,
    BIN = 2,
    DEC = 10,
}

pub struct NumberValue<T> {
    pub base: NumberBase,
    pub value: T,
    pub is_address: bool,
}

pub type ParsedU8 = NumberValue<u8>;
pub type ParsedI8 = NumberValue<i8>;
pub type ParsedU16 = NumberValue<u16>;


pub enum ParsedAddress {
    Implicit,
    Immediate(ParsedU8),

    ZeroPage(ParsedU8),
    ZeroPageX(ParsedU8),
    ZeroPageY(ParsedU8),

    RelativeOffset(ParsedI8),
    RelativeTarget(ParsedU16),

    Absolute(ParsedU16),
    AbsoluteX(ParsedU16),
    AbsoluteY(ParsedU16),

    Indirect(ParsedU16),

    IndexedIndirect(ParsedU8),
    IndirectIndexed(ParsedU8),

}


impl From<&ParsedAddress> for usize {
    //enums with data cant (for now) be converted to the variant index
    fn from(variant: &ParsedAddress) -> Self {
        match variant {
            ParsedAddress::Implicit => 0,
            ParsedAddress::Immediate(_) => 1,
            ParsedAddress::ZeroPage(_) => 2,
            ParsedAddress::ZeroPageX(_) => 3,
            ParsedAddress::ZeroPageY(_) => 4,
            ParsedAddress::RelativeOffset(_) => 5,
            ParsedAddress::RelativeTarget(_) => 5,
            ParsedAddress::Absolute(_) => 6,
            ParsedAddress::AbsoluteX(_) => 7,
            ParsedAddress::AbsoluteY(_) => 8,
            ParsedAddress::Indirect(_) => 9,
            ParsedAddress::IndexedIndirect(_) => 10,
            ParsedAddress::IndirectIndexed(_) => 11,
        }
    }
}
