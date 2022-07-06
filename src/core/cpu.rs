use crate::core::registers::{Registers, CPUFlags};
use crate::core::mmu::MMU;

pub struct CPU {
    reg: Registers,
    mmu: MMU,
    halted: bool,
    
}