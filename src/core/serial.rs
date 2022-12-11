pub struct Serial {
    sb: u8,
    pub sc: u8,
    pub serial_data: Vec<u8>
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0,
            serial_data: Vec::new(),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => {
                self.sb = value;
                self.serial_data.push(value);
            }
            0xFF02 => {
                self.sc = value;
            },
            _ => unreachable!(),
        };
    }
}