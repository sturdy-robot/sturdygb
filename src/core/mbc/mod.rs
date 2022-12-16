use super::cartridge::{CartridgeHeader, MBCTypes};

pub mod romonly;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod mbc5;
pub mod mbc6;
pub mod mbc7;



pub trait Mbc {
    // fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self;
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
}
