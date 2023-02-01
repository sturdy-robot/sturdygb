use super::Memory;

pub struct Serial {
    sb: u8,
    sc: u8,
    serial_data: Vec<u8>,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0,
            serial_data: Vec::new(),
        }
    }

    pub fn get_serial_message(&mut self) -> Option<String> {
        if !self.serial_data.is_empty() {
            let serial_string = self.serial_data.escape_ascii().to_string();
            Some(serial_string.to_owned())
        } else {
            None
        }
    }
}

impl Memory for Serial {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.sb,
            0xF002 => self.sc,
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => {
                self.sb = value;
                self.serial_data.push(value);
            }
            0xFF02 => {
                self.sc = value;
            }
            _ => unreachable!(),
        };
    }
}
