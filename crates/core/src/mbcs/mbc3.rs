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
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            // ROM Bank Number
            0x2000..=0x3FFF => {
                let mut bank = value & 0x7F;
                if bank == 0 {
                    bank = 1;
                }
                self.rom_bank = bank as usize;
            }
            // RAM Bank Number or RTC Register Select
            0x4000..=0x5FFF => {
                if value <= 0x03 {
                    self.ram_bank = value as usize;
                } else if self.has_timer && (0x08..=0x0C).contains(&value) {
                    // RTC Register select
                    self.ram_bank = value as usize;
                }
            }
            // Latch Clock Data
            0x6000..=0x7FFF => {
                if self.has_timer {
                    if self.rtc_latch == 0 && value == 1 {
                        self.rtc_latched = !self.rtc_latched;
                        // TODO: If unlatching, copy current time to rtc_registers
                    }
                    self.rtc_latch = value;
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        match address {
            0xA000..=0xBFFF => {
                if self.has_timer && self.ram_bank >= 0x08 {
                    // RTC Register read
                    let rtc_reg = self.ram_bank - 0x08;
                    self.rtc_registers[rtc_reg]
                } else if self.has_ram && self.ram_bank <= 0x03 {
                    let addr = self.ram_bank * 0x2000 + (address as usize - 0xA000);
                    if addr < self.ram.len() {
                        self.ram[addr]
                    } else {
                        0xFF
                    }
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
            0xA000..=0xBFFF => {
                if self.has_timer && self.ram_bank >= 0x08 {
                    // RTC Register write
                    let rtc_reg = self.ram_bank - 0x08;
                    self.rtc_registers[rtc_reg] = value;
                } else if self.has_ram && self.ram_bank <= 0x03 {
                    let addr = self.ram_bank * 0x2000 + (address as usize - 0xA000);
                    if addr < self.ram.len() {
                        self.ram[addr] = value;
                    }
                }
            }
            _ => {}
        }
    }
}
