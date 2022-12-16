use super::mbc::{self, Mbc};
use std::fs;

#[derive(PartialEq, Eq)]
pub enum MBCTypes {
    Romonly,
    Mbc1,
    Mbc2,
    Mmm01,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
    Tama5,
    Huc1,
    Huc3,
    Unknown,
}

pub struct CartridgeHeader {
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: String,
    pub cgb_flag: u8,
    pub sgb_flag: u8,
    pub rom_type: MBCTypes,
    pub rom_size: u8,
    pub ram_size: u32,
    pub dest_code: u8,
    pub checksum: u8,
}

impl CartridgeHeader {
    pub fn new(rom_data: &Vec<u8>) -> Result<Self, &'static str> {
        let rom_type: MBCTypes = match &rom_data[0x0147] {
            0x00 => MBCTypes::Romonly,
            0x01..=0x03 => MBCTypes::Mbc1,
            0x05..=0x06 => MBCTypes::Mbc2,
            0x0B..=0x0D => MBCTypes::Mmm01,
            0x0F..=0x13 => MBCTypes::Mbc3,
            0x19..=0x1E => MBCTypes::Mbc5,
            0x20 => MBCTypes::Mbc6,
            0x22 => MBCTypes::Mbc7,
            _ => MBCTypes::Unknown,
        };
        let rom_size = 32 * (1 << rom_data[0x0148]);
        let ram_size: u32 = match rom_data[0x149] {
            0x00 => 0,
            0x02 => 0x2000,
            0x03 => 0x8000,
            0x04 => 0x200000,
            0x05 => 0x10000,
            _ => 0,
        };
        match checksum(rom_data) {
            true => Ok(Self {
                entry: rom_data[0x100..=0x103].try_into().unwrap(),
                logo: rom_data[0x104..=0x133].try_into().unwrap(),
                title: rom_data[0x134..=0x143].escape_ascii().to_string(),
                cgb_flag: rom_data[0x143],
                sgb_flag: rom_data[0x146],
                rom_type,
                rom_size,
                ram_size,
                dest_code: rom_data[0x14A],
                checksum: rom_data[0x14D],
            }),
            false => Err("Rom is not valid or corrupted!"),
        }
    }
}

pub fn get_mbc(rom_data: Vec<u8>, header: CartridgeHeader) -> Box<dyn Mbc> {
    match header.rom_type {
        MBCTypes::Romonly => Box::new(mbc::romonly::Romonly::new(rom_data, header)),
        MBCTypes::Mbc1 => Box::new(mbc::mbc1::Mbc1::new(rom_data, header)),
        MBCTypes::Mbc2 => Box::new(mbc::mbc2::Mbc2::new(rom_data, header)),
        MBCTypes::Mmm01 => unimplemented!(),
        _ => unimplemented!(),
    }
}

fn checksum(rom_data: &Vec<u8>) -> bool {
    let mut x: u8 = 0;
    let mut i: usize = 0x0134;
    while i <= 0x014C {
        x = x.wrapping_sub(rom_data[i]).wrapping_sub(1);
        i += 1;
    }
    x == rom_data[0x014D]
}

fn get_logo() -> [u8; 48] {
    [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ]
}

fn get_rom_banks(value: u8) -> u16 {
    match value {
        0x00 => 0,
        0x01 => 4,
        0x02 => 8,
        0x03 => 15,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x08 => 512,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => 0,
    }
}

fn get_ram_banks(value: u8) -> u8 {
    match value {
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => 0,
    }
}


pub fn load_cartridge(filename: &str) -> Result<Box<dyn Mbc>, &str> {
    let rom_data: Vec<u8> = fs::read(filename).expect("Unable to read file contents!");
    match CartridgeHeader::new(&rom_data) {
        Ok(h) => Ok(get_mbc(rom_data, h)),
        Err(f) => return Err(f)
    }
}
