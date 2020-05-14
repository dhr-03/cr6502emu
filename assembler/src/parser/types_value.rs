use crate::assembler::AssemblerInterface;


#[derive(Copy, Clone)]
pub enum AddressingMode {
    Implicit = 0,
    //U8
    Immediate,

    //U8
    ZeroPage,
    ZeroPageX,
    ZeroPageY,

    //I8
    RelativeOffset,

    //U16
    Absolute,
    AbsoluteX,
    AbsoluteY,

    //U16
    Indirect,

    //U8
    IndexedIndirect,
    IndirectIndexed,
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
    pub fn get_size(&self) -> usize {
        use ValueMode::*;

        match self {
            None => 0,
            U8(_) | I8(_) | LabelLo(_) | LabelHi(_) => 1,
            U16(_) | Label(_) => 2
        }
    }

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
}

pub struct ParsedValue {
    mode: AddressingMode,
    value: ValueMode,
    #[allow(dead_code)]
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

    pub fn resolve(&self, asm: &AssemblerInterface) -> Option<u16> {
        use ValueMode::*;

        match &self.value {
            None => Some(0), //None is reserved for errors, use get_size or is_none for checking

            U16(v) => Some(*v),
            U8(v) => Some(*v as u16),
            I8(v) => Some(*v as u16),

            Label(k) => asm.get_label_value(k.as_str()),
            LabelLo(k) => asm.get_label_value(k.as_str()),
            LabelHi(k) => asm.get_label_value(k.as_str())
                .map(|v| v >> 8)
        }
    }
}
