use std::fs;
use std::io::Error;
use std::io::prelude::*;
use std::iter::FromIterator;


#[derive(Clone, PartialEq)]
pub struct Cartridge {
    pub rom_size: usize,
    pub rom_data: Vec<u8>,
}

pub fn load_cartridge(filename: String) -> Vec<u8>{
    let mut rom_data = fs::read(filename)
        .expect("Unable to read file contents!");

    rom_data
}

impl Cartridge {
    pub fn new(filename: &str) -> Self {
        let mut rom_data = load_cartridge(filename.to_string());
        let rom_size = match rom_data[0x0148] {
            0x00 => 32,
            0x01 => 64,
            0x02 => 128,
            0x03 => 256,
            0x04 => 512,
            0x05 => 1024,
            0x06 => 2048,
            0x07 => 4096,
            0x08 => 8192,
            0x52 => 1100,
            0x53 => 1200,
            0x54 => 1500,
            _ => 0,
        };

        Self {
            rom_data,
            rom_size,
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
}

pub fn get_logo() -> [u8; 48] {
    [
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
        0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
        0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
    ]
}

#[cfg(test)]
mod test {
    use super::{Cartridge, get_logo, load_cartridge};

    fn get_load_cartridge() -> Vec<u8> {
        let mut cartridge_data = load_cartridge("roms/cgb-acid2.gbc".to_string());
        cartridge_data
    }

    fn get_load_cartridge2() -> Vec<u8> {
        let mut cartridge_data = load_cartridge("roms/dmg-acid2.gb".to_string());
        cartridge_data
    }

    fn checksum(rom_data: &mut Vec<u8>, checksum: u8) -> bool {
        let mut x: u8 = 0;
        let mut i: usize = 0x0134;
        while i <= 0x014C {
            x = x.wrapping_sub(rom_data[i]).wrapping_sub(1);
            i += 1;
        }
        x == checksum
    }

    #[test]
    fn test_create_new_cartridge() {
        let mut cartridge = Cartridge::new("roms/cgb-acid2.gbc");
        assert_eq!(cartridge.rom_size, 32);
    }

    #[test]
    fn test_load_cartridge() {
        let mut rom_data = get_load_cartridge();
        assert_ne!(rom_data, Vec::new());
        assert_eq!(rom_data.len(), 32768);
    }

    #[test]
    fn test_entry() {
        let mut rom_data = get_load_cartridge();
        let entry = [rom_data[0x0100], rom_data[0x0101], rom_data[0x0102], rom_data[0x0103]];
        assert_eq!(entry, [0, 195, 80, 1]);
    }

    #[test]
    fn test_entry2() {
        let mut rom_data = get_load_cartridge2();
        let entry = [rom_data[0x0100], rom_data[0x0101], rom_data[0x0102], rom_data[0x0103]];
        assert_eq!(entry, [0, 195, 80, 1]);
    }

    #[test]
    fn test_logo1() {
        let mut logo = get_logo().to_vec();
        let mut rom_data = get_load_cartridge();
        let mut logo_data = &rom_data[0x0104..0x0134];
        assert_eq!(logo, logo_data);
    }

    #[test]
    fn test_logo2() {
        let mut logo = get_logo().to_vec();
        let mut rom_data = get_load_cartridge2();
        let mut logo_data = &rom_data[0x0104..0x0134]; 
        assert_eq!(logo, logo_data);
    }

    #[test]
    fn test_cgb_flag1() {
        let mut rom_data = get_load_cartridge();
        let mut cgb_flag = rom_data[0x0143];
        assert_eq!(cgb_flag, 0xC0);
    }

    #[test]
    fn test_cgb_flag2() {
        let mut rom_data = get_load_cartridge2();
        let mut cgb_flag = rom_data[0x0143];
        assert_eq!(cgb_flag, 0x00);
    }

    #[test]
    fn test_new_licensee_code1() {
        let mut rom_data = get_load_cartridge();
        let new_licensee_code = &rom_data[0x0144..0x0145];
        assert_eq!(new_licensee_code, [0x00]); 
    }

    #[test]
    fn test_new_licensee_code2() {
        let mut rom_data = get_load_cartridge2();
        let new_licensee_code = &rom_data[0x0144..0x0145];
        assert_eq!(new_licensee_code, [0x00]);
    }

    #[test]
    fn test_sgb_flag1() {
        let mut rom_data = get_load_cartridge();
        let sgb_flag = rom_data[0x0146];
        assert_eq!(sgb_flag, 0x00);
    }

    #[test]
    fn test_sgb_flag2() {
        let mut rom_data = get_load_cartridge2();
        let sgb_flag = rom_data[0x0146];
        assert_eq!(sgb_flag, 0x00);
    }

    #[test]
    fn test_cartridge_type1() {
        let mut rom_data = get_load_cartridge();
        let cartridge_type = rom_data[0x0147];
        assert_eq!(cartridge_type, 0x00);
    }


    #[test]
    fn test_cartridge_type2() {
        let mut rom_data = get_load_cartridge2();
        let cartridge_type = rom_data[0x0147];
        assert_eq!(cartridge_type, 0x00);
    }

    #[test]
    fn test_rom_size1() {
        let mut rom_data = get_load_cartridge();
        let rom_size = rom_data[0x0148];
        assert_eq!(rom_size, 0x00);
    }

    #[test]
    fn test_rom_size2() {
        let mut rom_data = get_load_cartridge2();
        let rom_size = rom_data[0x0148];
        assert_eq!(rom_size, 0x00);
    }

    #[test]
    fn test_ram_size1() {
        let mut rom_data = get_load_cartridge();
        let ram_size = rom_data[0x0149];
        assert_eq!(ram_size, 0x00);
    }

    #[test]
    fn test_ram_size2() {
        let mut rom_data = get_load_cartridge2();
        let ram_size = rom_data[0x0149];
        assert_eq!(ram_size, 0x00);
    }

    #[test]
    fn test_checksum1() {
        let mut rom_data = get_load_cartridge();
        let rom_checksum: u8 = rom_data[0x014D];
        let result = checksum(&mut rom_data, rom_checksum);
        assert_eq!(result, true);
    }

    #[test]
    fn test_checksum2() {
        let mut rom_data = get_load_cartridge2();
        let rom_checksum: u8 = rom_data[0x014D];
        let result = checksum(&mut rom_data, rom_checksum);
        assert_eq!(result, true);
    }

}
