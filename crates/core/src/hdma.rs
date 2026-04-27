// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
use super::memory::Memory;

pub enum HdmaAction {
    None,
    General,
    HBlank,
    Cancelled,
}

pub struct Hdma {
    hdma1: u8,
    hdma2: u8,
    hdma3: u8,
    hdma4: u8,
    hdma5: u8,
    active: bool,
    hblank_mode: bool,
    remaining_blocks: u8,
}

impl Hdma {
    pub fn new() -> Self {
        Self {
            hdma1: 0xFF,
            hdma2: 0xFF,
            hdma3: 0xFF,
            hdma4: 0xFF,
            hdma5: 0xFF,
            active: false,
            hblank_mode: false,
            remaining_blocks: 0,
        }
    }

    pub fn get_hdma_source(&self) -> u16 {
        (((self.hdma1 as u16) << 8) | (self.hdma2 as u16)) & 0xFFF0
    }

    pub fn get_hdma_destination(&self) -> u16 {
        0x8000 | ((((self.hdma3 as u16) << 8) | (self.hdma4 as u16)) & 0x1FF0)
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_hblank_mode(&self) -> bool {
        self.active && self.hblank_mode
    }

    pub fn write_hdma5(&mut self, value: u8) -> HdmaAction {
        if self.active && self.hblank_mode && value & 0x80 == 0 {
            self.active = false;
            self.hblank_mode = false;
            self.hdma5 = 0x80 | self.remaining_blocks.saturating_sub(1);
            return HdmaAction::Cancelled;
        }

        self.active = true;
        self.hblank_mode = value & 0x80 != 0;
        self.remaining_blocks = (value & 0x7F).wrapping_add(1);
        self.hdma5 = value;

        if self.hblank_mode {
            HdmaAction::HBlank
        } else {
            HdmaAction::General
        }
    }

    pub fn finish_transfer(&mut self) {
        self.active = false;
        self.hblank_mode = false;
        self.remaining_blocks = 0;
        self.hdma5 = 0xFF;
    }

    pub fn advance_block(&mut self) {
        let source = self.get_hdma_source().wrapping_add(0x10);
        let destination = self.get_hdma_destination().wrapping_add(0x10);

        self.hdma1 = (source >> 8) as u8;
        self.hdma2 = (source & 0xF0) as u8;
        self.hdma3 = ((destination >> 8) as u8) & 0x1F;
        self.hdma4 = (destination & 0xF0) as u8;

        self.remaining_blocks = self.remaining_blocks.saturating_sub(1);
        if self.remaining_blocks == 0 {
            self.finish_transfer();
        }
    }

    fn status(&self) -> u8 {
        if self.active {
            self.remaining_blocks.saturating_sub(1) & 0x7F
        } else if self.remaining_blocks == 0 {
            0xFF
        } else {
            0x80 | (self.remaining_blocks.saturating_sub(1) & 0x7F)
        }
    }
}

impl Memory for Hdma {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF55 => self.status(),
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF51 => self.hdma1 = value,
            0xFF52 => self.hdma2 = value & 0xF0,
            0xFF53 => self.hdma3 = value & 0x1F,
            0xFF54 => self.hdma4 = value & 0xF0,
            0xFF55 => {
                let _ = self.write_hdma5(value);
            }
            _ => unreachable!(),
        };
    }
}

impl super::gb::Gb {
    fn hdma_block_mcycles(&self) -> usize {
        match self.speed_mode {
            super::gb::SpeedMode::Normal => 8,
            super::gb::SpeedMode::Double => 16,
        }
    }

    fn transfer_hdma_block(&mut self) -> bool {
        let source = self.ppu.hdma.get_hdma_source();
        let destination = self.ppu.hdma.get_hdma_destination();

        if destination > 0x9FF0 {
            self.ppu.hdma.finish_transfer();
            return false;
        }

        for offset in 0..0x10u16 {
            let value = self.read_byte(source.wrapping_add(offset));
            self.ppu
                .write_vram_dma_byte(destination.wrapping_add(offset), value);
        }

        self.ppu.hdma.advance_block();
        true
    }

    pub fn write_hdma_register(&mut self, address: u16, value: u8) {
        if self.gb_type != super::gb::GbTypes::Cgb {
            return;
        }

        match address {
            0xFF51..=0xFF54 => self.ppu.hdma.write_byte(address, value),
            0xFF55 => match self.ppu.hdma.write_hdma5(value) {
                HdmaAction::General => {
                    let mut transferred_blocks = 0usize;
                    while self.ppu.hdma.is_active() {
                        if !self.transfer_hdma_block() {
                            break;
                        }
                        transferred_blocks += 1;
                    }
                    self.cpu.instruction_cycles += transferred_blocks * self.hdma_block_mcycles();
                }
                HdmaAction::HBlank | HdmaAction::Cancelled | HdmaAction::None => {}
            },
            _ => unreachable!(),
        }
    }

    pub fn hdma_hblank_step(&mut self) {
        if !self.ppu.hdma.is_hblank_mode() || self.cpu.is_halted || self.cpu.is_stopped {
            return;
        }
        if self.ppu.get_ly() >= 144 {
            return;
        }

        let _ = self.transfer_hdma_block();
    }
}

#[cfg(test)]
mod tests {
    use crate::gb::ModelSelection;
    use crate::prelude::GbInstance;

    fn make_test_rom(cgb_flag: u8) -> Vec<u8> {
        let mut rom = vec![0u8; 0x8000];
        rom[0x0143] = cgb_flag;
        rom[0x0147] = 0x00;
        rom[0x0148] = 0x00;
        rom[0x0149] = 0x00;

        let mut checksum = 0u8;
        for byte in &rom[0x0134..=0x014C] {
            checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
        }
        rom[0x014D] = checksum;
        rom
    }

    fn make_cgb() -> crate::gb::Gb {
        GbInstance::build_from_bytes_with_model(make_test_rom(0xC0), None, ModelSelection::Auto)
            .expect("CGB instance should build")
    }

    #[test]
    fn general_hdma_copies_full_block_into_current_vram_bank() {
        let mut gb = make_cgb();
        gb.write_byte(0xFF4F, 0x01);
        for offset in 0..0x10u16 {
            gb.write_byte(0xC000 + offset, offset as u8);
        }

        gb.write_byte(0xFF51, 0xC0);
        gb.write_byte(0xFF52, 0x00);
        gb.write_byte(0xFF53, 0x00);
        gb.write_byte(0xFF54, 0x00);
        gb.write_byte(0xFF55, 0x00);

        assert_eq!(&gb.ppu.vram[0x2000..0x2010], (0u8..=0x0F).collect::<Vec<_>>().as_slice());
        assert_eq!(gb.read_byte(0xFF55), 0xFF);
    }

    #[test]
    fn hblank_hdma_copies_one_block_per_step() {
        let mut gb = make_cgb();
        for offset in 0..0x20u16 {
            gb.write_byte(0xC000 + offset, (0x80 + offset) as u8);
        }

        gb.write_byte(0xFF51, 0xC0);
        gb.write_byte(0xFF52, 0x00);
        gb.write_byte(0xFF53, 0x00);
        gb.write_byte(0xFF54, 0x00);
        gb.write_byte(0xFF55, 0x81);

        gb.hdma_hblank_step();
        assert_eq!(&gb.ppu.vram[0x0000..0x0010], (0x80u8..=0x8F).collect::<Vec<_>>().as_slice());
        assert_eq!(gb.read_byte(0xFF55), 0x00);

        gb.hdma_hblank_step();
        assert_eq!(&gb.ppu.vram[0x0010..0x0020], (0x90u8..=0x9F).collect::<Vec<_>>().as_slice());
        assert_eq!(gb.read_byte(0xFF55), 0xFF);
    }
}
