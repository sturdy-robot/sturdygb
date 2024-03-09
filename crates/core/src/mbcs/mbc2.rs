// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
use rand::prelude::*;

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct Mbc2 {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
    external_ram: [u8; 0x2000],
    ram_enabled: bool,
    banking_mode: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
}

impl Mbc2 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader, has_ram: bool, has_battery: bool) -> Self {
        let mut external_ram = [0; 0x2000];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut external_ram);
        Self {
            header,
            rom_data,
            external_ram,
            ram_enabled: false,
            banking_mode: false,
            current_rom_bank: 1,
            current_ram_bank: 0,
        }
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = if address < 0x4000 { 0 } else { self.current_rom_bank };
        self.rom_data[(bank * 0x4000) | ((address as usize) & 0x3FFF)]
    }

    fn read_ram(&self, address: u16) -> u8 {
        0xFF
    }


    fn write_rom(&mut self, address: u16, value: u8) {}

    fn write_ram(&mut self, address: u16, value: u8) {}
}