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
        match instruction {
            0x00 => self.nop(),
            0x01 => self.ld_bc(),
            0x02 => self.ld_bc_a(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            _ => self.not_supported_instruction(instruction),
        };
        self.reg.pc += 1;
    }

    fn nop(&mut self) {
        println!("NOP");
    }

    fn ld_bc(&mut self) {
        self.reg.c = self.mmu.read_byte(self.reg.pc);
        self.reg.b = self.mmu.read_byte(self.reg.pc.wrapping_add(1) & 0xFFFF);
        self.reg.pc = (self.reg.pc.wrapping_add(2)) & 0xFFFF;
        println!("LD BC, NN");
    }

    fn ld_bc_a(&mut self) {
        let value = self.mmu.read_byte(self.reg.pc);
        self.mmu.write_byte(self.reg.bc(), value);
        println!("LD BC, A");
    }

    fn inc_bc(&mut self) {
        let value = self.reg.set_bc(self.reg.bc().wrapping_add(1));
        println!("INC BC");
    }

    fn inc_b(&mut self) {
        self.reg.b = self.reg.b.wrapping_add(1) & 0xFF;
        self.reg.set_f(CPUFlags::Z, self.reg.b == 0);
        self.reg.set_f(CPUFlags::H, (self.reg.b & 0x0F).wrapping_add(1) > 0x0F);
        self.reg.set_f(CPUFlags::N, false);
        println!("INC B");
    }

    fn not_supported_instruction(&mut self, instruction: u8) {
        println!("Instruction not supported, {:x}", instruction);
    }
}
