// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
use super::gb::Gb;


pub struct Dma {
    pub active: bool,
    pub byte: u8,
    pub value: u8,
    pub delay: u8,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            active: false,
            byte: 0,
            value: 0,
            delay: 0,
        }
    }
    
    pub fn start_transfer(&mut self, value: u8) {
        self.active = true;
        self.value = value;
        self.byte = 0;
        self.delay = 2;
    }
}

impl Gb {
    pub fn run_dma(&mut self) {
        if self.ppu.dma.active {
            self.dma_transfer()
        }
    }

    pub fn dma_transfer(&mut self) {
        if self.ppu.dma.delay > 0 {
            self.ppu.dma.delay -= 1;
            return;
        }
        
        let address = self.ppu.dma.value as u16 * 0x100 + self.ppu.dma.byte as u16;
        let value = self.read_byte(address);
        self.write_byte(address, value);
        self.ppu.dma.byte = self.ppu.dma.byte.wrapping_add(1);
        self.ppu.dma.active = self.ppu.dma.byte < 0xA0;
        if !self.ppu.dma.active {
            self.ppu.dma.byte = 0;
            self.ppu.dma.delay = 2;
            self.ppu.dma.value = 0;
        }
    }
}
