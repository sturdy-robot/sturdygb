// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::mbcs::get_mbc;
use std::fs;

pub fn load_cartridge(filename: &str) -> Result<(Box<dyn Mbc>, GbMode), String> {
    let rom_data =
        fs::read(filename).map_err(|e| format!("Unable to read ROM '{filename}': {e}"))?;
    let save_path = std::path::PathBuf::from(filename).with_extension("sav");
    match CartridgeHeader::new(&rom_data) {
        Ok(header) => Ok(get_mbc(rom_data, header, save_path)),
        Err(f) => Err(f.to_string()),
    }
}

pub fn load_cartridge_from_bytes(rom_data: Vec<u8>) -> Result<(Box<dyn Mbc>, GbMode), String> {
    // For web/memory loads, we do not have a save file path
    let save_path = std::path::PathBuf::new();
    match CartridgeHeader::new(&rom_data) {
        Ok(header) => Ok(get_mbc(rom_data, header, save_path)),
        Err(f) => Err(f.to_string()),
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
    pub company: String,
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
            let company = get_company_name(rom_data[0x14B], &rom_data[0x144..=0x145]);
            Ok(Self {
                entry: rom_data[0x100..=0x103].try_into().unwrap(),
                logo: rom_data[0x104..=0x133].try_into().unwrap(),
                title: rom_data[0x134..=0x143]
                    .iter()
                    .filter(|&&b| b != 0)
                    .map(|&b| b as char)
                    .collect::<String>()
                    .trim()
                    .to_string(),
                cgb_flag: rom_data[0x143],
                sgb_flag,
                mbc_type,
                rom_size,
                ram_size,
                company,
            })
        } else {
            Err("Cartridge is not a valid GB ROM")
        }
    }
}

fn get_company_name(old_code: u8, new_code: &[u8]) -> String {
    if old_code != 0x33 {
        // Fall back to old ascii hash (approximate values missing)
        return format!("Old Licensee: {old_code:02X}");
    }

    let code_str = std::str::from_utf8(new_code).unwrap_or("00");
    match code_str {
        "01" => "Nintendo".to_string(),
        "08" => "Capcom".to_string(),
        "13" => "Electronic Arts".to_string(),
        "18" => "Hudson Soft".to_string(),
        "19" => "b-ai".to_string(),
        "20" => "kss".to_string(),
        "22" => "pow".to_string(),
        "24" => "PCM Complete".to_string(),
        "25" => "san-x".to_string(),
        "28" => "Kemco Japan".to_string(),
        "29" => "seta".to_string(),
        "30" => "Viacom".to_string(),
        "31" => "Nintendo".to_string(),
        "32" => "Bandai".to_string(),
        "33" => "Ocean/Acclaim".to_string(),
        "34" => "Konami".to_string(),
        "35" => "Hector".to_string(),
        "37" => "Taito".to_string(),
        "38" => "Hudson".to_string(),
        "39" => "Banpresto".to_string(),
        "41" => "Ubi Soft".to_string(),
        "42" => "Atlus".to_string(),
        "44" => "Malibu".to_string(),
        "46" => "angel".to_string(),
        "47" => "Bullet-Proof".to_string(),
        "49" => "irem".to_string(),
        "50" => "Absolute".to_string(),
        "51" => "Acclaim".to_string(),
        "52" => "Activision".to_string(),
        "53" => "American sammy".to_string(),
        "54" => "Konami".to_string(),
        "55" => "Hi tech entertainment".to_string(),
        "56" => "LJN".to_string(),
        "57" => "Matchbox".to_string(),
        "58" => "Mattel".to_string(),
        "59" => "Milton Bradley".to_string(),
        "60" => "Titus".to_string(),
        "61" => "Virgin".to_string(),
        "64" => "LucasArts".to_string(),
        "67" => "Ocean".to_string(),
        "69" => "Electronic Arts".to_string(),
        "70" => "Infogrames".to_string(),
        "71" => "Interplay".to_string(),
        "72" => "Broderbund".to_string(),
        "73" => "sculptured".to_string(),
        "75" => "sci".to_string(),
        "78" => "THQ".to_string(),
        "79" => "Accolade".to_string(),
        "80" => "misawa".to_string(),
        "83" => "lozc".to_string(),
        "86" => "Tokuma Shoten Intermedia".to_string(),
        "87" => "Tsukuda Original".to_string(),
        "91" => "Chunsoft".to_string(),
        "92" => "Video system".to_string(),
        "93" => "Ocean/Acclaim".to_string(),
        "95" => "Varie".to_string(),
        "96" => "Yonezawa/s'pal".to_string(),
        "97" => "Kaneko".to_string(),
        "99" => "Pack in soft".to_string(),
        "A4" => "Konami (Yu-Gi-Oh!)".to_string(),
        _ => format!("Unknown: {code_str}"),
    }
}
