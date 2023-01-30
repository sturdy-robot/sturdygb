use rand::prelude::*;

#[allow(unused_variables)]
pub trait Mbc {
    fn read_rom(&self, address: u16) -> u8 {
        0xFF
    }

    fn read_ram(&self, address: u16) -> u8 {
        0xFF
    }

    fn write_rom(&mut self, address: u16, value: u8);

    fn write_ram(&mut self, address: u16, value: u8);
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
    Mbc1,
    Mmm01,
    Mbc2,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
    Unknown,
}

#[derive(PartialEq, Eq)]
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

impl CartridgeHeader {
    pub fn new(rom_data: &Vec<u8>) -> Result<Self, &'static str> {
        if checksum(&rom_data) {
            let mbc_type: MBCTypes = match &rom_data[0x0147] {
                0x00 => MBCTypes::RomOnly,
                0x01..=0x03 => MBCTypes::Mbc1,
                0x05..=0x06 => MBCTypes::Mbc2,
                0x0B..=0x0D => MBCTypes::Mmm01,
                0x0F..=0x13 => MBCTypes::Mbc3,
                0x19..=0x1E => MBCTypes::Mbc5,
                0x20 => MBCTypes::Mbc6,
                0x22 => MBCTypes::Mbc7,
                _ => MBCTypes::Unknown,
            };
            let rom_size: u32 = 32 * (1 << rom_data[0x0148]);
            let ram_size: u32 = match rom_data[0x149] {
                0x00 => 0,
                0x02 => 0x2000,
                0x03 => 0x8000,
                0x04 => 0x200000,
                0x05 => 0x10000,
                _ => 0,
            };
            let sgb_flag = rom_data[0x146] == 0x03;
            Ok(Self {
                entry: rom_data[0x100..=0x103].try_into().unwrap(),
                logo: rom_data[0x104..=0x133].try_into().unwrap(),
                title: rom_data[0x134..=0x143].escape_ascii().to_string(),
                cgb_flag: rom_data[0x143].try_into().unwrap(),
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

#[allow(dead_code)]
pub struct MbcBase {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
    has_ram: bool,
    has_battery: bool,
    has_rtc: bool,
}

pub struct RomOnly {
    mbc: MbcBase,
}

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

impl RomOnly {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        Self {
            mbc: MbcBase {
                header,
                rom_data,
                has_ram: false,
                has_battery: false,
                has_rtc: false,
            },
        }
    }
}

#[allow(unused_variables)]
impl Mbc for RomOnly {
    fn read_rom(&self, address: u16) -> u8 {
        self.mbc.rom_data[address as usize]
    }

    fn read_ram(&self, address: u16) -> u8 {
        0xFF
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        // Do nothing
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        // Do nothing
    }
}

pub struct Mbc1 {
    mbc: MbcBase,
    external_ram: Vec<u8>,
    ram_enabled: bool,
    banking_mode: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
}

impl Mbc1 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        let has_ram = header.ram_size > 0;
        let has_battery = rom_data[0x147] == 0x03;
        let mut external_ram: Vec<u8> = vec![0; header.ram_size as usize];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut external_ram);

        Self {
            mbc: MbcBase {
                header,
                rom_data,
                has_ram,
                has_battery,
                has_rtc: false,
            },
            external_ram,
            ram_enabled: false,
            banking_mode: true,
            current_rom_bank: 0,
            current_ram_bank: 0,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.mbc.rom_data[address as usize]
        } else {
            self.mbc.rom_data[self.current_rom_bank * 0x4000 | (address as usize) & 0x3FFF]
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.ram_enabled {
            self.external_ram[(self.current_ram_bank * 0x2000) | (address & 0x1FFF) as usize]
        } else {
            0
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value == 0x0A;
            }
            0x2000..=0x3FFF => {
                let r: usize = if value & 0x1F == 0 { 1 } else { value as usize };
                self.current_rom_bank = (self.current_rom_bank & 0x60) | r;
            }
            0x4000..=0x5FFF => {
                if self.banking_mode {
                    self.current_rom_bank =
                        self.current_rom_bank & 0x1F | ((value & 3) << 5) as usize;
                } else {
                    self.current_ram_bank = (value & 3) as usize;
                }
            }
            0x6000..=0x7FFF => {
                self.banking_mode = value & 1 == 1;
            }
            _ => (),
        };
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled {
            if !self.banking_mode {
                self.external_ram[(self.current_ram_bank * 0x2000) | (address & 0x1FFF) as usize] =
                    value;
            }
        }
    }
}

pub struct Mbc2 {
    mbc: MbcBase,
}

pub struct Mbc3 {
    mbc: MbcBase,
}

pub struct Mbc5 {
    mbc: MbcBase,
}

pub struct Mbc6 {
    mbc: MbcBase,
}

pub struct Mmm01 {
    mbc: MbcBase,
}

pub struct Mbc7 {
    mbc: MbcBase,
}
