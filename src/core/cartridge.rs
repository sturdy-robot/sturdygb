use std::fs;

use super::mbc;

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

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub struct CartridgeRegisters {
    pub rom_bank_lower_bits: u8,
    pub rom_bank_upper_bits: u8,
    pub ram_bank_enable: u8,
    pub bank_mode: u8,
    pub is_rtc: bool,
}

fn get_rtc(rom_data: u8) -> bool {
    return rom_data == 0x0F || rom_data == 0x10
}

impl CartridgeRegisters {
    pub fn new(is_rtc: bool) -> Self {
        Self {
            rom_bank_lower_bits: 0,
            rom_bank_upper_bits: 0,
            ram_bank_enable: 0,
            bank_mode: 0,
            is_rtc,
        }
    }
}

impl CartridgeHeader {
    pub fn new(rom_data: &[u8]) -> Self {
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
        Self {
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
        }
    }
}

fn get_eram(header: &CartridgeHeader) -> Vec<u8> {
    let eram_size: usize = header.ram_size as usize;
    let eram: Vec<u8>;
    if eram_size != 0 {
        eram = Vec::with_capacity(eram_size);
    } else {
        eram = Vec::with_capacity(0);
    }

    eram
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
    match  value {
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => 0,
    }
}

#[derive(PartialEq, Eq)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub rom_data: Vec<u8>,
    pub registers: CartridgeRegisters,
    pub has_ram: bool,
    pub rom_banks: u16,
    pub ram_banks: u8,
    pub current_rom_bank: u8,
    pub eram: Vec<u8>,
    pub eram_bank: u8,
    pub ram_enabled: bool,
    pub banking_mode: bool,
}

pub fn load_file(filename: String) -> Vec<u8> {
    // TODO: check if file is .gb, .gbc, or even try to open ZIP files if they're identified
    // Even if there's no extension, we need to check if this is a compatible file
    fs::read(filename).expect("Unable to read file contents!")
}

impl Cartridge {
    pub fn new(filename: &str) -> Self {
        let rom_data = load_file(filename.to_string());

        let ram = rom_data[0x147];
        let has_ram = match ram{
            0x02 | 0x03 | 0x08 | 0x09 | 0x0C | 0x0D | 0x10 | 0x12 | 0x13 | 0x1A | 0x1B | 0x1D | 0x1E | 0x22 | 0xFF => true,
            _ => false,
        };
        let header = CartridgeHeader::new(&rom_data);
        let rom_banks: u16 = get_rom_banks(rom_data[0x148]);
        let ram_banks = get_ram_banks(rom_data[0x149]);
        let eram = get_eram(&header);
        let is_rtc = get_rtc(rom_data[0x147]);

        Self {
            header, 
            rom_data,
            registers: CartridgeRegisters::new(is_rtc),
            has_ram,
            rom_banks,
            ram_banks,
            current_rom_bank: 0,
            eram,
            eram_bank: 0,
            ram_enabled: false,
            banking_mode: true,
        }
    }

    pub fn checksum(&self) -> bool {
        let mut x: u8 = 0;
        let mut i: usize = 0x0134;
        while i <= 0x014C {
            x = x.wrapping_sub(self.rom_data[i]).wrapping_sub(1);
            i += 1;
        }
        x == self.rom_data[0x014D]
    }

    pub fn is_cgb_only(&self) -> bool {
        match self.header.cgb_flag {
            0x80 => false,
            0xC0 => true,
            _ => false,
        }
    }
}

pub fn get_logo() -> [u8; 48] {
    [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00,
        0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
        0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB,
        0xB9, 0x33, 0x3E,
    ]
}

#[cfg(test)]
mod test {
    use super::{get_logo, load_file, Cartridge};

    // TODO: clean up tests and test actual CartridgeHeader and Cartridge structs

    fn get_file() -> Vec<u8> {
        let cartridge_data = load_file("roms/cgb-acid2.gbc".to_string());
        cartridge_data
    }

    fn get_file2() -> Vec<u8> {
        let cartridge_data = load_file("roms/dmg-acid2.gb".to_string());
        cartridge_data
    }

    fn get_cartridge() -> Cartridge {
        Cartridge::new("roms/cgb-acid2.gbc")
    }

    fn get_cartridge2() -> Cartridge {
        Cartridge::new("roms/dmg-acid2.gb")
    }

    #[test]
    fn test_create_new_cartridge() {
        let cartridge = Cartridge::new("roms/cgb-acid2.gbc");
        assert_eq!(cartridge.header.rom_size, 32);
    }

    #[test]
    fn test_load_cartridge() {
        let rom_data = get_file();
        assert_ne!(rom_data, Vec::new());
        assert_eq!(rom_data.len(), 32768);
    }

    #[test]
    fn test_entry() {
        let rom_data = get_file();
        let entry = [
            rom_data[0x0100],
            rom_data[0x0101],
            rom_data[0x0102],
            rom_data[0x0103],
        ];
        assert_eq!(entry, [0, 0xC3, 0x50, 0x01]);
    }

    #[test]
    fn test_entry2() {
        let rom_data = get_file2();
        let entry = [
            rom_data[0x0100],
            rom_data[0x0101],
            rom_data[0x0102],
            rom_data[0x0103],
        ];
        assert_eq!(entry, [0, 0xC3, 0x50, 0x01]);
    }

    #[test]
    fn test_logo1() {
        let logo = get_logo().to_vec();
        let rom_data = get_file();
        let logo_data = &rom_data[0x0104..0x0134];
        assert_eq!(logo, logo_data);
    }

    #[test]
    fn test_logo2() {
        let logo = get_logo().to_vec();
        let rom_data = get_file2();
        let logo_data = &rom_data[0x0104..0x0134];
        assert_eq!(logo, logo_data);
    }

    #[test]
    fn test_title1() {
        let rom_data = get_file();
        let title = rom_data[0x134..=0x143].escape_ascii().to_string();
        // TODO: clean up string
        let title_string = "CGB-ACID2\\x00\\x00\\x00\\x00\\x00\\x00\\xc0".to_string();
        assert_eq!(title, title_string);
    }

    #[test]
    fn test_cgb_flag1() {
        let rom_data = get_file();
        let cgb_flag = rom_data[0x0143];
        assert_eq!(cgb_flag, 0xC0);
    }

    #[test]
    fn test_cgb_flag2() {
        let rom_data = get_file2();
        let cgb_flag = rom_data[0x0143];
        assert_eq!(cgb_flag, 0x00);
    }

    #[test]
    fn test_new_licensee_code1() {
        let rom_data = get_file();
        let new_licensee_code = &rom_data[0x0144..0x0145];
        assert_eq!(new_licensee_code, [0x00]);
    }

    #[test]
    fn test_new_licensee_code2() {
        let rom_data = get_file2();
        let new_licensee_code = &rom_data[0x0144..0x0145];
        assert_eq!(new_licensee_code, [0x00]);
    }

    #[test]
    fn test_sgb_flag1() {
        let rom_data = get_file();
        let sgb_flag = rom_data[0x0146];
        assert_eq!(sgb_flag, 0x00);
    }

    #[test]
    fn test_sgb_flag2() {
        let rom_data = get_file2();
        let sgb_flag = rom_data[0x0146];
        assert_eq!(sgb_flag, 0x00);
    }

    #[test]
    fn test_cartridge_type1() {
        let rom_data = get_file();
        let cartridge_type = rom_data[0x0147];
        assert_eq!(cartridge_type, 0x00);
    }

    #[test]
    fn test_cartridge_type2() {
        let rom_data = get_file2();
        let cartridge_type = rom_data[0x0147];
        assert_eq!(cartridge_type, 0x00);
    }

    #[test]
    fn test_rom_size1() {
        let rom_data = get_file();
        let rom_size = rom_data[0x0148];
        assert_eq!(rom_size, 0x00);
    }

    #[test]
    fn test_rom_size2() {
        let rom_data = get_file2();
        let rom_size = rom_data[0x0148];
        assert_eq!(rom_size, 0x00);
    }

    #[test]
    fn test_ram_size1() {
        let rom_data = get_file();
        let ram_size = rom_data[0x0149];
        assert_eq!(ram_size, 0x00);
    }

    #[test]
    fn test_ram_size2() {
        let rom_data = get_file2();
        let ram_size = rom_data[0x0149];
        assert_eq!(ram_size, 0x00);
    }

    #[test]
    fn test_checksum1() {
        let cartridge = get_cartridge();
        let result = cartridge.checksum();
        assert_eq!(result, true);
    }

    #[test]
    fn test_checksum2() {
        let cartridge = get_cartridge2();
        let result = cartridge.checksum();
        assert_eq!(result, true);
    }
}
