use crate::core::registers::{Registers, CPUFlags};
use crate::core::mmu::MMU;

pub struct CPU {
    reg: Registers,
    mmu: MMU,
    halted: bool,
    
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            mmu: MMU::new(),
            halted: true,
        }
    }
}