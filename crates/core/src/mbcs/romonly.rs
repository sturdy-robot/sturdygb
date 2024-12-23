// SPDX-FileCopyrightText: 2024 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct RomOnly {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
}

impl RomOnly {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        Self { header, rom_data }
    }
}

#[allow(unused_variables)]
impl Mbc for RomOnly {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom_data[address as usize]
    }
}
