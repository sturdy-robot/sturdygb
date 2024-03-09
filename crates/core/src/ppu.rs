// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::dma::Dma;
use super::gb::Gb;
use super::hdma::Hdma;
use super::interrupts::Interrupt;
use super::cartridge::GbMode;
use super::Memory;

pub enum PpuMode {
    HBlank,
    VBlank,
    SearchingOAM,
    Transferring,
}

pub struct Ppu {
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    key1: u8,
    vbk: u8,
    bcps: u8,
    bcpd: u8,
    ocps: u8,
    ocpd: u8,
    svbk: u8,
    pub vram: Vec<u8>,
    pub oam: [u8; 0xA0],
    pub oam_corruption_bug: bool,
    pub mode: PpuMode,
    pub dma: Dma,
    pub hdma: Hdma,
    pub mode_clock: u32,
}

impl Ppu {
    pub fn new(gb_mode: &GbMode) -> Self {
        let vram: Vec<u8> = match gb_mode {
            GbMode::CgbMode => vec![0; 0x4000],
            _ => vec![0; 0x2000],
        };
        let oam: [u8; 0xA0] = [0; 0xA0];
        Self {
            lcdc: 0x91,
            stat: 0x81,
            scy: 0,
            scx: 0,
            ly: 0x91,
            lyc: 0,
            bgp: 0xFC,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            key1: 0xFF,
            vbk: 0xFF,
            bcps: 0xFF,
            bcpd: 0xFF,
            ocps: 0xFF,
            ocpd: 0xFF,
            svbk: 0xFF,
            vram,
            oam,
            oam_corruption_bug: false,
            mode: PpuMode::SearchingOAM,
            dma: Dma::new(),
            hdma: Hdma::new(),
            mode_clock: 0,
        }
    }

    pub fn get_ppu_mode(&self) -> PpuMode {
        match self.stat & 0x03 {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::SearchingOAM,
            3 => PpuMode::Transferring,
            _ => unreachable!(),
        }
    }
}

impl Memory for Ppu {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => {
                if self.vram.len() == 0x4000 {
                    self.vram[((self.vbk as usize & 1) * 0x2000) | ((address & 0x1FFF) as usize)]
                } else {
                    self.vram[(address & 0x1FFF) as usize]
                }
            }
            0xFE00..=0xFE9F => {
                if self.dma.active {
                    0xFF
                } else {
                    self.oam[address as usize - 0xFE00]
                }
            }
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0xFF,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4D => self.key1,
            0xFF4F => self.vbk,
            0xFF51..=0xFF55 => self.hdma.read_byte(address),
            0xFF68 => self.bcps,
            0xFF69 => self.bcpd,
            0xFF6A => self.ocps,
            0xFF6B => self.ocpd,
            0xFF70 => self.svbk,
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => match self.get_ppu_mode() {
                PpuMode::HBlank | PpuMode::VBlank | PpuMode::SearchingOAM => {
                    if self.vram.len() == 0x4000 {
                        self.vram
                            [((self.vbk as usize & 1) * 0x2000) | ((address & 0x1FFF) as usize)] =
                            value;
                    } else {
                        self.vram[(address & 0x1FFF) as usize] = value;
                    }
                }
                _ => {}
            },
            0xFE00..=0xFE9F => {
                if self.dma.active {
                    return;
                }

                self.oam[address as usize - 0xFE00] = value;
            }
            0xFF40 => self.lcdc = value,
            0xFF41 => {
                // Bit 7 is always set
                let v = value & 0b0111_1000;
                self.stat &= 0b1000_0111;
                self.stat |= v;
                self.mode = self.get_ppu_mode();
            }
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF46 => self.dma.start_transfer(value),
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF4D => self.key1 = value,
            0xFF4F => self.vbk = value,
            0xFF51..=0xFF55 => self.hdma.write_byte(address, value),
            0xFF68 => self.bcps = value,
            0xFF69 => self.bcpd = value,
            0xFF6A => self.ocps = value,
            0xFF6B => self.ocpd = value,
            0xFF70 => self.svbk = value,
            _ => {}
        };
    }
}

impl Gb {
    pub fn ppu_tick(&mut self, ticks: u32) {
        for _ in 0..ticks {
            match self.ppu.mode {
                PpuMode::HBlank => {
                    if self.ppu.mode_clock >= 204 {
                        self.ppu.mode_clock = 0;
                        // Update the LY register
                        self.ppu.ly += 1;

                        // Check for LY=LYC coincidence and request LCD STAT interrupt if necessary
                        if self.ppu.ly == self.ppu.lyc {
                            self.ppu.stat |= 0x04; // Set coincidence flag
                            if (self.ppu.stat & 0x40) != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                        } else {
                            self.ppu.stat &= !0x04; // Clear coincidence flag
                        }

                        // Check if HBlank period has ended and switch to the next mode
                        if self.ppu.ly == 143 {
                            self.ppu.mode = PpuMode::VBlank;
                            // Request VBlank interrupt
                            self.request_interrupt(Interrupt::Vblank);
                        } else {
                            self.ppu.mode = PpuMode::SearchingOAM;
                            // Request LCD STAT interrupt if necessary
                            if (self.ppu.stat & 0x20) != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                        }
                    }
                }
                PpuMode::VBlank => {
                    if self.ppu.mode_clock >= 456 {
                        // Update the LY register
                        self.ppu.ly += 1;

                        // Check if VBlank period has ended and switch to the next mode
                        if self.ppu.ly > 153 {
                            self.ppu.mode = PpuMode::SearchingOAM;
                            self.ppu.ly = 0;
                        }
                    }
                }
                PpuMode::SearchingOAM => {
                    // Check if Searching OAM period has ended and switch to the next mode
                    if self.ppu.mode_clock >= 80 {
                        self.ppu.mode = PpuMode::Transferring;
                        self.ppu.mode_clock = 0;
                    } else {
                        self.ppu.mode_clock += 1;
                    }
                }
                PpuMode::Transferring => {
                    if self.ppu.mode_clock >= 172 {
                        self.ppu.mode_clock = 0;
                        self.ppu.mode = PpuMode::HBlank;
                    }
                }
            }
        }
    }
}
