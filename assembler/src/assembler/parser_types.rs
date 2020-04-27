use wasm_bindgen::__rt::core::hint::unreachable_unchecked;

pub enum ParseError {
    UnknownOpcode,
    UnknownMacro,
    //UnknownIdentifier,

    UnknownAddressingMode,
    WrongAddressingMode,

    InvalidValue,
    ValueTooBig,

    SyntaxError,
}

pub type ParseResult<T> = Result<T, ParseError>;


// ################### - ###################

pub enum AddressingMode {
    Implicit,
    Immediate, //U8

    ZeroPage, //U8
    ZeroPageX, //U8
    ZeroPageY, //U8

    RelativeOffset, //I8

    Absolute, //U16
    AbsoluteX, //U16
    AbsoluteY, //U16

    Indirect, //U16

    IndexedIndirect, //U8
    IndirectIndexed, //U8
}

pub enum ValueMode {
    None,

    U8(u8),
    I8(i8),
    U16(u16),

    Label(String),
    LabelLo(String),
    LabelHi(String),
}

impl ValueMode {
    pub fn is_zp(&self) -> bool {
        use ValueMode::*;

        match self {
            None | U8(_) | I8(_) |
            LabelLo(_) | LabelHi(_) => true,

            U16(_) | Label(_) => false
        }
    }

    pub fn is_i8(&self) -> bool {
        if let ValueMode::I8(_) = self {
            true
        } else {
            false
        }
    }

    pub fn into_u16(self) -> ValueMode {
        use ValueMode::{U8, U16};

        match &self {
            U8(v) => U16(*v as u16),
            U16(_) => self,

            _ => {
                #[cfg(debug_assertions)]
                panic!("invalid into_u16");

                unsafe {
                    //(kind of) hardcoded params (ParsedValue),
                    // if properly tested, should never happen

                    unreachable_unchecked();
                }
            }
        }
    }
}

pub struct ParsedValue {
    mode: AddressingMode,
    value: ValueMode,
    is_address: bool,
}

impl ParsedValue {
    pub fn new(mode: AddressingMode, value: ValueMode, is_address: bool) -> ParsedValue {
        #[cfg(debug_assertions)] {
            //if debug checks enabled
            use AddressingMode::*;
            use ValueMode::*;
            match mode {
                Implicit => match value {
                    None => (),
                    _ => panic!("inv mode 1")
                },

                Immediate | ZeroPage | ZeroPageX | ZeroPageY |
                IndexedIndirect | IndirectIndexed => match value {
                    U8(_) | LabelHi(_) | LabelLo(_) => (),
                    _ => panic!("inv mode 2")
                },

                RelativeOffset => match value {
                    I8(_) => (),
                    _ => panic!("inv mode 3")
                },

                Absolute | AbsoluteX | AbsoluteY |
                Indirect => match value {
                    U16(_) | Label(_) => (),
                    _ => panic!("inv mode 4")
                },
            }

            match mode {
                Implicit | RelativeOffset => assert!(!is_address),
                _ => assert!(is_address)
            }
        }

        ParsedValue {
            mode,
            value,
            is_address,
        }
    }

    pub fn addr_mode(&self) -> &AddressingMode {
        &self.mode
    }

    pub fn value(&self) -> &ValueMode {
        &self.value
    }
}

impl From<&AddressingMode> for usize {
    fn from(variant: &AddressingMode) -> Self {
        match variant {
            AddressingMode::Implicit => 0,
            AddressingMode::Immediate => 1,
            AddressingMode::ZeroPage => 2,
            AddressingMode::ZeroPageX => 3,
            AddressingMode::ZeroPageY => 4,
            AddressingMode::RelativeOffset => 5,
            AddressingMode::Absolute => 6,
            AddressingMode::AbsoluteX => 7,
            AddressingMode::AbsoluteY => 8,
            AddressingMode::Indirect => 9,
            AddressingMode::IndexedIndirect => 10,
            AddressingMode::IndirectIndexed => 11,
        }
    }
}
