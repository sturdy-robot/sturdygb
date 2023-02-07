use super::Memory;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new(div: u8) -> Self {
        Self {
            div,
            tima: 0,
            tma: 0,
            tac: 0xF8,
        }
    }

    pub fn run() {
        
    }
}

impl Memory for Timer {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => { }
            0xFF06 => { }
            0xFF07 => { }
            _ => unreachable!(),
        };
    }
}
