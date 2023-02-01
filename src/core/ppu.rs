use super::Memory;

pub struct Ppu {
    lcdc: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Self { lcdc: 0 }
    }

    fn vblank(&mut self) {}
}

impl Memory for Ppu {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x40 => 0xFF, // todo: implement this
            0x48 => self.lcdc,
            _ => {
                println!("Address not implemented {:04X}", address);
                0xFF
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            _ => println!("Address not implemented {:04X}", address),
        }
    }
}
