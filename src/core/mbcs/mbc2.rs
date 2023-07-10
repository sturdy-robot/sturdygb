// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::core::mbc::{MbcBase, CartridgeHeader, Mbc};

pub struct Mbc2 {
    mbc: MbcBase,
    external_ram: [u8; 0x200],
    ram_enabled: bool,
    banking_mode: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
}

impl Mbc2 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        todo!()
        // let has_ram = header.ram_size > 0;
        // let has_battery = rom_data[0x147] == 0x03;
        // let has_rtc = false;
        
        // Self {
        //     mbc: MbcBase {
        //         rom_data,
        //         has_battery,
        //         header,
        //         has_ram,
        //         has_rtc,
        //     }
        // }
        
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, address:u16) -> u8 {
        let bank = if address < 0x4000 { 0 } else { self.current_rom_bank };
        self.mbc.rom_data[(bank * 0x4000) | ((address as usize) & 0x3FFF)]
    }

    fn read_ram(&self, address: u16) -> u8 {
        0xFF
    }

    
    fn write_rom(&mut self, address: u16, value: u8) {
        
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        
    }

    
}