// SPDX-FileCopyrightText: 2024 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct Mbc2 {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
    // MBC2 has built-in 512×4 bits RAM
    ram: [u8; 0x200],
    ram_enabled: bool,
    rom_bank: usize,
    has_ram: bool,
    has_battery: bool,
}

impl Mbc2 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader, has_battery: bool, has_ram: bool) -> Self {
        Self {
            rom_data,
            header,
            ram: [0xFF; 0x200],
            ram_enabled: false,
            rom_bank: 1,
            has_ram,
            has_battery,
        }
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_data[address as usize],
            0x4000..=0x7FFF => {
                let bank = self.rom_bank;
                let addr = bank * 0x4000 + (address as usize - 0x4000);
                if addr < self.rom_data.len() {
                    self.rom_data[addr]
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM Enable and ROM Bank Number are controlled by bit 8 of the address
            0x0000..=0x3FFF => {
                let is_ram_enable = (address & 0x0100) == 0;
                if is_ram_enable {
                    // RAM Enable (bit 8 = 0)
                    self.ram_enabled = (value & 0x0F) == 0x0A;
                } else {
                    // ROM Bank Number (bit 8 = 1)
                    let mut bank = value & 0x0F;
                    if bank == 0 {
                        bank = 1;
                    }
                    self.rom_bank = bank as usize;
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled || !self.has_ram {
            return 0xFF;
        }

        match address {
            0xA000..=0xA1FF => {
                // MBC2 RAM is only 512×4 bits, only the lower 4 bits are valid
                // Upper 4 bits are always 1
                let ram_addr = address as usize & 0x1FF;
                0xF0 | (self.ram[ram_addr] & 0x0F)
            }
            _ => 0xFF,
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled || !self.has_ram {
            return;
        }

        match address {
            0xA000..=0xA1FF => {
                // MBC2 RAM is only 512×4 bits, only store the lower 4 bits
                let ram_addr = address as usize & 0x1FF;
                self.ram[ram_addr] = value & 0x0F;
            }
            _ => {}
        }
    }
}
