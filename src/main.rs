mod core;
use crate::core::gb::{Gb, GbTypes};
use crate::core::mbc::{GbMode, load_cartridge};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str;
    if args.len() == 1 {
        filename = "roms/cpu_instrs.gb";
    } else {
        filename = &args[1];
    }
    match load_cartridge(filename) {
        Ok((mbc, gb_mode)) => {
            let gb_type: GbTypes;
            if gb_mode == GbMode::CgbMode {
                gb_type = GbTypes::Cgb;
            } else {
                gb_type = GbTypes::Dmg;
            }
            let mut gb = Gb::new(mbc, gb_mode, gb_type);
            gb.run();
        }
        Err(e) => println!("Error loading ROM: {}", e),
    };
}
