use crate::core::cpu::CPU;
use crate::core::cartridge::Cartridge;
use crate::core::ppu::PPU;

/// GameBoy emulator implementation
pub struct GB {
    pub cpu: CPU,
}


impl GB {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            cpu: CPU::new(cartridge, is_cgb),
        }
    }

    pub fn run(&mut self) {
        self.cpu.execute();
    }
}
