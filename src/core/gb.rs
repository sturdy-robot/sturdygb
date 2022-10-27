use crate::core::cpu::Cpu;
use crate::core::cartridge::Cartridge;

/// GameBoy emulator implementation
pub struct GB {
    pub cpu: Cpu,
}


impl GB {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            cpu: Cpu::new(cartridge, is_cgb),
        }
    }

    pub fn run(&mut self) {
        self.cpu.execute();
    }
}
