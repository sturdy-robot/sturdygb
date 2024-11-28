// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT
use super::cartridge::{load_cartridge, GbMode};
use super::gb::{Gb, GbTypes};

pub struct GbInstance;

impl GbInstance {
    pub fn build(filename: &str) -> Result<Gb, ()> {
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
            Err(e) => Err(println!("Error loading ROM: {}", e)),
        }
    }
}
