use super::cartridge::{Cartridge, CartridgeHeader, MBCTypes};

pub struct MMU {
    pub current_rom_bank: u8,
    pub mbc: Cartridge,
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
            0x0000 ..= 0x7FFF => self.read_rom(address),
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x7FFF => self.write_rom(address, value),
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

    pub fn read_rom(&self, address: u16) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => self.mbc.rom_data[address as usize],
            MBCTypes::MBC1 => {
                match address {
                    0x4000 => 0 as u8,
                    0x6000 => 0 as u8,
                    0xA000 => 0 as u8,
                    _ => 0 as u8,
                }
            },
            MBCTypes::MBC2 => todo!(),
            MBCTypes::MMM01 => todo!(),
            MBCTypes::MBC3 => todo!(),
            MBCTypes::MBC5 => todo!(),
            MBCTypes::MBC6 => todo!(),
            MBCTypes::MBC7 => todo!(),
            MBCTypes::TAMA5 => todo!(),
            MBCTypes::HUC1 => todo!(),
            MBCTypes::HUC3 => todo!(),
            MBCTypes::UNKNOWN => todo!(),
        }
        
        
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => 0,
            MBCTypes::MBC1 => todo!(),
            MBCTypes::MBC2 => todo!(),
            MBCTypes::MMM01 => todo!(),
            MBCTypes::MBC3 => todo!(),
            MBCTypes::MBC5 => todo!(),
            MBCTypes::MBC6 => todo!(),
            MBCTypes::MBC7 => todo!(),
            MBCTypes::TAMA5 => todo!(),
            MBCTypes::HUC1 => todo!(),
            MBCTypes::HUC3 => todo!(),
            MBCTypes::UNKNOWN => todo!(),
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => 0,
            MBCTypes::MBC1 => todo!(),
            MBCTypes::MBC2 => todo!(),
            MBCTypes::MMM01 => todo!(),
            MBCTypes::MBC3 => todo!(),
            MBCTypes::MBC5 => todo!(),
            MBCTypes::MBC6 => todo!(),
            MBCTypes::MBC7 => todo!(),
            MBCTypes::TAMA5 => todo!(),
            MBCTypes::HUC1 => todo!(),
            MBCTypes::HUC3 => todo!(),
            MBCTypes::UNKNOWN => todo!(),
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => 0,
            MBCTypes::MBC1 => todo!(),
            MBCTypes::MBC2 => todo!(),
            MBCTypes::MMM01 => todo!(),
            MBCTypes::MBC3 => todo!(),
            MBCTypes::MBC5 => todo!(),
            MBCTypes::MBC6 => todo!(),
            MBCTypes::MBC7 => todo!(),
            MBCTypes::TAMA5 => todo!(),
            MBCTypes::HUC1 => todo!(),
            MBCTypes::HUC3 => todo!(),
            MBCTypes::UNKNOWN => todo!(),
        }
    }
    
}