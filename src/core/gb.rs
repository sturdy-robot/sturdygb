use crate::core::cpu::CPU;
use crate::core::cartridge::Cartridge;

pub struct GB {
    cpu: CPU,
    cartridge: Cartridge,
    is_cgb: bool,
}


impl GB {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            cpu: CPU::new(),
            cartridge: cartridge,
            is_cgb: is_cgb,
        }
    }
}
