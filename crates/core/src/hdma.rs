// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
use super::memory::Memory;

pub struct Hdma {
    hdma1: u8,
    hdma2: u8,
    hdma3: u8,
    hdma4: u8,
    hdma5: u8,
    enabled: bool,
}

impl Hdma {
    pub fn new() -> Self {
        Self {
            hdma1: 0xFF,
            hdma2: 0xFF,
            hdma3: 0xFF,
            hdma4: 0xFF,
            hdma5: 0xFF,
            enabled: false,
        }
    }

    pub fn get_hdma_source(&self) -> u16 {
        ((self.hdma1 as u16) << 8) | (self.hdma2 as u16)
    }

    pub fn get_hdma_destination(&self) -> u16 {
        ((self.hdma3 as u16) << 8) | (self.hdma4 as u16) | 0x8000
    }
}

impl Memory for Hdma {
    #[allow(unused_variables)]
    fn read_byte(&self, address: u16) -> u8 {
        0xFF // TCAGB says it always returns 0xFF when read
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF51 => self.hdma1 = value,
            0xFF52 => self.hdma2 = value & 0xF0,
            0xFF53 => self.hdma3 = value & 0x1F,
            0xFF54 => self.hdma4 = value & 0xF0,
            0xFF55 => self.hdma5 = value,
            _ => unreachable!(),
        };
    }
}
