// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
mod romonly;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod mbc6;
mod mbc7;


use mbc1::Mbc1;
use mbc2::Mbc2;
use romonly::RomOnly;

use super::cartridge::{CartridgeHeader, GbMode, Mbc, MBCTypes};

pub fn get_mbc(rom_data: Vec<u8>, header: CartridgeHeader) -> (Box<dyn Mbc>, GbMode) {
    let gb_mode = match header.cgb_flag {
        0x80 => GbMode::NonCgbMode,
        0xC0 => GbMode::CgbMode,
        _ => GbMode::DmgMode,
    };

    match header.mbc_type {
        MBCTypes::RomOnly => (Box::new(RomOnly::new(rom_data, header)), gb_mode),
        MBCTypes::Mbc1 { ram, battery } => {
            (Box::new(Mbc1::new(rom_data, header, ram, battery)), gb_mode)
        }
        MBCTypes::Mbc2 {battery, ram} => {
            (Box::new(Mbc2::new(rom_data, header, battery, ram)), gb_mode)
        }
        _ => unimplemented!(),
    }
}