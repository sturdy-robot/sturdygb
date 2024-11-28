// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::cartridge::GbMode;
use super::dma::Dma;
use super::gb::Gb;
use super::hdma::Hdma;
use super::interrupts::Interrupt;
use super::memory::Memory;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    SearchingOAM = 2,
    Transferring = 3,
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
    pub frame_ready: bool,
    screen: [[u8; 160]; 144],
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
            stat: 0x85, // Mode 2 (OAM Search), LYC=LY interrupt enabled
            scy: 0,
            scx: 0,
            ly: 0, // Start at scanline 0
            lyc: 0,
            bgp: 0xFC,  // Default background palette
            obp0: 0xFF, // Default sprite palette 0
            obp1: 0xFF, // Default sprite palette 1
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
            frame_ready: false,
            dma: Dma::new(),
            hdma: Hdma::new(),
            mode_clock: 0,
            screen: [[0; 160]; 144],
        }
    }

    pub fn get_ppu_mode(&self) -> PpuMode {
        let mode = self.stat & 0x03;
        match mode {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::SearchingOAM,
            3 => PpuMode::Transferring,
            _ => PpuMode::HBlank,
        }
    }

    pub fn set_mode(&mut self, mode: PpuMode) {
        self.stat &= !0x03;
        self.stat |= mode as u8;

        // Check if we should trigger STAT interrupt for this mode
        let mode_int_flag = match mode {
            PpuMode::HBlank => self.stat & 0x08,       // Mode 0
            PpuMode::VBlank => self.stat & 0x10,       // Mode 1
            PpuMode::SearchingOAM => self.stat & 0x20, // Mode 2
            PpuMode::Transferring => 0,                // Mode 3 doesn't trigger interrupt
        };

        if mode_int_flag != 0 {
            self.stat |= 0x04; // Set interrupt request bit
        }
    }

    pub fn check_lyc(&mut self) {
        // Update LY=LYC flag
        if self.ly == self.lyc {
            self.stat |= 0x04; // Set coincidence flag
            if self.stat & 0x40 != 0 {
                // LY=LYC interrupt enabled
                self.stat |= 0x04; // Request STAT interrupt
            }
        } else {
            self.stat &= !0x04; // Clear coincidence flag
        }
    }

    pub fn render_scanline(&mut self) {
        // Skip rendering if LCD is off
        if self.lcdc & 0x80 == 0 {
            return;
        }

        let current_line = self.ly as usize;
        if current_line >= 144 {
            return;
        }

        // Background rendering
        let bg_enabled = self.lcdc & 0x01 != 0;
        let bg_tile_map = if self.lcdc & 0x08 != 0 {
            0x9C00
        } else {
            0x9800
        };
        let bg_tile_data = if self.lcdc & 0x10 != 0 {
            0x8000
        } else {
            0x8800
        };
        let signed_tile_numbers = bg_tile_data == 0x8800;

        // Window rendering
        let window_enabled = self.lcdc & 0x20 != 0;
        let window_tile_map = if self.lcdc & 0x40 != 0 {
            0x9C00
        } else {
            0x9800
        };
        let window_y = self.wy as usize;
        let window_x = self.wx.wrapping_sub(7) as usize;

        // For each pixel in the scanline
        for x in 0..160 {
            let mut pixel = 0;

            if bg_enabled {
                // Calculate background tile coordinates
                let bg_x = (self.scx as usize + x) & 0xFF;
                let bg_y = (self.scy as usize + current_line) & 0xFF;
                let tile_x = bg_x >> 3;
                let tile_y = bg_y >> 3;
                let tile_pixel_x = bg_x & 7;
                let tile_pixel_y = bg_y & 7;

                // Get tile number from tile map
                let tile_map_addr = bg_tile_map + tile_y * 32 + tile_x;
                let tile_number = self.vram[(tile_map_addr & 0x1FFF) as usize];

                // Get tile data address
                let tile_addr = if signed_tile_numbers {
                    let signed_tile = tile_number as i8 as i16;
                    let offset = (signed_tile + 128) * 16;
                    (0x8800 + offset as usize) & 0x1FFF
                } else {
                    let offset = tile_number as usize * 16;
                    (bg_tile_data + offset) & 0x1FFF
                };

                // Get tile row data
                let row_addr = tile_addr + (tile_pixel_y * 2);
                let tile_data_low = self.vram[row_addr];
                let tile_data_high = self.vram[row_addr + 1];

                // Extract pixel color from tile data
                let color_bit = 7 - tile_pixel_x;
                let low_bit = (tile_data_low >> color_bit) & 1;
                let high_bit = (tile_data_high >> color_bit) & 1;
                pixel = (high_bit << 1) | low_bit;
            }

            // Check if we should draw window at this position
            if window_enabled && current_line >= window_y && x >= window_x {
                let window_rel_y = current_line - window_y;
                let window_rel_x = x - window_x;
                let tile_x = window_rel_x >> 3;
                let tile_y = window_rel_y >> 3;
                let tile_pixel_x = window_rel_x & 7;
                let tile_pixel_y = window_rel_y & 7;

                // Get window tile number
                let tile_map_addr = window_tile_map + tile_y * 32 + tile_x;
                let tile_number = self.vram[(tile_map_addr & 0x1FFF) as usize];

                // Get window tile data
                let tile_addr = if signed_tile_numbers {
                    let signed_tile = tile_number as i8 as i16;
                    let offset = (signed_tile + 128) * 16;
                    (0x8800 + offset as usize) & 0x1FFF
                } else {
                    let offset = tile_number as usize * 16;
                    (bg_tile_data + offset) & 0x1FFF
                };

                let row_addr = tile_addr + (tile_pixel_y * 2);
                let tile_data_low = self.vram[row_addr];
                let tile_data_high = self.vram[row_addr + 1];

                let color_bit = 7 - tile_pixel_x;
                let low_bit = (tile_data_low >> color_bit) & 1;
                let high_bit = (tile_data_high >> color_bit) & 1;
                pixel = (high_bit << 1) | low_bit;
            }

            // Apply background palette
            let palette_pixel = (self.bgp >> (pixel * 2)) & 0x03;
            self.screen[current_line][x] = palette_pixel;
        }

        // Sprite rendering
        if self.lcdc & 0x02 != 0 {
            // Sprites enabled
            let tall_sprites = self.lcdc & 0x04 != 0;
            let sprite_height = if tall_sprites { 16 } else { 8 };

            // Check all sprites in OAM
            for sprite_index in 0..40 {
                let oam_addr = sprite_index * 4;
                let sprite_y = self.oam[oam_addr].wrapping_sub(16);
                let sprite_x = self.oam[oam_addr + 1].wrapping_sub(8);
                let tile_number = self.oam[oam_addr + 2];
                let attributes = self.oam[oam_addr + 3];

                // Check if sprite is visible on this scanline
                if (current_line as u8) >= sprite_y
                    && (current_line as u8) < sprite_y.wrapping_add(sprite_height as u8)
                {
                    let palette = if attributes & 0x10 != 0 {
                        self.obp1
                    } else {
                        self.obp0
                    };
                    let flip_x = attributes & 0x20 != 0;
                    let flip_y = attributes & 0x40 != 0;
                    let behind_bg = attributes & 0x80 != 0;

                    let line = (current_line as u8).wrapping_sub(sprite_y);
                    let actual_line = if flip_y {
                        sprite_height - 1 - (line as usize)
                    } else {
                        line as usize
                    };

                    // Get tile data for sprite
                    let tile_addr = if tall_sprites {
                        let tile_num = tile_number & !1;
                        0x8000 + (tile_num as usize * 16) + (if actual_line >= 8 { 16 } else { 0 })
                    } else {
                        0x8000 + (tile_number as usize * 16)
                    };

                    // Get tile row data
                    let row_addr = (tile_addr & 0x1FFF) + ((actual_line & 7) * 2);
                    let tile_data_low = self.vram[row_addr];
                    let tile_data_high = self.vram[row_addr + 1];

                    // Draw sprite pixels
                    for x_pixel in 0..8 {
                        let screen_x = sprite_x.wrapping_add(x_pixel);
                        if screen_x >= 160 {
                            continue;
                        }

                        let bit = if flip_x { x_pixel } else { 7 - x_pixel };
                        let low_bit = (tile_data_low >> bit) & 1;
                        let high_bit = (tile_data_high >> bit) & 1;
                        let color = (high_bit << 1) | low_bit;

                        // Skip transparent pixels (color 0)
                        if color == 0 {
                            continue;
                        }

                        // Apply sprite palette
                        let palette_pixel = (palette >> (color * 2)) & 0x03;

                        // Check sprite priority
                        if !behind_bg || self.screen[current_line][screen_x as usize] == 0 {
                            self.screen[current_line][screen_x as usize] = palette_pixel;
                        }
                    }
                }
            }
        }
    }

    pub fn get_screen(&mut self) -> &[[u8; 160]; 144] {
        self.frame_ready = false;
        &self.screen
    }

    pub fn get_ly(&self) -> u8 {
        self.ly
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
        if self.ppu.lcdc & 0x80 == 0 {
            // LCD is off
            self.ppu.ly = 0;
            self.ppu.mode_clock = 0;
            self.ppu.stat &= !0x03; // Set mode to 0
            return;
        }

        for _ in 0..ticks {
            self.ppu.mode_clock += 1;

            match self.ppu.get_ppu_mode() {
                PpuMode::HBlank => {
                    if self.ppu.mode_clock >= 204 {
                        self.ppu.mode_clock = 0;
                        self.ppu.ly += 1;
                        self.ppu.check_lyc();

                        if self.ppu.ly == 144 {
                            self.ppu.set_mode(PpuMode::VBlank);
                            self.request_interrupt(Interrupt::Vblank);
                            if self.ppu.stat & 0x10 != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                            // self.ppu.frame_ready = true;
                        } else {
                            self.ppu.set_mode(PpuMode::SearchingOAM);
                            if self.ppu.stat & 0x20 != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                        }
                    }
                }
                PpuMode::VBlank => {
                    if self.ppu.mode_clock >= 456 {
                        self.ppu.mode_clock = 0;
                        self.ppu.ly += 1;

                        if self.ppu.ly > 153 {
                            self.ppu.ly = 0;
                            self.ppu.frame_ready = true;
                            self.ppu.set_mode(PpuMode::SearchingOAM);
                            if self.ppu.stat & 0x20 != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                        }
                        self.ppu.check_lyc();
                    }
                }
                PpuMode::SearchingOAM => {
                    if self.ppu.mode_clock >= 80 {
                        self.ppu.mode_clock = 0;
                        self.ppu.set_mode(PpuMode::Transferring);
                    }
                }
                PpuMode::Transferring => {
                    if self.ppu.mode_clock >= 172 {
                        self.ppu.mode_clock = 0;
                        self.ppu.set_mode(PpuMode::HBlank);
                        if self.ppu.stat & 0x08 != 0 {
                            self.request_interrupt(Interrupt::LcdStat);
                        }
                        self.ppu.render_scanline();
                    }
                }
            }
        }
    }
}
