// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use std::env;

use crate::core::gb::{Gb, GbTypes};
use crate::core::mbc::{GbMode, load_cartridge};
use crate::ui::Renderer;

mod core;
mod ui;

fn get_gb_instance(filename: &str) -> Result<Gb, ()> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = if args.len() == 1 {
        "roms/cpu_instrs.gb"
    } else {
        &args[1]
    };
    let mut gb = match get_gb_instance(filename) {
        Ok(mut gb) => gb,
        _ => panic!("Unable to open GameBoy game!"),
    };

    gb.headless_run();
}
