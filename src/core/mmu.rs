use super::{mbc::MBC, cartridge::Cartridge};

pub struct MMU {
    pub current_rom_bank: u8,
    mbc: MBC,
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
            mbc: MBC::new(cartridge.clone()),
            vram: Vec::new(),
            eram: Vec::new(),
            wram: Vec::new(),
            oam: Vec::new(),
            hram: Vec::new(),
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7FFF => self.mbc.read_rom(address),
            _ => 0,
        }
    }

    pub fn write_byte(&self, address: u16, value: u8) {

    }

    pub fn read_word(&self, address: u16) {

    }

    pub fn write_word(&self, address: u16, value: u16) {

    }
    
}