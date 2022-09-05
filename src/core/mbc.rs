use super::cartridge::Cartridge;

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

pub struct MBC {
    cartridge: Cartridge,
    mbc_type: MBCTypes,
}

impl MBC {
    pub fn new(cartridge: Cartridge) -> Self {
        let mbc_type: MBCTypes = match cartridge.rom_data[0x0147] {
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
        
        Self {
            cartridge: cartridge,
            mbc_type: mbc_type,
        }
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.cartridge.rom_data[address as usize]
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
