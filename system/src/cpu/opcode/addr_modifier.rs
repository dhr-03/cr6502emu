#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AddressingModifier {
    None,

    Read,
    Write,
    RMW,
}

impl AddressingModifier {
    pub fn has_read(&self) -> bool {
        match self {
            Self::Read => true,
            Self::RMW => true,

            _ => false
        }
    }

    /*
    pub fn has_write(&self) -> bool {
        match self {
            Self::Write => true,
            Self::RMW => true,

            _ => false
        }
    }
     */

    pub fn is_write(&self) -> bool {
        if let Self::Write = self {
            true
        } else {
            false
        }
    }
}
