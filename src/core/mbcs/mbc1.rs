// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::core::mbc::{MbcBase, CartridgeHeader, Mbc};
use rand::prelude::*;

pub struct Mbc1 {
    mbc: MbcBase,
    external_ram: Vec<u8>,
    ram_enabled: bool,
    banking_mode: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
}

impl Mbc1 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        let has_ram = match rom_data[0x147] {
            0x00 => false,
            0x01..=0x03 => true,
            _ => unreachable!(),
        };
        let has_battery = rom_data[0x147] == 0x03;
        let mut external_ram: Vec<u8>;
        if has_ram {
            external_ram = vec![0; header.ram_size as usize];
            let mut rng = rand::thread_rng();
            rng.fill_bytes(&mut external_ram);
        } else {
            external_ram = Vec::new();
        }

        Self {
            mbc: MbcBase {
                header,
                rom_data,
                has_ram,
                has_battery,
                has_rtc: false,
            },
            external_ram,
            ram_enabled: false,
            banking_mode: false,
            current_rom_bank: 1,
            current_ram_bank: 0,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = if address < 0x4000 { 0 } else { self.current_rom_bank };
        self.mbc.rom_data[(bank * 0x4000) | ((address as usize) & 0x3FFF)]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.ram_enabled {
            let bank = if self.banking_mode { self.current_ram_bank } else { 0 };
            self.external_ram[(bank * 0x2000) | ((address & 0x1FFF) as usize)]
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
        if self.ram_enabled {
            let bank = if self.banking_mode { self.current_ram_bank } else { 0 };
            self.external_ram[(bank * 0x2000) | ((address & 0x1FFF) as usize)] = value;
        }
    }
}