mod core;
use crate::core::gb::{Gb, GbTypes};
use crate::core::mbc::{load_cartridge, GbMode};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = if args.len() == 1 { "roms/cpu_instrs.gb" } else { &args[1] };
    match load_cartridge(filename) {
        Ok((mbc, gb_mode)) => {
            let gb_type: GbTypes = if gb_mode == GbMode::CgbMode { GbTypes::Cgb } else { GbTypes::Dmg };
            let mut gb = Gb::new(mbc, gb_mode, gb_type);
            gb.run();
        }
        Err(e) => println!("Error loading ROM: {}", e),
    };
}
