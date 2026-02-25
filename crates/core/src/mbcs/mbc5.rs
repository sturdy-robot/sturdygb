// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct Mbc3 {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
}

pub struct Mbc5 {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
    ram_enabled: bool,
    rom_bank: usize,
    ram_bank: usize,
    ram: Vec<u8>,
    has_ram: bool,
    has_battery: bool,
    has_rumble: bool,
    rumble_active: bool,
    save_path: std::path::PathBuf,
}

impl Mbc5 {
    pub fn new(
        rom_data: Vec<u8>,
        header: CartridgeHeader,
        ram: bool,
        battery: bool,
        rumble: bool,
        save_path: std::path::PathBuf,
    ) -> Self {
        let ram_size = if ram { header.ram_size as usize } else { 0 };
        let mut external_ram = vec![0; ram_size];
        if ram && battery && save_path.exists() {
            if let Ok(data) = std::fs::read(&save_path) {
                if data.len() == ram_size {
                    external_ram = data;
                }
            }
        }
        Self {
            rom_data,
            header,
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
            ram: external_ram,
            has_ram: ram,
            has_battery: battery,
            has_rumble: rumble,
            rumble_active: false,
            save_path,
        }
    }
}

impl Mbc for Mbc5 {
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
            // RAM Enable
            0x0000..=0x1FFF => {
                let was_enabled = self.ram_enabled;
                self.ram_enabled = (value & 0x0F) == 0x0A;
                if was_enabled && !self.ram_enabled && self.has_battery && !self.ram.is_empty() {
                    let _ = std::fs::write(&self.save_path, &self.ram);
                }
            }
            // ROM Bank Number (Lower 8 bits)
            0x2000..=0x2FFF => {
                let lower = value as usize;
                self.rom_bank = (self.rom_bank & 0x100) | lower;
            }
            // ROM Bank Number (9th bit)
            0x3000..=0x3FFF => {
                let upper = ((value & 0x01) as usize) << 8;
                self.rom_bank = (self.rom_bank & 0xFF) | upper;
                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
            }
            // RAM Bank Number
            0x4000..=0x5FFF => {
                if self.has_rumble {
                    // For rumble cartridges, bit 3 controls the rumble motor
                    self.rumble_active = value & 0x08 != 0;
                    self.ram_bank = (value & 0x07) as usize;
                } else {
                    self.ram_bank = (value & 0x0F) as usize;
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
            0xA000..=0xBFFF => {
                let addr = self.ram_bank * 0x2000 + (address as usize - 0xA000);
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
        if !self.ram_enabled || !self.has_ram {
            return;
        }

        match address {
            0xA000..=0xBFFF => {
                let addr = self.ram_bank * 0x2000 + (address as usize - 0xA000);
                if addr < self.ram.len() {
                    self.ram[addr] = value;
                }
            }
            _ => {}
        }
    }
}

impl Drop for Mbc5 {
    fn drop(&mut self) {
        if self.has_battery && !self.ram.is_empty() {
            let _ = std::fs::write(&self.save_path, &self.ram);
        }
    }
}
