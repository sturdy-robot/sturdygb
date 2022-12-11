use crate::core::cartridge::{Cartridge, MBCTypes, CartridgeRegisters};

impl Cartridge {
    pub fn read_rom(&mut self, address: u16) -> u8 {
        match self.header.rom_type {
            MBCTypes::Romonly => self.romonly_read_rom(address),
            MBCTypes::Mbc1 => self.mbc1_read_rom(address),
            // TODO: IMPLEMENT THESE
            MBCTypes::Mbc2 => 0xFF,
            MBCTypes::Mm01 => 0xFF,
            MBCTypes::Mbc3 => 0xFF,
            MBCTypes::Mbc5 => 0xFF,
            MBCTypes::Mbc6 => 0xFF,
            MBCTypes::Mbc7 => 0xFF,
            MBCTypes::Tama5 => unimplemented!(),
            MBCTypes::Huc1 => unimplemented!(),
            MBCTypes::Huc3 => unimplemented!(),
            MBCTypes::Unknown => 0xFF,
        }
    }

    pub fn read_ram(&mut self, address: u16) -> u8 {
        match self.header.rom_type {
            MBCTypes::Romonly => 0,
            // TODO: IMPLEMENT THESE
            MBCTypes::Mbc1 => self.mbc1_read_ram(address),
            MBCTypes::Mbc2 => 0xFF,
            MBCTypes::Mm01 => 0xFF,
            MBCTypes::Mbc3 => 0xFF,
            MBCTypes::Mbc5 => 0xFF,
            MBCTypes::Mbc6 => 0xFF,
            MBCTypes::Mbc7 => 0xFF,
            MBCTypes::Tama5 => unimplemented!(),
            MBCTypes::Huc1 => unimplemented!(),
            MBCTypes::Huc3 => unimplemented!(),
            MBCTypes::Unknown => 0xFF,
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        match self.header.rom_type {
            MBCTypes::Romonly => (),
            // TODO: IMPLEMENT THESE
            MBCTypes::Mbc1 => self.mbc1_write_rom(address, value),
            MBCTypes::Mbc2 => (),
            MBCTypes::Mm01 => (),
            MBCTypes::Mbc3 => (),
            MBCTypes::Mbc5 => (),
            MBCTypes::Mbc6 => (),
            MBCTypes::Mbc7 => (),
            MBCTypes::Tama5 => unimplemented!(),
            MBCTypes::Huc1 => unimplemented!(),
            MBCTypes::Huc3 => unimplemented!(),
            MBCTypes::Unknown => (),
        };
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        match self.header.rom_type {
            MBCTypes::Romonly => (),
            // TODO: IMPLEMENT THESE
            MBCTypes::Mbc1 => self.mbc1_write_ram(address, value),
            MBCTypes::Mbc2 => self.mbc2_write_ram(),
            MBCTypes::Mm01 => (),
            MBCTypes::Mbc3 => self.mbc3_write_ram(),
            MBCTypes::Mbc5 => self.mbc5_write_ram(),
            MBCTypes::Mbc6 => self.mbc6_write_ram(),
            MBCTypes::Mbc7 => self.mbc7_write_ram(),
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
                if value == 0x0A {
                    self.ram_enabled = true;
                } else {
                    self.ram_enabled = false;
                }
                self.rom_data[address as usize] = value;
            }
            0x2000..= 0x3FFF => {
                let mut v = value & 0xF1;
                if v == 0 || v == 1 {
                    v = 1;
                    self.current_rom_bank = 0;
                } else {
                    if v > self.ram_banks {
                        v = self.ram_banks - 1;
                    } else {
                        v = v - 1;
                    }
                    self.current_rom_bank = v;
                }
                self.rom_data[address as usize] = v;
            },
            0x4000..= 0x5FFF => {
                if self.banking_mode {
                    let v = value & 0x03;
                    let new_rom_bank = (v << 5) + (self.current_rom_bank + 1);
                    self.current_rom_bank = new_rom_bank - 1;
                    self.rom_data[(address + (0x4000 * self.current_rom_bank as u16)) as usize] = new_rom_bank;
                }
            },
            0x6000..= 0x7FFF => {
                // TODO: implement banking mode select
            }
            _ => (),
        };
    }

    fn mm01_read_rom(&mut self, address: u16) {
        
    }

    fn mm01_write_rom(&mut self) {

    }

    fn mbc2_read_rom(&mut self) {

    }

    fn mbc2_write_rom(&mut self) {

    }
    
    fn mbc3_read_rom(&mut self) {

    }

    fn mbc3_write_rom(&mut self) {
        
    }

    fn mbc5_read_rom(&mut self) {

    }

    fn mbc5_write_rom(&mut self) {
        
    }

    fn mbc6_read_rom(&mut self) {

    }

    fn mbc6_write_rom(&mut self) {
        
    }

    fn mbc7_read_rom(&mut self) {

    }

    fn mbc7_write_rom(&mut self) {
        
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

    fn mbc2_read_ram(&mut self, address: u16) {
        
    }

    fn mbc2_write_ram(&mut self) {
        
    }

    fn mbc3_read_ram(&mut self) {
        
    }

    fn mbc3_write_ram(&mut self) {
        
    }

    fn mbc5_read_ram(&mut self) {
        
    }

    fn mbc5_write_ram(&mut self) {
        
    }

    fn mbc6_read_ram(&mut self) {
        
    }

    fn mbc6_write_ram(&mut self) {
        
    }

    fn mbc7_read_ram(&mut self) {
        
    }

    fn mbc7_write_ram(&mut self) {
        
    }
}