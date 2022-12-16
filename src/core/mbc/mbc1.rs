use crate::core::cartridge::CartridgeHeader;

use super::Mbc;


pub struct Mbc1 {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
    external_ram: Vec<u8>,
    ram_enabled: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
    banking_mode: u8,
    rom_bank_upper_bits: u8,
    ram_bank_mask: u8,
    rom_bank_mask: u8,
}

impl Mbc1 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        let ram_size = header.ram_size as usize;
        Self {
            rom_data: rom_data,
            header: header,
            external_ram: Vec::with_capacity(ram_size),
            ram_enabled: false,
            current_rom_bank: 0,
            current_ram_bank: 0,
            banking_mode: 0,
            rom_bank_upper_bits: 0,
            ram_bank_mask: 0,
            rom_bank_mask: 0,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_data[address as usize],
            0x4000..=0x7FFF => self.rom_data[address as usize + 0x4000 * (self.current_rom_bank + 1)],
            _ => unreachable!()
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.external_ram[address as usize - 0xA000 * (self.current_ram_bank + 1)]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                if value & 0x0A == 0x0A {
                    self.ram_enabled = true;
                } else {
                    self.ram_enabled = false;
                }
            }
            0x2000..=0x3FFF => {
                if self.banking_mode == 1 {
                    self.rom_bank_mask = (self.rom_bank_upper_bits << 5) | value & 0xF1;
                }
            }
            _ => (),
        };
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        todo!()
    }
}