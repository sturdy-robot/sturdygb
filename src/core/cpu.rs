use crate::Cartridge;
use crate::core::registers::{Registers, CPUFlags};
use crate::core::mmu::MMU;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    pub halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            mmu: MMU::new(),
            halted: false,
        }
    }

    pub fn decode(&mut self, cartridge: &mut Cartridge) {
        let mut pc: usize = self.reg.pc as usize;
        let instruction: u16 = cartridge.rom_data[pc] as u16;
        let opcode = match instruction {
            0x00 => instruction_nop(self),
            _ => not_supported_instruction(self),
        };
    }
}

pub fn instruction_nop(cpu: &mut CPU) -> u8 {
    cpu.reg.pc += 1;
    println!("NOP instruction");
    1
}

pub fn not_supported_instruction(cpu: &mut CPU) -> u8 {
    cpu.reg.pc += 1;
    println!("Instruction not supported, {instruction}", instruction=cpu.reg.pc);
    0
}