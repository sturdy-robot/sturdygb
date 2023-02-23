mod romonly;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod mbc6;
mod mbc7;

use super::mbc::{ Mbc, CartridgeHeader, MBCTypes, GbMode };
use romonly::RomOnly;
use mbc1::Mbc1;


pub fn get_mbc(rom_data: Vec<u8>, header: CartridgeHeader) -> (Box<dyn Mbc>, GbMode) {
    let gb_mode = match header.cgb_flag {
        0x80 => GbMode::NonCgbMode,
        0xC0 => GbMode::CgbMode,
        _ => GbMode::DmgMode,
    };

    match header.mbc_type {
        MBCTypes::RomOnly => (Box::new(RomOnly::new(rom_data, header)), gb_mode),
        MBCTypes::Mbc1 => (Box::new(Mbc1::new(rom_data, header)), gb_mode),
        _ => unimplemented!(),
    }
}