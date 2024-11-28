// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use notan::prelude::*;
use std::env;
use sturdygb_core::prelude::GbInstance;

#[notan_main]
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename: &str = if args.len() == 1 {
        "../roms/cpu_instrs.gb"
    } else {
        &args[1]
    };
    let mut gb = match GbInstance::build(filename) {
        Ok(gb) => gb,
        Err(_) => panic!("Unable to get a valid instance of a GameBoy game."),
    };
    notan::init().build()
}
