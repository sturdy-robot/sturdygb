use crate::core::mbc::{MbcBase, CartridgeHeader, Mbc};


pub struct RomOnly {
    mbc: MbcBase,
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
}
