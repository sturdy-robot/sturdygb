use crate::core::cartridge::CartridgeHeader;

use super::Mbc;


pub struct Romonly {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
}

impl Romonly {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Romonly {
        Romonly { rom_data: rom_data, header: header }
    }
}

impl Mbc for Romonly {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom_data[address as usize]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        // Do nothing
    }

    fn read_ram(&self, address: u16) -> u8 {
        0x00
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        // Do nothing
    }
}