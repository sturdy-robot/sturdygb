// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use rand::prelude::*;

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct Mbc1 {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
    external_ram: Vec<u8>,
    ram_enabled: bool,
    banking_mode: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
    save_path: std::path::PathBuf,
    has_battery: bool,
    dirty: bool,
}

impl Mbc1 {
    pub fn new(
        rom_data: Vec<u8>,
        header: CartridgeHeader,
        has_ram: bool,
        has_battery: bool,
        save_path: std::path::PathBuf,
    ) -> Self {
        let mut external_ram: Vec<u8>;
        if has_ram && header.ram_size > 0 {
            if has_battery && save_path.exists() {
                external_ram =
                    std::fs::read(&save_path).unwrap_or_else(|_| vec![0; header.ram_size as usize]);
                if external_ram.len() != header.ram_size as usize {
                    external_ram.resize(header.ram_size as usize, 0);
                }
            } else {
                external_ram = vec![0; header.ram_size as usize];
                let mut rng = rand::rng();
                rng.fill_bytes(&mut external_ram);
            }
        } else {
            external_ram = Vec::new();
        }

        Self {
            header,
            rom_data,
            external_ram,
            ram_enabled: false,
            banking_mode: false,
            current_rom_bank: 1,
            current_ram_bank: 0,
            save_path,
            has_battery,
            dirty: false,
        }
    }

    fn rom_bank_count(&self) -> usize {
        (self.rom_data.len() / 0x4000).max(1)
    }

    fn ram_bank_count(&self) -> usize {
        if self.external_ram.is_empty() {
            0
        } else {
            (self.external_ram.len() / 0x2000).max(1)
        }
    }

    fn lower_rom_bank(&self) -> usize {
        if self.banking_mode {
            (self.current_rom_bank & 0x60) % self.rom_bank_count()
        } else {
            0
        }
    }

    fn upper_rom_bank(&self) -> usize {
        self.current_rom_bank % self.rom_bank_count()
    }

    fn ram_bank(&self) -> usize {
        if !self.banking_mode || self.external_ram.is_empty() {
            0
        } else {
            self.current_ram_bank % self.ram_bank_count()
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = if address < 0x4000 {
            self.lower_rom_bank()
        } else {
            self.upper_rom_bank()
        };
        let addr = (bank * 0x4000) | ((address as usize) & 0x3FFF);
        self.rom_data.get(addr).copied().unwrap_or(0xFF)
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled || self.external_ram.is_empty() {
            return 0xFF;
        }

        let bank = self.ram_bank();
        let addr = (bank * 0x2000) | ((address & 0x1FFF) as usize);
        self.external_ram.get(addr).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value == 0x0A;
            }
            0x2000..=0x3FFF => {
                let r = match (value as usize) & 0x1F {
                    0 => 1,
                    bank => bank,
                };
                self.current_rom_bank = (self.current_rom_bank & 0x60) | r;
            }
            0x4000..=0x5FFF => {
                if !self.banking_mode {
                    self.current_rom_bank =
                        self.current_rom_bank & 0x1F | (((value as usize) & 0x03) << 5);
                } else {
                    self.current_ram_bank = (value as usize) & 0x03;
                }
            }
            0x6000..=0x7FFF => {
                self.banking_mode = (value & 0x01) == 0x01;
            }
            _ => (),
        };
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled || self.external_ram.is_empty() {
            return;
        }

        let bank = self.ram_bank();
        let addr = (bank * 0x2000) | ((address & 0x1FFF) as usize);
        if let Some(cell) = self.external_ram.get_mut(addr) {
            if *cell != value {
                *cell = value;
                self.dirty = true;
            }
        }
    }

    fn get_battery_ram(&self) -> Option<&[u8]> {
        if self.has_battery && !self.external_ram.is_empty() {
            Some(&self.external_ram)
        } else {
            None
        }
    }

    fn set_battery_ram(&mut self, data: &[u8]) {
        if self.has_battery && !self.external_ram.is_empty() {
            let len = self.external_ram.len().min(data.len());
            self.external_ram[..len].copy_from_slice(&data[..len]);
        }
    }
}

impl Drop for Mbc1 {
    fn drop(&mut self) {
        if self.has_battery && self.dirty && !self.external_ram.is_empty() {
            let _ = std::fs::write(&self.save_path, &self.external_ram);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::MBCTypes;

    fn header(rom_size: usize, ram_size: u32) -> CartridgeHeader {
        CartridgeHeader {
            entry: [0; 4],
            title: String::new(),
            logo: [0; 0x30],
            cgb_flag: 0,
            sgb_flag: false,
            mbc_type: MBCTypes::Mbc1 {
                ram: ram_size > 0,
                battery: false,
            },
            rom_size: rom_size as u32,
            ram_size,
            company: String::new(),
        }
    }

    #[test]
    fn masks_mbc1_lower_rom_bank_bits() {
        let mut rom_data = vec![0; 0x80000];
        for (bank, chunk) in rom_data.chunks_exact_mut(0x4000).enumerate() {
            chunk[0] = bank as u8;
        }

        let mut mbc = Mbc1::new(
            rom_data,
            header(0x80000, 0),
            false,
            false,
            std::path::PathBuf::new(),
        );

        mbc.write_rom(0x2000, 0x42);

        assert_eq!(mbc.read_rom(0x4000), 0x02);
    }

    #[test]
    fn out_of_range_ram_bank_selection_wraps_safely() {
        let mut mbc = Mbc1::new(
            vec![0; 0x8000],
            header(0x8000, 0x2000),
            true,
            true,
            std::path::PathBuf::new(),
        );

        mbc.write_rom(0x0000, 0x0A);
        mbc.write_rom(0x6000, 0x01);
        mbc.write_rom(0x4000, 0x03);
        mbc.write_ram(0xA123, 0x5A);
        mbc.write_rom(0x4000, 0x00);

        assert_eq!(mbc.read_ram(0xA123), 0x5A);
    }

    #[test]
    fn disabling_ram_does_not_flush_save_file() {
        let save_path = unique_save_path("mbc1");
        let _ = std::fs::remove_file(&save_path);

        let mut mbc = Mbc1::new(
            vec![0; 0x8000],
            header(0x8000, 0x2000),
            true,
            true,
            save_path.clone(),
        );

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
