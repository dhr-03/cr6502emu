pub struct Bus {
    data: u8,
    addr: u16,

    /// Read/Write flag
    /// ---
    /// if set to true the bus was used to read, otherwise to write
    rw: bool,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            data: 0,
            addr: 0,

            rw: true,
        }
    }

    pub fn set_rw(&mut self, value: bool) {
        self.rw = value;
    }

    pub fn data(&self) -> u8 {
        self.data
    }

    pub fn addr(&self) -> u16 {
        self.addr
    }

    pub fn data_mut_ref(&mut self) -> &mut u8 {
        &mut self.data
    }

    pub fn addr_mut_ref(&mut self) -> &mut u16 {
        &mut self.addr
    }
}
