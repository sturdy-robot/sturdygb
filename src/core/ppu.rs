use super::Memory;

pub struct Ppu {}

impl Ppu {
    pub fn new() -> Self {
        Self {}
    }
}

impl Memory for Ppu {
    fn read_byte(&self, address: u16) -> u8 {
        0
    }

    fn write_byte(&self, address: u16, value: u8) {}
}
