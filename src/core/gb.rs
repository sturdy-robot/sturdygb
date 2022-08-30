use crate::core::cpu::CPU;
use crate::core::cartridge::Cartridge;

/// GameBoy emulator implementation
pub struct GB {
    pub cpu: CPU,
    pub is_cgb: bool,
}


impl GB {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            cpu: CPU::new(cartridge),
            is_cgb: is_cgb,
        }
    }

    pub fn run(&mut self) {
        while !self.cpu.halted && usize::from(self.cpu.reg.pc) < 32768 {
            self.cpu.decode()
        }
    }
}
