// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct Mbc3 {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
    ram_enabled: bool,
    rom_bank: usize,
    ram_bank: usize,
    ram: Vec<u8>,
    has_ram: bool,
    has_timer: bool,
    has_battery: bool,
    rtc_registers: [u8; 5], // Seconds, Minutes, Hours, Days Low, Days High/Control
    rtc_latch: u8,
    rtc_latched: bool,
}

impl Mbc3 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader, ram: bool, timer: bool, battery: bool) -> Self {
        let ram_size = if ram { header.ram_size as usize } else { 0 };
        Self {
            rom_data,
            header,
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
            ram: vec![0; ram_size],
            has_ram: ram,
            has_timer: timer,
            has_battery: battery,
            rtc_registers: [0; 5],
            rtc_latch: 0xFF,
            rtc_latched: false,
        }
    }
}

impl Mbc for Mbc3 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                // Fast path for ROM bank 0
                self.rom_data.get(address as usize).copied().unwrap_or(0xFF)
            }
            0x4000..=0x7FFF => {
                // Fast path for banked ROM access
                let addr = (self.rom_bank << 14) | (address as usize & 0x3FFF);
                self.rom_data.get(addr).copied().unwrap_or(0xFF)
            }
            _ => 0xFF,
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM Enable
            0x0000..=0x1FFF => {
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            // ROM Bank Number
            0x2000..=0x3FFF => {
                // Fast path for ROM bank switching
                let bank = if value & 0x7F == 0 { 1 } else { value & 0x7F };
                let max_bank = (self.rom_data.len() >> 14).saturating_sub(1);
                self.rom_bank = (bank as usize).min(max_bank);
            }
            // RAM Bank Number or RTC Register Select
            0x4000..=0x5FFF => {
                if value <= 0x03 || (self.has_timer && (0x08..=0x0C).contains(&value)) {
                    self.ram_bank = value as usize;
                }
            }
            // Latch Clock Data
            0x6000..=0x7FFF => {
                if self.has_timer && self.rtc_latch == 0 && value == 1 {
                    self.rtc_latched = !self.rtc_latched;
                    if !self.rtc_latched {
                        // Simplified RTC update - just increment seconds for now
                        self.rtc_registers[0] = self.rtc_registers[0].wrapping_add(1);
                    }
                }
                self.rtc_latch = value;
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        if address >= 0xA000 && address <= 0xBFFF {
            if self.has_timer && self.ram_bank >= 0x08 {
                // Fast path for RTC register read
                return self.rtc_registers[self.ram_bank - 0x08];
            }
            if self.has_ram && self.ram_bank <= 0x03 {
                // Fast path for RAM read
                let addr = (self.ram_bank << 13) | (address as usize & 0x1FFF);
                return self.ram.get(addr).copied().unwrap_or(0xFF);
            }
        }
        0xFF
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        if address >= 0xA000 && address <= 0xBFFF {
            if self.has_timer && self.ram_bank >= 0x08 {
                // Fast path for RTC register write
                self.rtc_registers[self.ram_bank - 0x08] = value;
                return;
            }
            if self.has_ram && self.ram_bank <= 0x03 {
                // Fast path for RAM write
                let addr = (self.ram_bank << 13) | (address as usize & 0x1FFF);
                if let Some(ram_cell) = self.ram.get_mut(addr) {
                    *ram_cell = value;
                }
            }
        }
    }
}
