#![allow(dead_code)]
#![allow(unused_variables)]

use crate::core::cartridge::Cartridge;
use crate::core::gb::GB;

mod core;

fn init_gb(cartridge: Cartridge) -> GB {
    let is_cgb = cartridge.is_cgb_only();
    GB::new(cartridge, is_cgb)
}

fn load_cartridge(filename: String) -> Cartridge {
    Cartridge::new(&filename)
}

fn main() {
    let cartridge: Cartridge =
        load_cartridge("roms/gb-test-roms/cpu_instrs/cpu_instrs.gb".to_string());
    let mut gb: GB = init_gb(cartridge);
    gb.run();
}
