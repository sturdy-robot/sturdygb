// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct Mbc6 {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
    ram: Vec<u8>,
    ram_enabled: bool,
    // MBC6 has two independently switchable ROM banks
    rom_bank_a: usize,
    rom_bank_b: usize,
    // And two independently switchable RAM banks
    ram_bank_a: usize,
    ram_bank_b: usize,
    // Bank mapping registers
    bank_a_mapping: usize, // 0x4000-0x5FFF
    bank_b_mapping: usize, // 0x6000-0x7FFF
    flash_mode: bool,
    flash_command: u8,
    save_path: std::path::PathBuf,
}

impl Mbc6 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader, save_path: std::path::PathBuf) -> Self {
        let mut ram = vec![0; 0x4000]; // 128Kb of RAM
        if save_path.exists() {
            if let Ok(data) = std::fs::read(&save_path) {
                if data.len() == 0x4000 {
                    ram = data;
                }
            }
        }
        Self {
            rom_data,
            header,
            ram,
            ram_enabled: false,
            rom_bank_a: 1,
            rom_bank_b: 2,
            ram_bank_a: 0,
            ram_bank_b: 0,
            bank_a_mapping: 0x4000,
            bank_b_mapping: 0x6000,
            flash_mode: false,
            flash_command: 0,
            save_path,
        }
    }

    fn handle_flash_command(&mut self, address: u16, value: u8) {
        match (address, value) {
            (0x0000, 0xF0) => {
                // Reset flash mode
                self.flash_mode = false;
                self.flash_command = 0;
            }
            (0xAAA, 0xAA) if self.flash_command == 0 => {
                self.flash_command = 0xAA;
            }
            (0x555, 0x55) if self.flash_command == 0xAA => {
                self.flash_command = 0x55;
            }
            (0xAAA, 0xA0) if self.flash_command == 0x55 => {
                // Enable flash write
                self.flash_mode = true;
                self.flash_command = 0;
            }
            _ => {
                self.flash_command = 0;
            }
        }
    }
}

impl Mbc for Mbc6 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_data[address as usize],
            0x4000..=0x5FFF => {
                let bank = self.rom_bank_a;
                let addr = bank * 0x2000 + (address as usize - 0x4000);
                if addr < self.rom_data.len() {
                    self.rom_data[addr]
                } else {
                    0xFF
                }
            }
            0x6000..=0x7FFF => {
                let bank = self.rom_bank_b;
                let addr = bank * 0x2000 + (address as usize - 0x6000);
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
        if self.flash_mode {
            // Handle flash memory writes
            match address {
                0x4000..=0x5FFF => {
                    let addr = self.rom_bank_a * 0x2000 + (address as usize - 0x4000);
                    if addr < self.rom_data.len() {
                        self.rom_data[addr] &= value; // Flash memory can only clear bits
                    }
                }
                0x6000..=0x7FFF => {
                    let addr = self.rom_bank_b * 0x2000 + (address as usize - 0x6000);
                    if addr < self.rom_data.len() {
                        self.rom_data[addr] &= value; // Flash memory can only clear bits
                    }
                }
                _ => {}
            }
            self.flash_mode = false;
            return;
        }

        match address {
            // RAM Enable
            0x0000..=0x1FFF => {
                let was_enabled = self.ram_enabled;
                self.ram_enabled = (value & 0x0F) == 0x0A;
                if was_enabled && !self.ram_enabled {
                    let _ = std::fs::write(&self.save_path, &self.ram);
                }
            }
            // ROM Bank A Number
            0x2000..=0x2FFF => {
                self.rom_bank_a = value as usize;
                if self.rom_bank_a == 0 {
                    self.rom_bank_a = 1;
                }
            }
            // ROM Bank B Number
            0x3000..=0x3FFF => {
                self.rom_bank_b = value as usize;
                if self.rom_bank_b == 0 {
                    self.rom_bank_b = 1;
                }
            }
            // RAM Bank A Number
            0x4000..=0x4FFF => {
                self.ram_bank_a = value as usize & 0x7;
            }
            // RAM Bank B Number
            0x5000..=0x5FFF => {
                self.ram_bank_b = value as usize & 0x7;
            }
            // Flash commands
            _ => {
                self.handle_flash_command(address, value);
            }
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        match address {
            0xA000..=0xAFFF => {
                let addr = self.ram_bank_a * 0x1000 + (address as usize - 0xA000);
                if addr < self.ram.len() {
                    self.ram[addr]
                } else {
                    0xFF
                }
            }
            0xB000..=0xBFFF => {
                let addr = self.ram_bank_b * 0x1000 + (address as usize - 0xB000);
                if addr < self.ram.len() {
                    self.ram[addr]
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        match address {
            0xA000..=0xAFFF => {
                let addr = self.ram_bank_a * 0x1000 + (address as usize - 0xA000);
                if addr < self.ram.len() {
                    self.ram[addr] = value;
                }
            }
            0xB000..=0xBFFF => {
                let addr = self.ram_bank_b * 0x1000 + (address as usize - 0xB000);
                if addr < self.ram.len() {
                    self.ram[addr] = value;
                }
            }
            _ => {}
        }
    }
}

impl Drop for Mbc6 {
    fn drop(&mut self) {
        let _ = std::fs::write(&self.save_path, &self.ram);
    }
}
