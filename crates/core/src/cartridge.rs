// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::mbcs::get_mbc;
use std::fs;

pub fn load_cartridge(filename: &str) -> Result<(Box<dyn Mbc>, GbMode), &str> {
    let rom_data = fs::read(filename).expect("Unable to read file contents");
    match CartridgeHeader::new(&rom_data) {
        Ok(header) => Ok(get_mbc(rom_data, header)),
        Err(f) => return Err(f),
    }
}

#[allow(unused_variables)]
pub trait Mbc {
    fn read_rom(&self, address: u16) -> u8;

    fn read_ram(&self, address: u16) -> u8 {
        0xFF
    }

    fn write_rom(&mut self, address: u16, value: u8) {}

    fn write_ram(&mut self, address: u16, value: u8) {}
}

pub struct CartridgeHeader {
    pub entry: [u8; 4],
    pub title: String,
    pub logo: [u8; 0x30],
    pub cgb_flag: u8,
    pub sgb_flag: bool,
    pub mbc_type: MBCTypes,
    pub rom_size: u32,
    pub ram_size: u32,
}

pub enum MBCTypes {
    RomOnly,
    Mbc1 {
        ram: bool,
        battery: bool,
    },
    Mmm01 {
        ram: bool,
        battery: bool,
    },
    Mbc2 {
        ram: bool,
        battery: bool,
    },
    Mbc3 {
        ram: bool,
        timer: bool,
        battery: bool,
    },
    Mbc5 {
        ram: bool,
        battery: bool,
        rumble: bool,
    },
    Mbc6,
    Mbc7,
    Unknown,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum GbMode {
    DmgMode,
    NonCgbMode,
    CgbMode,
}

fn checksum(rom_data: &Vec<u8>) -> bool {
    let mut x: u8 = 0;
    for i in 0x0134..=0x014C {
        x = x.wrapping_sub(rom_data[i]).wrapping_sub(1);
    }
    x == rom_data[0x014D]
}

fn get_mbc_type(mbc_type: &u8) -> MBCTypes {
    match mbc_type {
        0x00 => MBCTypes::RomOnly,
        0x01 => MBCTypes::Mbc1 {
            ram: false,
            battery: false,
        },
        0x02 => MBCTypes::Mbc1 {
            ram: true,
            battery: false,
        },
        0x03 => MBCTypes::Mbc1 {
            ram: true,
            battery: true,
        },
        0x05 => MBCTypes::Mbc2 {
            ram: false,
            battery: false,
        },
        0x06 => MBCTypes::Mbc2 {
            ram: false,
            battery: true,
        },
        0x0B => MBCTypes::Mmm01 {
            ram: false,
            battery: false,
        },
        0x0C => MBCTypes::Mmm01 {
            ram: true,
            battery: false,
        },
        0x0D => MBCTypes::Mmm01 {
            ram: true,
            battery: true,
        },
        0x0F => MBCTypes::Mbc3 {
            ram: false,
            timer: true,
            battery: true,
        },
        0x10 => MBCTypes::Mbc3 {
            ram: true,
            timer: true,
            battery: true,
        },
        0x11 => MBCTypes::Mbc3 {
            ram: false,
            timer: false,
            battery: false,
        },
        0x12 => MBCTypes::Mbc3 {
            ram: true,
            timer: false,
            battery: false,
        },
        0x13 => MBCTypes::Mbc3 {
            ram: true,
            timer: false,
            battery: true,
        },
        0x19 => MBCTypes::Mbc5 {
            ram: false,
            battery: false,
            rumble: false,
        },
        0x1A => MBCTypes::Mbc5 {
            ram: true,
            battery: false,
            rumble: false,
        },
        0x1B => MBCTypes::Mbc5 {
            ram: true,
            battery: true,
            rumble: false,
        },
        0x1C => MBCTypes::Mbc5 {
            ram: false,
            battery: false,
            rumble: true,
        },
        0x1D => MBCTypes::Mbc5 {
            ram: true,
            battery: false,
            rumble: true,
        },
        0x1E => MBCTypes::Mbc5 {
            ram: true,
            battery: true,
            rumble: true,
        },
        0x20 => MBCTypes::Mbc6,
        0x22 => MBCTypes::Mbc7,
        _ => MBCTypes::Unknown,
    }
}

fn get_ram_size(ram_size: &u8) -> u32 {
    match ram_size {
        0x00 => 0,
        0x02 => 0x2000,
        0x03 => 0x8000,
        0x04 => 0x200000,
        0x05 => 0x10000,
        _ => 0,
    }
}

impl CartridgeHeader {
    pub fn new(rom_data: &Vec<u8>) -> Result<Self, &'static str> {
        if checksum(rom_data) {
            let mbc_type: MBCTypes = get_mbc_type(&rom_data[0x0147]);
            let rom_size: u32 = 32 * (1 << rom_data[0x0148]);
            let ram_size: u32 = get_ram_size(&rom_data[0x0149]);
            let sgb_flag = rom_data[0x146] == 0x03;
            Ok(Self {
                entry: rom_data[0x100..=0x103].try_into().unwrap(),
                logo: rom_data[0x104..=0x133].try_into().unwrap(),
                title: rom_data[0x134..=0x143].escape_ascii().to_string(),
                cgb_flag: rom_data[0x143],
                sgb_flag,
                mbc_type,
                rom_size,
                ram_size,
            })
        } else {
            Err("Cartridge is not a valid GB ROM")
        }
    }
}
