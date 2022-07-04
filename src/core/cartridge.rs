use std::fs;
use std::io::Error;
use std::io::prelude::*;

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

#[derive(Clone)]
pub struct CartridgeHeader {
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: String,
    pub licensee_code: u16,
    pub sgb_flag: u8,
    pub rom_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub license_code: u8,
    pub checksum: u8,
    pub global_checksum: u16,
}

#[derive(Clone)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub filename: String,
    pub rom_size: usize,
    pub rom_data: Vec<u8>,
}

// impl CartridgeHeader {
//     pub fn new(rom_data: &Vec<u8>) -> Self {
//
//
//         Self {
//             entry,
//             logo,
//             title,
//             licensee_code,
//             sgb_flag,
//             rom_type,
//             rom_size,
//             ram_size,
//             dest_code,
//             license_code,
//             checksum,
//             global_checksum,
//         }
//     }
// }

impl Cartridge {
    pub fn load_cartridge(filename: String) -> Vec<u8>{
        let rom_data = fs::read(filename)
            .expect("Unable to read file contents!");

        rom_data
    }
}

#[cfg(test)]
mod test {
    use super::{CartridgeHeader, Cartridge};

    #[test]
    fn test_load_cartridge() {

    }
}