// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
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
    save_path: std::path::PathBuf,
    dirty: bool,
}

impl Mbc2 {
    pub fn new(
        rom_data: Vec<u8>,
        header: CartridgeHeader,
        has_battery: bool,
        has_ram: bool,
        save_path: std::path::PathBuf,
    ) -> Self {
        let mut ram = [0xFF; 0x200];
        if has_ram && has_battery && save_path.exists() {
            if let Ok(data) = std::fs::read(&save_path) {
                if data.len() == 0x200 {
                    ram.copy_from_slice(&data);
                }
            }
        }

        Self {
            rom_data,
            header,
            ram,
            ram_enabled: false,
            rom_bank: 1,
            has_ram,
            has_battery,
            save_path,
            dirty: false,
        }
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_data.get(address as usize).copied().unwrap_or(0xFF),
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
                let value = value & 0x0F;
                if self.ram[ram_addr] != value {
                    self.ram[ram_addr] = value;
                    self.dirty = true;
                }
            }
            _ => {}
        }
    }

    fn get_battery_ram(&self) -> Option<&[u8]> {
        if self.has_battery && self.has_ram {
            Some(&self.ram)
        } else {
            None
        }
    }

    fn set_battery_ram(&mut self, data: &[u8]) {
        if self.has_battery && self.has_ram {
            let len = self.ram.len().min(data.len());
            self.ram[..len].copy_from_slice(&data[..len]);
        }
    }
}

impl Drop for Mbc2 {
    fn drop(&mut self) {
        if self.has_battery && self.dirty {
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
            mbc_type: MBCTypes::Mbc2 {
                ram: true,
                battery: true,
            },
            rom_size: 0x8000,
            ram_size: 0,
            company: String::new(),
        }
    }

    #[test]
    fn disabling_ram_does_not_flush_save_file() {
        let save_path = unique_save_path("mbc2");
        let _ = std::fs::remove_file(&save_path);

        let mut mbc = Mbc2::new(vec![0; 0x8000], header(), true, true, save_path.clone());

        mbc.write_rom(0x0000, 0x0A);
        mbc.write_ram(0xA000, 0x5A);
        mbc.write_rom(0x0000, 0x00);

        assert!(!save_path.exists());

        drop(mbc);

        let persisted = std::fs::read(&save_path).expect("dirty RAM should flush on drop");
        assert_eq!(persisted[0] & 0x0F, 0x0A);

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
