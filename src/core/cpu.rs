use crate::Cartridge;
use crate::core::registers::{Registers, CPUFlags};
use crate::core::mmu::MMU;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    pub halted: bool,
}

impl CPU {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            reg: Registers::new(),
            mmu: MMU::new(cartridge),
            halted: false,
        }
    }

    pub fn decode(&mut self) {
        let mut pc: u16 = self.reg.pc as u16;
        let instruction: u8 = self.mmu.read_byte(pc);
        let opcode = match instruction {
            0x00 => self.in_nop(),
            0x01 => self.in_ld_bc(),
            0x02 => self.in_ld_bc_a(),
            _ => self.not_supported_instruction(),
        };
        self.reg.pc += 1;
    }

    fn in_nop(&mut self) {
        
    }

    fn in_ld_bc(&mut self) {
        self.reg.c = self.mmu.read_byte(self.reg.pc);
        self.reg.b = self.mmu.read_byte(self.reg.pc.wrapping_add(1) & 0xFFFF);
        self.reg.pc = (self.reg.pc.wrapping_add(2)) & 0xFFFF;
    }

    fn in_ld_bc_a(&mut self) {
        let value = self.mmu.read_byte(self.reg.pc);
        self.mmu.write_byte(self.reg.bc(), value);
    }

    fn not_supported_instruction(&mut self) {
        println!("Instruction not supported, {instruction}", instruction=self.reg.pc);
    }
}



