use super::{cartridge::Cartridge};

pub struct MMU {
    pub current_rom_bank: u8,
    mbc: Cartridge,
    vram: Vec<u8>,
    eram: Vec<u8>,
    wram: Vec<u8>,
    oam: Vec<u8>,
    hram: Vec<u8>,
}

impl MMU {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            current_rom_bank: 0,
            mbc: cartridge,
            vram: Vec::new(),
            eram: Vec::new(),
            wram: Vec::new(),
            oam: Vec::new(),
            hram: Vec::new(),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7FFF => self.mbc.read_rom(address),
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x7FFF => self.mbc.write_rom(address, value),
            _ => todo!(),
        };
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value * 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }
    
}