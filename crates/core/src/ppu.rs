// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::cartridge::GbMode;
use super::dma::Dma;
use super::gb::Gb;
use super::hdma::Hdma;
use super::interrupts::Interrupt;
use super::memory::Memory;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    SearchingOAM = 2,
    Transferring = 3,
}

#[derive(Copy, Clone)]
struct LineSprite {
    y: u8,
    x: u8,
    tile_number: u8,
    attributes: u8,
    oam_index: u8,
}

#[derive(Copy, Clone)]
struct BgPixel {
    color: u8,
}

#[derive(Copy, Clone)]
struct SpritePixel {
    color: u8,
    palette: u8,
    behind_bg: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum FetcherStep {
    GetTile,
    GetTileDataLow,
    GetTileDataHigh,
    Push,
}

pub struct Ppu {
    lcdc: u8,
    pub stat: u8,
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
    line_clock: u32,
    pub frame_ready: bool,
    screen: [[u8; 160]; 144],
    bg_fifo: VecDeque<BgPixel>,
    sprite_fifo: VecDeque<Option<SpritePixel>>,
    line_sprites: Vec<LineSprite>,
    oam_scan_index: usize,
    next_sprite_index: usize,
    fetcher_step: FetcherStep,
    fetcher_step_clock: u8,
    fetcher_map_x: usize,
    fetcher_using_window: bool,
    fetcher_tile_number: u8,
    fetcher_tile_data_low: u8,
    fetcher_tile_data_high: u8,
    fetch_x: i16,
    visible_x: u8,
    sprite_fetch_delay: u8,
    window_line_counter: u8,
    window_triggered: bool,
    window_rendering_this_line: bool,
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
            ly: 0,
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
            line_clock: 0,
            screen: [[0; 160]; 144],
            bg_fifo: VecDeque::with_capacity(16),
            sprite_fifo: VecDeque::with_capacity(16),
            line_sprites: Vec::with_capacity(10),
            oam_scan_index: 0,
            next_sprite_index: 0,
            fetcher_step: FetcherStep::GetTile,
            fetcher_step_clock: 0,
            fetcher_map_x: 0,
            fetcher_using_window: false,
            fetcher_tile_number: 0,
            fetcher_tile_data_low: 0,
            fetcher_tile_data_high: 0,
            fetch_x: 0,
            visible_x: 0,
            sprite_fetch_delay: 0,
            window_line_counter: 0,
            window_triggered: false,
            window_rendering_this_line: false,
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
        // Update mode bits without affecting other bits
        self.stat = (self.stat & !0x03) | (mode as u8);
        self.mode = mode;
    }

    pub fn check_lyc(&mut self) -> bool {
        let previous = self.stat & 0x04 != 0;
        let coincidence = self.ly == self.lyc;
        // Update coincidence flag (bit 2)
        if coincidence {
            self.stat |= 0x04;
        } else {
            self.stat &= !0x04;
        }
        coincidence && !previous && self.stat & 0x40 != 0
    }

    fn bg_enabled(&self) -> bool {
        self.lcdc & 0x01 != 0
    }

    fn sprites_enabled(&self) -> bool {
        self.lcdc & 0x02 != 0
    }

    fn tall_sprites(&self) -> bool {
        self.lcdc & 0x04 != 0
    }

    fn sprite_height(&self) -> u8 {
        if self.tall_sprites() {
            16
        } else {
            8
        }
    }

    fn window_enabled_for_line(&self) -> bool {
        self.bg_enabled() && self.lcdc & 0x20 != 0 && self.ly >= self.wy && self.wx <= 166
    }

    fn window_start_x(&self) -> i16 {
        self.wx as i16 - 7
    }

    fn clear_pixel_fifos(&mut self) {
        self.bg_fifo.clear();
        self.sprite_fifo.clear();
        self.next_sprite_index = 0;
        self.sprite_fetch_delay = 0;
        self.visible_x = 0;
    }

    fn reset_fetcher(&mut self, using_window: bool) {
        self.fetcher_using_window = using_window;
        self.fetcher_step = FetcherStep::GetTile;
        self.fetcher_step_clock = 0;
        self.fetcher_map_x = if using_window {
            0
        } else {
            (self.scx as usize >> 3) & 0x1F
        };
        self.fetcher_tile_number = 0;
        self.fetcher_tile_data_low = 0;
        self.fetcher_tile_data_high = 0;
    }

    fn reset_lcd_state(&mut self) {
        self.ly = 0;
        self.mode_clock = 0;
        self.line_clock = 0;
        self.window_line_counter = 0;
        self.window_triggered = false;
        self.window_rendering_this_line = false;
        self.oam_scan_index = 0;
        self.line_sprites.clear();
        self.fetch_x = 0;
        self.clear_pixel_fifos();
        self.reset_fetcher(false);
        self.set_mode(PpuMode::HBlank);
        let _ = self.check_lyc();
    }

    fn enable_lcd(&mut self) {
        self.ly = 0;
        self.mode_clock = 0;
        self.line_clock = 0;
        self.window_line_counter = 0;
        self.oam_scan_index = 0;
        self.line_sprites.clear();
        self.window_triggered = false;
        self.window_rendering_this_line = false;
        self.fetch_x = 0;
        self.clear_pixel_fifos();
        self.reset_fetcher(false);
        self.set_mode(PpuMode::SearchingOAM);
        let _ = self.check_lyc();
    }

    fn start_oam_search(&mut self) {
        self.mode_clock = 0;
        self.set_mode(PpuMode::SearchingOAM);
        self.oam_scan_index = 0;
        self.line_sprites.clear();
        self.window_triggered = false;
        self.window_rendering_this_line = false;
        self.fetch_x = 0;
        self.clear_pixel_fifos();
        self.reset_fetcher(false);
    }

    fn start_transfer(&mut self) {
        self.mode_clock = 0;
        self.set_mode(PpuMode::Transferring);
        self.line_sprites
            .sort_by_key(|sprite| (sprite.x, sprite.oam_index));
        let start_with_window = self.window_enabled_for_line() && self.window_start_x() <= 0;
        self.window_triggered = start_with_window;
        self.window_rendering_this_line = start_with_window;
        self.fetch_x = if start_with_window {
            0
        } else {
            -i16::from(self.scx & 0x07)
        };
        self.clear_pixel_fifos();
        self.reset_fetcher(start_with_window);
    }

    fn scan_oam_entry(&mut self, sprite_index: usize) {
        if self.line_sprites.len() >= 10 {
            return;
        }

        let oam_addr = sprite_index * 4;
        let sprite = LineSprite {
            y: self.oam[oam_addr],
            x: self.oam[oam_addr + 1],
            tile_number: self.oam[oam_addr + 2],
            attributes: self.oam[oam_addr + 3],
            oam_index: sprite_index as u8,
        };
        let top = i16::from(sprite.y) - 16;
        let line = i16::from(self.ly);
        let height = i16::from(self.sprite_height());

        if line >= top && line < top + height {
            self.line_sprites.push(sprite);
        }
    }

    fn tick_oam_search(&mut self) {
        if self.mode_clock % 2 == 0 && self.oam_scan_index < 40 {
            self.scan_oam_entry(self.oam_scan_index);
            self.oam_scan_index += 1;
        }
    }

    fn tile_map_base(&self, using_window: bool) -> usize {
        if using_window {
            if self.lcdc & 0x40 != 0 {
                0x1C00
            } else {
                0x1800
            }
        } else if self.lcdc & 0x08 != 0 {
            0x1C00
        } else {
            0x1800
        }
    }

    fn fetcher_tile_y(&self) -> usize {
        if self.fetcher_using_window {
            (self.window_line_counter as usize >> 3) & 0x1F
        } else {
            (((self.scy as usize) + (self.ly as usize)) & 0xFF) >> 3
        }
    }

    fn fetcher_pixel_y(&self) -> usize {
        if self.fetcher_using_window {
            self.window_line_counter as usize & 0x07
        } else {
            ((self.scy as usize) + (self.ly as usize)) & 0x07
        }
    }

    fn read_bg_tile_number(&self) -> u8 {
        let tile_map_base = self.tile_map_base(self.fetcher_using_window);
        let tile_addr = tile_map_base + self.fetcher_tile_y() * 32 + (self.fetcher_map_x & 0x1F);
        self.vram[tile_addr & 0x1FFF]
    }

    fn read_bg_tile_row_addr(&self, tile_number: u8) -> usize {
        let tile_row = self.fetcher_pixel_y() * 2;
        if self.lcdc & 0x10 != 0 {
            ((tile_number as usize) * 16 + tile_row) & 0x1FFF
        } else {
            let signed_tile = tile_number as i8 as i16;
            (0x1000i32 + i32::from(signed_tile) * 16 + tile_row as i32) as usize & 0x1FFF
        }
    }

    fn tick_fetcher(&mut self) {
        match self.fetcher_step {
            FetcherStep::Push => {
                if self.bg_fifo.len() <= 8 {
                    let bg_enabled = self.bg_enabled();
                    for bit in (0..8).rev() {
                        let color = if bg_enabled {
                            (((self.fetcher_tile_data_high >> bit) & 1) << 1)
                                | ((self.fetcher_tile_data_low >> bit) & 1)
                        } else {
                            0
                        };
                        self.bg_fifo.push_back(BgPixel { color });
                    }
                    while self.sprite_fifo.len() < self.bg_fifo.len() {
                        self.sprite_fifo.push_back(None);
                    }
                    self.fetcher_map_x = (self.fetcher_map_x + 1) & 0x1F;
                    self.fetcher_step = FetcherStep::GetTile;
                }
            }
            _ => {
                self.fetcher_step_clock = self.fetcher_step_clock.wrapping_add(1);
                if self.fetcher_step_clock < 2 {
                    return;
                }
                self.fetcher_step_clock = 0;

                match self.fetcher_step {
                    FetcherStep::GetTile => {
                        self.fetcher_tile_number = self.read_bg_tile_number();
                        self.fetcher_step = FetcherStep::GetTileDataLow;
                    }
                    FetcherStep::GetTileDataLow => {
                        let row_addr = self.read_bg_tile_row_addr(self.fetcher_tile_number);
                        self.fetcher_tile_data_low = self.vram[row_addr];
                        self.fetcher_step = FetcherStep::GetTileDataHigh;
                    }
                    FetcherStep::GetTileDataHigh => {
                        let row_addr = self.read_bg_tile_row_addr(self.fetcher_tile_number);
                        self.fetcher_tile_data_high = self.vram[(row_addr + 1) & 0x1FFF];
                        self.fetcher_step = FetcherStep::Push;
                    }
                    FetcherStep::Push => {}
                }
            }
        }
    }

    fn fetch_sprite(&mut self, sprite: LineSprite) {
        let sprite_height = self.sprite_height() as usize;
        let sprite_top = sprite.y.wrapping_sub(16);
        let line = self.ly.wrapping_sub(sprite_top) as usize;
        let flip_x = sprite.attributes & 0x20 != 0;
        let flip_y = sprite.attributes & 0x40 != 0;
        let behind_bg = sprite.attributes & 0x80 != 0;
        let palette = if sprite.attributes & 0x10 != 0 {
            self.obp1
        } else {
            self.obp0
        };
        let mut row = if flip_y {
            sprite_height - 1 - line
        } else {
            line
        };
        let mut tile_number = sprite.tile_number;

        if self.tall_sprites() {
            tile_number &= !1;
            if row >= 8 {
                tile_number = tile_number.wrapping_add(1);
            }
            row &= 0x07;
        }

        let row_addr = ((tile_number as usize) * 16 + row * 2) & 0x1FFF;
        let tile_data_low = self.vram[row_addr];
        let tile_data_high = self.vram[(row_addr + 1) & 0x1FFF];
        let left_edge = i16::from(sprite.x) - 8;

        for x in 0..8 {
            let screen_x = left_edge + x as i16;
            if screen_x < self.fetch_x {
                continue;
            }

            let bit = if flip_x { x } else { 7 - x };
            let color = (((tile_data_high >> bit) & 1) << 1) | ((tile_data_low >> bit) & 1);
            if color == 0 {
                continue;
            }

            let queue_index = (screen_x - self.fetch_x) as usize;
            if self.sprite_fifo.len() <= queue_index {
                self.sprite_fifo.resize(queue_index + 1, None);
            }
            if self.sprite_fifo[queue_index].is_none() {
                self.sprite_fifo[queue_index] = Some(SpritePixel {
                    color,
                    palette,
                    behind_bg,
                });
            }
        }
    }

    fn resolve_pixel(&self, bg_pixel: BgPixel, sprite_pixel: Option<SpritePixel>) -> u8 {
        if let Some(sprite_pixel) = sprite_pixel {
            if !self.bg_enabled() || !sprite_pixel.behind_bg || bg_pixel.color == 0 {
                return (sprite_pixel.palette >> (sprite_pixel.color * 2)) & 0x03;
            }
        }

        (self.bgp >> (bg_pixel.color * 2)) & 0x03
    }

    fn tick_transfer(&mut self) -> bool {
        if !self.window_triggered
            && self.window_enabled_for_line()
            && self.fetch_x >= self.window_start_x()
        {
            self.window_triggered = true;
            self.window_rendering_this_line = true;
            self.bg_fifo.clear();
            self.reset_fetcher(true);
        }

        if self.sprite_fetch_delay > 0 {
            self.sprite_fetch_delay -= 1;
            return false;
        }

        if self.sprites_enabled() && self.next_sprite_index < self.line_sprites.len() {
            let sprite = self.line_sprites[self.next_sprite_index];
            if i16::from(sprite.x) - 8 <= self.fetch_x {
                self.fetch_sprite(sprite);
                self.next_sprite_index += 1;
                self.sprite_fetch_delay = 5;
                return false;
            }
        }

        self.tick_fetcher();

        if self.bg_fifo.len() > 8 {
            let bg_pixel = self.bg_fifo.pop_front().unwrap();
            let sprite_pixel = if self.sprite_fifo.is_empty() {
                None
            } else {
                self.sprite_fifo.pop_front().unwrap()
            };

            if self.fetch_x >= 0 && self.visible_x < 160 {
                let current_line = self.ly as usize;
                let screen_x = self.visible_x as usize;
                self.screen[current_line][screen_x] = self.resolve_pixel(bg_pixel, sprite_pixel);
                self.visible_x = self.visible_x.wrapping_add(1);
            }

            self.fetch_x += 1;
        }

        self.visible_x >= 160
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
                if self.get_ppu_mode() == PpuMode::Transferring {
                    0xFF
                } else if self.vram.len() == 0x4000 {
                    self.vram[((self.vbk as usize & 1) * 0x2000) | ((address & 0x1FFF) as usize)]
                } else {
                    self.vram[(address & 0x1FFF) as usize]
                }
            }
            0xFE00..=0xFE9F => {
                if self.dma.active
                    || matches!(
                        self.get_ppu_mode(),
                        PpuMode::SearchingOAM | PpuMode::Transferring
                    )
                {
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
                if self.dma.active
                    || matches!(
                        self.get_ppu_mode(),
                        PpuMode::SearchingOAM | PpuMode::Transferring
                    )
                {
                    return;
                }

                self.oam[address as usize - 0xFE00] = value;
            }
            0xFF40 => {
                let was_enabled = self.lcdc & 0x80 != 0;
                self.lcdc = value;
                let is_enabled = self.lcdc & 0x80 != 0;
                if was_enabled && !is_enabled {
                    self.reset_lcd_state();
                } else if !was_enabled && is_enabled {
                    self.enable_lcd();
                }
            }
            0xFF41 => {
                // Bit 7 is always set
                let v = value & 0b0111_1000;
                self.stat &= 0b1000_0111;
                self.stat |= v;
                self.mode = self.get_ppu_mode();
            }
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => {} // LY is read-only,
            0xFF45 => {
                self.lyc = value;
                let _ = self.check_lyc();
            }
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
            return;
        }

        for _ in 0..ticks {
            self.ppu.line_clock = self.ppu.line_clock.wrapping_add(1);
            self.ppu.mode_clock = self.ppu.mode_clock.wrapping_add(1);

            match self.ppu.get_ppu_mode() {
                PpuMode::HBlank => {
                    if self.ppu.line_clock >= 456 {
                        self.ppu.line_clock = 0;
                        self.ppu.mode_clock = 0;
                        if self.ppu.window_rendering_this_line {
                            self.ppu.window_line_counter =
                                self.ppu.window_line_counter.wrapping_add(1);
                        }
                        self.ppu.ly = self.ppu.ly.wrapping_add(1);
                        if self.ppu.check_lyc() {
                            self.request_interrupt(Interrupt::LcdStat);
                        }

                        if self.ppu.ly == 144 {
                            self.ppu.set_mode(PpuMode::VBlank);
                            self.request_interrupt(Interrupt::Vblank);
                            if self.ppu.stat & 0x10 != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                        } else {
                            self.ppu.start_oam_search();
                            if self.ppu.stat & 0x20 != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                        }
                    }
                }
                PpuMode::VBlank => {
                    if self.ppu.line_clock >= 456 {
                        self.ppu.line_clock = 0;
                        self.ppu.mode_clock = 0;
                        self.ppu.ly = self.ppu.ly.wrapping_add(1);

                        if self.ppu.ly > 153 {
                            self.ppu.ly = 0;
                            self.ppu.frame_ready = true;
                            self.ppu.window_line_counter = 0;
                            self.ppu.start_oam_search();
                            if self.ppu.stat & 0x20 != 0 {
                                self.request_interrupt(Interrupt::LcdStat);
                            }
                        }
                        if self.ppu.check_lyc() {
                            self.request_interrupt(Interrupt::LcdStat);
                        }
                    }
                }
                PpuMode::SearchingOAM => {
                    self.ppu.tick_oam_search();
                    if self.ppu.mode_clock >= 80 {
                        self.ppu.start_transfer();
                    }
                }
                PpuMode::Transferring => {
                    if self.ppu.tick_transfer() {
                        self.ppu.mode_clock = 0;
                        self.ppu.set_mode(PpuMode::HBlank);
                        if self.ppu.stat & 0x08 != 0 {
                            self.request_interrupt(Interrupt::LcdStat);
                        }
                    }
                }
            }
        }
    }
}
