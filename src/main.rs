mod core;
use crate::core::gb::{Gb, GbTypes};
use crate::core::mbc::{get_mbc, CartridgeHeader, GbMode, Mbc};
use std::env;
use std::fs;

fn load_cartridge(filename: &str) -> Result<(Box<dyn Mbc>, GbMode), &str> {
    let rom_data = fs::read(filename).expect("Unable to read file contents");
    match CartridgeHeader::new(&rom_data) {
        Ok(header) => Ok(get_mbc(rom_data, header)),
        Err(f) => return Err(f),
    }
}

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
