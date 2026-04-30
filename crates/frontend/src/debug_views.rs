use eframe::egui;
use std::collections::BTreeSet;
use sturdygb_core::gb::{MemoryWriteEvent, OamSprite};

const VRAM_TILES_PER_ROW: usize = 16;
const BG_MAP_TILES_PER_ROW: usize = 32;
const OAM_SPRITES_PER_ROW: usize = 10;

pub fn find_watchpoint_hit<'a>(
    writes: &'a [MemoryWriteEvent],
    watchpoints: &BTreeSet<u16>,
) -> Option<&'a MemoryWriteEvent> {
    writes
        .iter()
        .find(|write| watchpoints.contains(&write.address))
}

pub fn parse_hex_u16(input: &str, default: u16) -> u16 {
    let trimmed = input.trim();
    let trimmed = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    u16::from_str_radix(trimmed, 16).unwrap_or(default)
}

pub fn parse_hex_usize(input: &str, default: usize) -> usize {
    let trimmed = input.trim();
    let trimmed = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    usize::from_str_radix(trimmed, 16).unwrap_or(default)
}

pub fn format_byte_list(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join(" ")
}

fn decode_tile_pixel(
    tile_bank0: &[u8],
    tile_bank1: Option<&[u8]>,
    tile_index: usize,
    x: usize,
    y: usize,
    bank: usize,
) -> u8 {
    let tile_data = if bank == 1 {
        tile_bank1.unwrap_or(tile_bank0)
    } else {
        tile_bank0
    };
    let tile_offset = tile_index.saturating_mul(16);
    if tile_offset + 15 >= tile_data.len() {
        return 0;
    }
    let low = tile_data[tile_offset + y * 2];
    let high = tile_data[tile_offset + y * 2 + 1];
    let bit = 7 - x;
    (((high >> bit) & 1) << 1) | ((low >> bit) & 1)
}

fn shade_to_rgba(shade: u8) -> [u8; 4] {
    let value = match shade {
        0 => 255,
        1 => 192,
        2 => 96,
        _ => 0,
    };
    [value, value, value, 255]
}

pub fn build_vram_image(vram: &[u8]) -> (egui::ColorImage, usize, usize) {
    let tile_count = vram.len() / 16;
    let tile_rows = tile_count.div_ceil(VRAM_TILES_PER_ROW);
    let width = VRAM_TILES_PER_ROW * 8;
    let height = tile_rows * 8;
    let mut rgba = vec![0; width * height * 4];

    for tile_index in 0..tile_count {
        let tile_x = (tile_index % VRAM_TILES_PER_ROW) * 8;
        let tile_y = (tile_index / VRAM_TILES_PER_ROW) * 8;

        for row in 0..8 {
            for col in 0..8 {
                let shade = decode_tile_pixel(vram, None, tile_index, col, row, 0);
                let pixel = shade_to_rgba(shade);
                let x = tile_x + col;
                let y = tile_y + row;
                let index = (y * width + x) * 4;
                rgba[index..index + 4].copy_from_slice(&pixel);
            }
        }
    }

    (
        egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba),
        width,
        height,
    )
}

pub fn build_bg_map_image(
    tile_bank0: &[u8],
    tile_bank1: Option<&[u8]>,
    tile_map: &[u8],
    attr_map: Option<&[u8]>,
    signed_mode: bool,
) -> (egui::ColorImage, usize, usize) {
    let width = BG_MAP_TILES_PER_ROW * 8;
    let height = BG_MAP_TILES_PER_ROW * 8;
    let mut rgba = vec![0; width * height * 4];

    for tile_y in 0..BG_MAP_TILES_PER_ROW {
        for tile_x in 0..BG_MAP_TILES_PER_ROW {
            let map_index = tile_y * BG_MAP_TILES_PER_ROW + tile_x;
            let tile_number = tile_map.get(map_index).copied().unwrap_or(0);
            let attributes = attr_map.and_then(|map| map.get(map_index)).copied().unwrap_or(0);
            let tile_bank = if attributes & 0x08 != 0 { 1 } else { 0 };
            let hflip = attributes & 0x20 != 0;
            let vflip = attributes & 0x40 != 0;
            let tile_index = if signed_mode {
                ((tile_number as i8 as i16) + 128) as usize
            } else {
                tile_number as usize
            };

            for row in 0..8 {
                for col in 0..8 {
                    let sample_x = if hflip { 7 - col } else { col };
                    let sample_y = if vflip { 7 - row } else { row };
                    let shade = decode_tile_pixel(
                        tile_bank0,
                        tile_bank1,
                        tile_index,
                        sample_x,
                        sample_y,
                        tile_bank,
                    );
                    let pixel = shade_to_rgba(shade);
                    let x = tile_x * 8 + col;
                    let y = tile_y * 8 + row;
                    let index = (y * width + x) * 4;
                    rgba[index..index + 4].copy_from_slice(&pixel);
                }
            }
        }
    }

    (
        egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba),
        width,
        height,
    )
}

pub fn build_oam_image(
    tile_bank0: &[u8],
    tile_bank1: Option<&[u8]>,
    sprites: &[OamSprite],
    sprite_height: usize,
) -> (egui::ColorImage, usize, usize) {
    let rows = sprites.len().div_ceil(OAM_SPRITES_PER_ROW);
    let width = OAM_SPRITES_PER_ROW * 8;
    let height = rows * sprite_height.max(8);
    let mut rgba = vec![0; width * height * 4];

    for (index, sprite) in sprites.iter().enumerate() {
        let sprite_x = (index % OAM_SPRITES_PER_ROW) * 8;
        let sprite_y = (index / OAM_SPRITES_PER_ROW) * sprite_height.max(8);
        let tile_bank = if sprite.attributes & 0x08 != 0 { 1 } else { 0 };
        let hflip = sprite.attributes & 0x20 != 0;
        let vflip = sprite.attributes & 0x40 != 0;

        for row in 0..sprite_height {
            let tile_index = if sprite_height == 16 {
                ((sprite.tile_number & 0xFE) as usize) + (row / 8)
            } else {
                sprite.tile_number as usize
            };
            for col in 0..8 {
                let sample_x = if hflip { 7 - col } else { col };
                let local_y = row % 8;
                let sample_y = if vflip { 7 - local_y } else { local_y };
                let shade = decode_tile_pixel(
                    tile_bank0,
                    tile_bank1,
                    tile_index,
                    sample_x,
                    sample_y,
                    tile_bank,
                );
                let pixel = shade_to_rgba(shade);
                let x = sprite_x + col;
                let y = sprite_y + row;
                let index = (y * width + x) * 4;
                rgba[index..index + 4].copy_from_slice(&pixel);
            }
        }
    }

    (
        egui::ColorImage::from_rgba_unmultiplied([width, height], &rgba),
        width,
        height,
    )
}
