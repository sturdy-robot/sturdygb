// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct RomOnly {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
}

impl RomOnly {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader) -> Self {
        Self { header, rom_data }
    }
}

#[allow(unused_variables)]
impl Mbc for RomOnly {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom_data.get(address as usize).copied().unwrap_or(0xFF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::MBCTypes;

    #[test]
    fn short_rom_reads_return_ff_instead_of_panicking() {
        let rom = RomOnly::new(
            vec![0x12; 0x100],
            CartridgeHeader {
                entry: [0; 4],
                title: String::new(),
                logo: [0; 0x30],
                cgb_flag: 0,
                sgb_flag: false,
                mbc_type: MBCTypes::RomOnly,
                rom_size: 0x100,
                ram_size: 0,
                company: String::new(),
            },
        );

        assert_eq!(rom.read_rom(0x3FFF), 0xFF);
    }
}
