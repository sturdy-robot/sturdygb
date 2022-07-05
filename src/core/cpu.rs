use crate::core::registers::{Registers, CPUFlags};
use crate::core::mmu::MMU;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
pub struct CPU {
    reg: Registers,
    mmu: MMU,
    halted: bool,
    
}