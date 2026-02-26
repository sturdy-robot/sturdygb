// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
use super::cartridge::{load_cartridge, GbMode};
use super::gb::{Gb, GbTypes};

pub struct GbInstance;

impl GbInstance {
    pub fn build(filename: &str) -> Result<Gb, String> {
        let gb_type: GbTypes;
        match load_cartridge(filename) {
            Ok((mbc, gb_mode)) => {
                gb_type = if gb_mode == GbMode::CgbMode {
                    GbTypes::Cgb
                } else {
                    GbTypes::Dmg
                };
                Ok(Gb::new(mbc, gb_mode, gb_type))
            }
            Err(e) => Err(format!("Error loading ROM: {e}")),
        }
    }

    pub fn build_from_bytes(rom_data: Vec<u8>) -> Result<Gb, String> {
        use crate::cartridge::load_cartridge_from_bytes;
        let gb_type: GbTypes;
        match load_cartridge_from_bytes(rom_data) {
            Ok((mbc, gb_mode)) => {
                gb_type = if gb_mode == GbMode::CgbMode {
                    GbTypes::Cgb
                } else {
                    GbTypes::Dmg
                };
                Ok(Gb::new(mbc, gb_mode, gb_type))
            }
            Err(e) => Err(format!("Error parsing ROM bytes: {e}")),
        }
    }
}
