use std::fs;

#[derive(PartialEq, Eq)]
pub enum MBCTypes {
    ROMONLY,
    MBC1,
    MBC2,
    MMM01,
    MBC3,
    MBC5,
    MBC6,
    MBC7,
    TAMA5,
    HUC1,
    HUC3,
    UNKNOWN,
}

fn get_licensee_codes(old_licensee: u8, new_licensee: &[u8]) -> String {
    let mut code: u16;
    
    if old_licensee == 0x33 {
        code = (new_licensee[0] as u16) << 8 | new_licensee[1] as u16;
    } else {
        code = old_licensee as u16;
    }

    let licensee_codes = match code {
        0x01 => "Nintendo",
        0x08 => "Capcom",
        0x09 => "Hot-B",
        0x0A => "Jaleco",
        0x0B => "Coconuts Japan",
        0x0C => "Elite Systems",
        0x13 => "Electronic Arts",
        0x18 => "Hudson Soft",
        0x19 => "b-ai",
        0x20 => "kss",
        0x22 => "pow",
        0x24 => "PCM Complete",
        0x25 => "san-x",
        0x28 => "Kemco Japan",
        0x29 => "seta",
        0x30 => "Viacom",
        0x31 => "Nintendo",
        0x32 => "Bandai",
        0x33 => "Ocean/Acclaim",
        0x34 => "Konami",
        0x35 => "Hector",
        0x37 => "Taito",
        0x38 => "Hudson",
        0x39 => "Banpresto",
        0x41 => "UbiSoft",
        0x42 => "Atlus",
        0x44 => "Malibu",
        0x46 => "angel",
        0x47 => "Bullet-Proof",
        0x49 => "irem",
        0x50 => "Absolute",
        0x51 => "Acclaim",
        0x52 => "Activision",
        0x53 => "American sammy",
        0x54 => "Konami",
        0x55 => "Hi tech entertainment",
        0x56 => "LJN",
        0x57 => "Matchbox",
        0x58 => "Mattel",
        0x59 => "Milton Bradley",
        0x60 => "Titus",
        0x61 => "Virgin",
        0x64 => "LucasArts",
        0x67 => "Ocean",
        0x69 => "Electronic Arts",
        0x70 => "Infogrames",
        0x71 => "Interplay",
        0x72 => "Broderbund",
        0x73 => "sculptured",
        0x75 => "sci",
        0x78 => "THQ",
        0x79 => "Accolade",
        0x80 => "misawa",
        0x83 => "lozc",
        0x86 => "Tokuma Shoten Intermedia",
        0x87 => "Tsukuda Original",
        0x91 => "Chunsoft",
        0x92 => "Video system",
        0x93 => "Ocean/Acclaim",
        0x95 => "Varie",
        0x96 => "Yonezawa/s'pal",
        0x97 => "Kaneko",
        0x99 => "Pack in soft",
        0xA4 => "Konami (Yu-Gi-Oh!)",
        _ => "",
    };
    licensee_codes.to_string()
}


#[derive(PartialEq, Eq)]
pub struct CartridgeHeader {
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: [u8; 16],
    pub cgb_flag: u8,
    pub licensee_code: String,
    pub sgb_flag: u8,
    pub rom_type: MBCTypes,
    pub rom_size: u8,
    pub ram_size: u32,
    pub dest_code: u8,
    pub checksum: u8,
}

impl CartridgeHeader {
    pub fn new(rom_data: &[u8]) -> Self {
        let rom_type: MBCTypes = match &rom_data[0x0147] {
            0x00 => MBCTypes::ROMONLY,
            0x01 ..= 0x03 => MBCTypes::MBC1,
            0x05 ..= 0x06 => MBCTypes::MBC2,
            0x0B ..= 0x0D => MBCTypes::MMM01,
            0x0F ..= 0x13 => MBCTypes::MBC3,
            0x19 ..= 0x1E => MBCTypes::MBC5,
            0x20 => MBCTypes::MBC6,
            0x22 => MBCTypes::MBC7,
            _ => MBCTypes::UNKNOWN,
        };
        let rom_size = 32 * (1 << rom_data[0x0148]);
        let ram_size: u32 = match rom_data[0x149] {
            0x00 => 0,
            0x01 => 2048,
            0x02 => 8192,
            0x03 => 32768,
            0x04 => 131072,
            0x05 => 65536,
            _ => 0,
        };
        Self {
            entry: rom_data[0x100..=0x103].try_into().unwrap(),
            logo: rom_data[0x104..=0x133].try_into().unwrap(),
            title: rom_data[0x134..=0x143].try_into().unwrap(),
            cgb_flag: rom_data[0x143],
            licensee_code: get_licensee_codes(rom_data[0x14B], rom_data[0x144..=0x145].try_into().unwrap()),
            sgb_flag: rom_data[0x146],
            rom_type,
            rom_size,
            ram_size,
            dest_code: rom_data[0x14A],
            checksum: rom_data[0x14D],
        }
    }

}

#[derive(PartialEq, Eq)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub rom_data: Vec<u8>,
    pub ram: Vec<u8>,
}

pub fn load_file(filename: String) -> Vec<u8>{
    let rom_data = fs::read(filename)
        .expect("Unable to read file contents!");

    rom_data
}

impl Cartridge {
    pub fn new(filename: &str) -> Self {
        let rom_data = load_file(filename.to_string());

        Self {
            header: CartridgeHeader::new(&rom_data),
            rom_data,
            ram: Vec::new(),
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

    pub fn read_rom(&self, address: u16) -> u8 {
        self.rom_data[address as usize]
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        todo!()
    }

    pub fn write_rom(&mut self, address: u16, value: u8) -> u8 {
        todo!()
    }

    pub fn write_ram(&mut self, address: u16, value: u8) -> u8 {
        todo!()
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
    use super::{Cartridge, get_logo, load_file};

    fn get_file() -> Vec<u8> {
        let cartridge_data = load_file("roms/cgb-acid2.gbc".to_string());
        cartridge_data
    }

    fn get_file2() -> Vec<u8> {
        let cartridge_data = load_file("roms/dmg-acid2.gb".to_string());
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
        let entry = [rom_data[0x0100], rom_data[0x0101], rom_data[0x0102], rom_data[0x0103]];
        assert_eq!(entry, [0, 195, 80, 1]);
    }

    #[test]
    fn test_entry2() {
        let rom_data = get_file2();
        let entry = [rom_data[0x0100], rom_data[0x0101], rom_data[0x0102], rom_data[0x0103]];
        assert_eq!(entry, [0, 195, 80, 1]);
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
        let mut rom_data = get_file();
        let rom_checksum: u8 = rom_data[0x014D];
        let result = checksum(&mut rom_data, rom_checksum);
        assert_eq!(result, true);
    }

    #[test]
    fn test_checksum2() {
        let mut rom_data = get_file2();
        let rom_checksum: u8 = rom_data[0x014D];
        let result = checksum(&mut rom_data, rom_checksum);
        assert_eq!(result, true);
    }

}
