use crate::core::cartridge::CartridgeHeader;
use rand::{RngCore};

use super::Mbc;


pub struct Mbc1 {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
    external_ram: Vec<u8>,
    ram_enabled: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
    banking_mode: bool,
}

impl Mbc1 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        let ram_size = header.ram_size as usize;
        let mut rng = rand::thread_rng();
        let mut external_ram: Vec<u8> = vec![u8::default(); ram_size];
        rng.fill_bytes(&mut external_ram);
        
        Self {
            rom_data: rom_data,
            header: header,
            external_ram: external_ram,
            ram_enabled: false,
            current_rom_bank: 0,
            current_ram_bank: 0,
            banking_mode: true,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_data[address as usize],
            0x4000..=0x7FFF => self.rom_data[self.current_rom_bank * 0x4000 | (address as usize) & 0x3FFF],
            _ => unreachable!()
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.ram_enabled {
            self.external_ram[(self.current_ram_bank * 0x2000) | (address & 0x1FFF) as usize]
        } else {
            0
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value == 0x0A;
            }
            0x2000..=0x3FFF => {
                let r: usize = if value & 0x1F == 0 { 1 } else { value as usize };
                self.current_rom_bank = (self.current_rom_bank & 0x60) | r;
            }
            0x4000..=0x5FFF => {
                if self.banking_mode {
                    self.current_rom_bank = self.current_rom_bank & 0x1F | ((value & 3) << 5) as usize 
                } else {
                    self.current_ram_bank = (value & 3) as usize;
                }
            }
            0x6000..=0x7FFF => {
                self.banking_mode = value & 1 == 1;
            }
            _ => (),
        };
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled {
            if !self.banking_mode {
                self.external_ram[(self.current_ram_bank * 0x2000) | (address & 0x1FFF) as usize] = value;
            }
        }
    }
}