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
    dirty: bool,
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
            dirty: false,
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
            0x0000..=0x3FFF => self.rom_data.get(address as usize).copied().unwrap_or(0xFF),
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
                self.ram_enabled = (value & 0x0F) == 0x0A;
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
                    if self.ram[addr] != value {
                        self.ram[addr] = value;
                        self.dirty = true;
                    }
                }
            }
            0xB000..=0xBFFF => {
                let addr = self.ram_bank_b * 0x1000 + (address as usize - 0xB000);
                if addr < self.ram.len() {
                    if self.ram[addr] != value {
                        self.ram[addr] = value;
                        self.dirty = true;
                    }
                }
            }
            _ => {}
        }
    }

    fn get_battery_ram(&self) -> Option<&[u8]> {
        if !self.ram.is_empty() {
            Some(&self.ram)
        } else {
            None
        }
    }

    fn set_battery_ram(&mut self, data: &[u8]) {
        if !self.ram.is_empty() {
            let len = self.ram.len().min(data.len());
            self.ram[..len].copy_from_slice(&data[..len]);
        }
    }
}

impl Drop for Mbc6 {
    fn drop(&mut self) {
        if self.dirty {
            let _ = std::fs::write(&self.save_path, &self.ram);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::MBCTypes;

    fn header() -> CartridgeHeader {
        CartridgeHeader {
            entry: [0; 4],
            title: String::new(),
            logo: [0; 0x30],
            cgb_flag: 0,
            sgb_flag: false,
            mbc_type: MBCTypes::Mbc6,
            rom_size: 0x8000,
            ram_size: 0,
            company: String::new(),
        }
    }

    #[test]
    fn disabling_ram_does_not_flush_save_file() {
        let save_path = unique_save_path("mbc6");
        let _ = std::fs::remove_file(&save_path);

        let mut mbc = Mbc6::new(vec![0; 0x8000], header(), save_path.clone());

        mbc.write_rom(0x0000, 0x0A);
        mbc.write_ram(0xA000, 0x5A);
        mbc.write_rom(0x0000, 0x00);

        assert!(!save_path.exists());

        drop(mbc);

        let persisted = std::fs::read(&save_path).expect("dirty RAM should flush on drop");
        assert_eq!(persisted[0], 0x5A);

        let _ = std::fs::remove_file(save_path);
    }

    fn unique_save_path(prefix: &str) -> std::path::PathBuf {
        let suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("sturdygb-{prefix}-{suffix}.sav"))
    }
}
