use crate::core::cartridge::{Cartridge, MBCTypes};

impl Cartridge {
    pub fn read_rom(&mut self, address: u16) -> u8 {
        match self.header.rom_type {
            MBCTypes::Romonly => self.romonly_read_rom(address),
            MBCTypes::Mbc1 => self.mbc1_read_rom(address),
            MBCTypes::Mbc2 => self.mbc2_read_rom(address),
            MBCTypes::Mmm01 => self.mmm01_read_rom(address),
            MBCTypes::Mbc3 => self.mbc3_read_rom(address),
            MBCTypes::Mbc5 => self.mbc5_read_rom(address),
            MBCTypes::Mbc6 => self.mbc6_read_rom(address),
            MBCTypes::Mbc7 => self.mbc7_read_rom(address),
            MBCTypes::Tama5 => unimplemented!(),
            MBCTypes::Huc1 => unimplemented!(),
            MBCTypes::Huc3 => unimplemented!(),
            MBCTypes::Unknown => 0xFF,
        }
    }

    pub fn read_ram(&mut self, address: u16) -> u8 {
        match self.header.rom_type {
            MBCTypes::Romonly => 0,
            MBCTypes::Mbc1 => self.mbc1_read_ram(address),
            MBCTypes::Mbc2 => self.mbc2_read_ram(address),
            MBCTypes::Mmm01 => self.mmm01_read_ram(address),
            MBCTypes::Mbc3 => self.mbc3_read_ram(address),
            MBCTypes::Mbc5 => self.mbc5_read_ram(address),
            MBCTypes::Mbc6 => self.mbc6_read_ram(address),
            MBCTypes::Mbc7 => self.mbc7_read_ram(address),
            MBCTypes::Tama5 => unimplemented!(),
            MBCTypes::Huc1 => unimplemented!(),
            MBCTypes::Huc3 => unimplemented!(),
            MBCTypes::Unknown => 0xFF,
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        match self.header.rom_type {
            MBCTypes::Romonly => (),
            MBCTypes::Mbc1 => self.mbc1_write_rom(address, value),
            MBCTypes::Mbc2 => self.mbc2_write_rom(address, value),
            MBCTypes::Mmm01 => self.mmm01_write_rom(address, value),
            MBCTypes::Mbc3 => self.mbc3_write_rom(address, value),
            MBCTypes::Mbc5 => self.mbc5_write_rom(address, value),
            MBCTypes::Mbc6 => self.mbc6_write_rom(address, value),
            MBCTypes::Mbc7 => self.mbc7_write_rom(address, value),
            MBCTypes::Tama5 => unimplemented!(),
            MBCTypes::Huc1 => unimplemented!(),
            MBCTypes::Huc3 => unimplemented!(),
            MBCTypes::Unknown => (),
        };
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        match self.header.rom_type {
            MBCTypes::Romonly => (),
            MBCTypes::Mbc1 => self.mbc1_write_ram(address, value),
            MBCTypes::Mbc2 => self.mbc2_write_ram(address, value),
            MBCTypes::Mmm01 => self.mmm01_write_ram(address, value),
            MBCTypes::Mbc3 => self.mbc3_write_ram(address, value),
            MBCTypes::Mbc5 => self.mbc5_write_ram(address, value),
            MBCTypes::Mbc6 => self.mbc6_write_ram(address, value),
            MBCTypes::Mbc7 => self.mbc7_write_ram(address, value),
            MBCTypes::Tama5 => unimplemented!(),
            MBCTypes::Huc1 => unimplemented!(),
            MBCTypes::Huc3 => unimplemented!(),
            MBCTypes::Unknown => (),
        };
    }
}

// Read and Write ROM implementations
impl Cartridge {
    fn romonly_read_rom(&mut self, address: u16) -> u8 {
        self.rom_data[address as usize]
    }

    fn romonly_write_rom(&mut self) {

    }

    fn mbc1_read_rom(&mut self, address: u16) -> u8 {
        match address {
            0 ..=0x3FFF => self.rom_data[address as usize],
            0 ..=0x7FFF => self.rom_data[address as usize], // TODO: Implement switchable bank
            _ => 0xFF,
        }
    }

    fn mbc1_write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x1FFF => {
                if value & 0x0A == 0x0A {
                    self.ram_enabled = true;
                } else {
                    self.ram_enabled = false;
                }
                self.registers.ram_bank_enable = value;
            }
            0x2000..= 0x3FFF => {
                // TODO: FIX THIS
                
            },
            0x4000..= 0x5FFF => {
                if self.banking_mode {
                    // TODO: check this
                    let v = value & 0x03;
                    let new_rom_bank = (v << 5) + (self.current_rom_bank + 1);
                    self.current_rom_bank = new_rom_bank - 1;
                    self.registers.rom_bank_upper_bits = value;
                }
            },
            0x6000..= 0x7FFF => {
                // TODO: implement banking mode select
            }
            _ => (),
        };
    }

    fn mmm01_read_rom(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mmm01_write_rom(&mut self, address: u16, value: u8) {

    }

    fn mbc2_read_rom(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc2_write_rom(&mut self, address: u16, value: u8) {

    }
    
    fn mbc3_read_rom(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc3_write_rom(&mut self, address: u16, value: u8) {
        
    }

    fn mbc5_read_rom(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc5_write_rom(&mut self, address: u16, value: u8) {
        
    }

    fn mbc6_read_rom(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc6_write_rom(&mut self, address: u16, value: u8) {
        
    }

    fn mbc7_read_rom(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc7_write_rom(&mut self, address: u16, value: u8) {
        
    }
}

// Read and write RAM
impl Cartridge {
    fn mbc1_read_ram(&mut self, address: u16) -> u8 {
        if self.ram_enabled {
            self.eram[(address + (self.eram_bank as u16 * 0x2000)  - 0xA000) as usize]
        } else {
            0xFF
        }
    }

    fn mbc1_write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled {
            self.eram[(address + (self.eram_bank as u16 * 0x2000) - 0xA000) as usize] = value;
        }
    }

    fn mmm01_read_ram(&mut self, address: u16) -> u8 {
        0
    }

    fn mmm01_write_ram(&mut self, address: u16, value: u8) {
    
    }

    fn mbc2_read_ram(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc2_write_ram(&mut self, address: u16, value: u8) {
        
    }

    fn mbc3_read_ram(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc3_write_ram(&mut self, address: u16, value: u8) {
        
    }

    fn mbc5_read_ram(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc5_write_ram(&mut self, address: u16, value: u8) {
        
    }

    fn mbc6_read_ram(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc6_write_ram(&mut self, address: u16, value: u8) {

    }

    fn mbc7_read_ram(&mut self, address: u16) -> u8 {
        0xFF
    }

    fn mbc7_write_ram(&mut self, address: u16, value: u8) {
        
    }
}