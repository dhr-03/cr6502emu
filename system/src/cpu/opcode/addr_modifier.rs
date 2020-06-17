#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AddressingModifier {
    None,

    Read,
    Write,
    RMW,

    PlaceHolder, //TODO: remove
}