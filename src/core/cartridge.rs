use std::fs;
use std::io::Error;
use std::io::prelude::*;
use std::iter::FromIterator;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
pub enum RomTypes {
    ROMONLY = 0x00,
    MBC1 = 0x01,
    MBC1RAM = 0x02,
    MBC1RAMBATTERY = 0x03,
    MBC2 = 0x05,
    MBC2BATTERY = 0x06,
    ROMRAM = 0x08,
    ROMRAMBATTERY = 0x09,
    MM01 = 0x0B,
    MM01RAM = 0x0C,
    MM01RAMBATTERY = 0x0D,
    MBC3TIMERBATTERY = 0x0F,
    MBC3TIMERRAMBATTERY = 0x10,
    MBC3 = 0x11,
    MBC3RAM = 0x12,
    MBC3RAMBATTERY = 0x13,
    MBC5 = 0x19,
    MBC5RAM = 0x1A,
    MBC5RAMBATTERY = 0x1B,
    MBC5RUMBLE = 0x1C,
    MBC5RUMBLERAM = 0x1D,
    MBC5RUMBLERAMBATTERY = 0x1E,
    MBC6 = 0x20,
    MBC7SENSORRUMBLERAMBATTERY = 0x22,
    POCKETCAMERA = 0xFC,
    BANDAITAMA5 = 0xFD,
    HUC3 = 0xFE,
    HUC1RAMBATTERY = 0xFF,
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[derive(Debug, PartialEq)]
pub enum NewLicenseeCodes {
    None = 0x00,
    NintendoRD1 = 0x01,
    Capcom = 0x08,
    ElectronicArts = 0x13,
    HudsonSoft = 0x18,
    Bai = 0x19,
    Kss = 0x20,
    Pow = 0x22,
    PCMComplete = 0x24,
    SanX = 0x25,
    KemcoJapan = 0x28,
    Seta = 0x29,
    Viacom = 0x30,
    Nintendo = 0x31,
    Bandai = 0x32,
    OceanAcclaim = 0x33,
    Konami = 0x34,
    Hector = 0x35,
    Taito = 0x37,
    Hudson = 0x38,
    Banpresto = 0x39,
    UbiSoft = 0x41,
    Atlus = 0x42,
    Malibu = 0x44,
    Angel = 0x46,
    BulletProof = 0x47,
    Irem = 0x49,
    Absolute = 0x50,
    Acclaim = 0x51,
    Activision = 0x52,
    AmericanSammy = 0x53,
    KonamiJ = 0x54,
    HiTechEntartainment = 0x55,
    LJN = 0x56,
    Matchbox = 0x57,
    Mattel = 0x58,
    MiltonBradley = 0x59,
    Titus = 0x60,
    Virgin = 0x61,
    LucasArts = 0x64,
    Ocean = 0x67,
    ElectronicArtsJ = 0x69,
    Infogrames = 0x70,
    Interplay = 0x71,
    Broderbund = 0x72,
    Sculptured = 0x73,
    Sci = 0x75,
    THQ = 0x78,
    Accolade = 0x79,
    Misawa = 0x80,
    Lozc = 0x83,
    TokumaShotenIntermedia = 0x86,
    TsukudaOriginal = 0x87,
    Chunsoft = 0x91,
    VideoSystem = 0x92,
    OceanAcclaimJ = 0x93,
    Varie = 0x95,
    YonezawaPal = 0x96,
    Kaneko = 0x97,
    PackInSoft = 0x99,
    KonamiYuGiOh = 0xA4,
}


#[derive(Clone, PartialEq)]
pub struct CartridgeHeader {
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: [u8; 16],
    pub manufacturer_code: u8,
    pub cgb_flag: u8,
    pub rom_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub sgb_flag: u8,
    pub dest_code: u8,
    pub new_licensee_code: u8,
    pub old_licensee_code: u8,
    pub version_number: u8,
    pub checksum: u8,
    pub global_checksum: u16,
}

#[derive(Clone, PartialEq)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub filename: String,
    pub rom_size: usize,
    pub rom_data: Vec<u8>,
}

// impl CartridgeHeader {
//     pub fn new(rom_data: &mut Vec<u8>) -> Self {
//         let entry= rom_data[0x0100..0x0104];
//         let logo = rom_data[0x0104..0x0134];
//         let title = rom_data[0x0134..0x0144];
//         let manufacturer_code = rom_data[0x013F..0x0143];
//         let cgb_flag = rom_data[0x0143];
//         let rom_type = rom_data[0x0147];
//         let new_licensee_code = rom_data[0x0144..0x0145];
//         let sgb_flag = rom_data[0x0146];
//         let rom_size = rom_data[0x0148];
//         let ram_size = rom_data[0x0149];
//         let dest_code = rom_data[0x014A];
//         let old_licensee_code = rom_data[0x014B];
//         let version_number = rom_data[0x014C];
//         let checksum = rom_data[0x14D];
//         let global_checksum = rom_data[0x014E..0x14F];
//
//         Self {
//             entry,
//             logo,
//             title,
//             manufacturer_code,
//             cgb_flag,
//             rom_type,
//             rom_size,
//             ram_size,
//             sgb_flag,
//             dest_code,
//             new_licensee_code,
//             old_licensee_code,
//             version_number,
//             checksum,
//             global_checksum,
//         }
//     }
// }

impl Cartridge {
    pub fn load_cartridge(filename: String) -> Vec<u8>{
        let mut rom_data = fs::read(filename)
            .expect("Unable to read file contents!");

        rom_data
    }

    pub fn checksum(&self) -> bool {
        let mut x: u8 = 0;
        let mut i: usize = 0x0134;
        while i <= 0x014C {
            x = x.wrapping_sub(self.rom_data[i]).wrapping_sub(1);
            i += 1;
        }
        x == self.header.checksum
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
    use vulkano::buffer::BufferContents;
    use super::{Cartridge, CartridgeHeader, get_logo, NewLicenseeCodes};

    fn get_load_cartridge() -> Vec<u8> {
        let mut cartridge_data = Cartridge::load_cartridge("roms/cgb-acid2.gbc".to_string());
        cartridge_data
    }

    fn get_load_cartridge2() -> Vec<u8> {
        let mut cartridge_data = Cartridge::load_cartridge("roms/dmg-acid2.gb".to_string());
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
    fn test_load_cartridge() {
        let mut rom_data = get_load_cartridge();
        assert_ne!(rom_data, Vec::new());
        assert_eq!(rom_data.len(), 32768);
    }

    // #[test]
    // fn test_new_cartridge_header() {
    //     let mut rom_data = get_load_cartridge();
    //     let mut cartridge_header = CartridgeHeader::new(&mut rom_data);
    //     assert_eq!(cartridge_header, 0);
    // }

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
