use crate::core::cpu::CPU;
use crate::core::cartridge::Cartridge;
use crate::core::gpu::GPU;

/// GameBoy emulator implementation
pub struct GB {
    pub cpu: CPU,
    pub gpu: GPU,
}


impl GB {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            cpu: CPU::new(cartridge, is_cgb),
            gpu: GPU::new(),
        }
    }

    pub fn run(&mut self) {
        while !self.cpu.halted && usize::from(self.cpu.reg.pc) < 0x8000 {
            self.cpu.decode()
        }
    }
}
