// SPDX-FileCopyrightText: 2024 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod mbc6;
mod mbc7;
mod romonly;

use mbc1::Mbc1;
use mbc2::Mbc2;
use mbc3::Mbc3;
use mbc5::Mbc5;
use mbc6::Mbc6;
use mbc7::Mbc7;
use romonly::RomOnly;

use super::cartridge::{CartridgeHeader, GbMode, MBCTypes, Mbc};

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
        MBCTypes::Mbc2 { battery, ram } => {
            (Box::new(Mbc2::new(rom_data, header, battery, ram)), gb_mode)
        }
        MBCTypes::Mbc3 {
            ram,
            timer,
            battery,
        } => (
            Box::new(Mbc3::new(rom_data, header, ram, timer, battery)),
            gb_mode,
        ),
        MBCTypes::Mbc5 {
            ram,
            battery,
            rumble,
        } => (
            Box::new(Mbc5::new(rom_data, header, ram, battery, rumble)),
            gb_mode,
        ),
        MBCTypes::Mbc6 => (Box::new(Mbc6::new(rom_data, header)), gb_mode),
        MBCTypes::Mbc7 => (Box::new(Mbc7::new(rom_data, header)), gb_mode),
        _ => unimplemented!(),
    }
}
