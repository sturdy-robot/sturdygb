use crate::Cartridge;
use crate::core::registers::{Registers, CPUFlags};
use crate::core::mmu::MMU;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    pub halted: bool,
    pub cycles: u8,
}

impl CPU {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            reg: Registers::new(),
            mmu: MMU::new(cartridge),
            halted: false,
            cycles: 0,
        }
    }

    fn fetch_instruction(&mut self) -> u8 {
        let instruction = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        instruction
    }

    pub fn decode(&mut self) {
        let instruction: u8 = self.fetch_instruction();
        match instruction {
            0x00 => self.nop(),
            0x01 => self.ld_bc_nn(),
            0x02 => self.ld_bc_a(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_n(),
            0x07 => self.rlca(),
            0x08 => self.ld_nn_sp(),
            0x09 => self.add_hl_bc(),
            0x0A => self.ld_a_bc(),
            0x0B => self.dec_bc(),
            0x0C => self.inc_c(),
            0x0D => self.dec_c(),
            0x0E => self.ld_c_n(),
            0x0F => self.rrca(),
            0x10 => self.stop(),
            0x11 => self.ld_de_nn(),
            0x12 => self.ld_de_a(),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x16 => self.ld_d_n(),
            0x17 => self.rla(),
            0x18 => (()),
            0x19 => self.add_hl_de(),
            0x1A => self.ld_a_de(),
            0x1B => self.dec_de(),
            0x1C => self.inc_e(),
            0x1D => self.dec_e(),
            0x1E => self.ld_e_n(),
            0x1F => self.rra(),
            0x20 => self.jr_nz_n(),
            0x21 => self.ld_hl_nn(),
            0x22 => (()),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_n(),
            0x27 => self.daa(),
            0x28 => self.jr_z_n(),
            0x29 => self.add_hl_hl(),
            0x2A => (()),
            0x2B => self.dec_hl(),
            0x2C => self.inc_l(),
            0x2D => self.dec_l(),
            0x2E => self.ld_l_n(),
            0x2F => self.cpl(),
            0x30 => (()),
            0x31 => (()),
            0x32 => (()),
            0x33 => (()),
            0x34 => (()),
            0x35 => (()),
            0x36 => (()),
            0x37 => (()),
            0x38 => (()),
            0x39 => (()),
            0x3A => (()),
            0x3B => (()),
            0x3C => (()),
            0x3D => (()),
            0x3E => (()),
            0x3F => (()),
            0x40 => (()),
            0x41 => (()),
            0x42 => (()),
            0x43 => (()),
            0x44 => (()),
            0x45 => (()),
            0x46 => (()),
            0x47 => (()),
            0x48 => (()),
            0x49 => (()),
            0x4A => (()),
            0x4B => (()),
            0x4C => (()),
            0x4D => (()),
            0x4E => (()),
            0x4F => (()),
            0x50 => (()),
            0x51 => (()),
            0x52 => (()),
            0x53 => (()),
            0x54 => (()),
            0x55 => (()),
            0x56 => (()),
            0x57 => (()),
            0x58 => (()),
            0x59 => (()),
            0x5A => (()),
            0x5B => (()),
            0x5C => (()),
            0x5D => (()),
            0x5E => (()),
            0x5F => (()),
            0x60 => (()),
            0x61 => (()),
            0x62 => (()),
            0x63 => (()),
            0x64 => (()),
            0x65 => (()),
            0x66 => (()),
            0x67 => (()),
            0x68 => (()),
            0x69 => (()),
            0x6A => (()),
            0x6B => (()),
            0x6C => (()),
            0x6D => (()),
            0x6E => (()),
            0x6F => (()),
            0x70 => (()),
            0x71 => (()),
            0x72 => (()),
            0x73 => (()),
            0x74 => (()),
            0x75 => (()),
            0x76 => (()),
            0x77 => (()),
            0x78 => (()),
            0x79 => (()),
            0x7A => (()),
            0x7B => (()),
            0x7C => (()),
            0x7D => (()),
            0x7E => (()),
            0x7F => (()),
            0x80 => (()),
            0x81 => (()),
            0x82 => (()),
            0x83 => (()),
            0x84 => (()),
            0x85 => (()),
            0x86 => (()),
            0x87 => (()),
            0x88 => (()),
            0x89 => (()),
            0x8A => (()),
            0x8B => (()),
            0x8C => (()),
            0x8D => (()),
            0x8E => (()),
            0x8F => (()),
            0x90 => (()),
            0x91 => (()),
            0x92 => (()),
            0x93 => (()),
            0x94 => (()),
            0x95 => (()),
            0x96 => (()),
            0x97 => (()),
            0x98 => (()),
            0x99 => (()),
            0x9A => (()),
            0x9B => (()),
            0x9C => (()),
            0x9D => (()),
            0x9E => (()),
            0x9F => (()),
            0xA0 => (()),
            0xA1 => (()),
            0xA2 => (()),
            0xA3 => (()),
            0xA4 => (()),
            0xA5 => (()),
            0xA6 => (()),
            0xA7 => (()),
            0xA8 => (()),
            0xA9 => (()),
            0xAA => (()),
            0xAB => (()),
            0xAC => (()),
            0xAD => (()),
            0xAE => (()),
            0xAF => (()),
            0xB0 => (()),
            0xB1 => (()),
            0xB2 => (()),
            0xB3 => (()),
            0xB4 => (()),
            0xB5 => (()),
            0xB6 => (()),
            0xB7 => (()),
            0xB8 => (()),
            0xB9 => (()),
            0xBA => (()),
            0xBB => (()),
            0xBC => (()),
            0xBD => (()),
            0xBE => (()),
            0xBF => (()),
            0xC0 => (()),
            0xC1 => (()),
            0xC2 => (()),
            0xC3 => (()),
            0xC4 => (()),
            0xC5 => (()),
            0xC6 => (()),
            0xC7 => (()),
            0xC8 => (()),
            0xC9 => (()),
            0xCA => (()),
            0xCB => (()),
            0xCC => (()),
            0xCD => (()),
            0xCE => (()),
            0xCF => (()),
            0xD0 => (()),
            0xD1 => (()),
            0xD2 => (()),
            0xD3 => (()),
            0xD4 => (()),
            0xD5 => (()),
            0xD6 => (()),
            0xD7 => (()),
            0xD8 => (()),
            0xD9 => (()),
            0xDA => (()),
            0xDB => (()),
            0xDC => (()),
            0xDD => (()),
            0xDE => (()),
            0xDF => (()),
            0xE0 => (()),
            0xE1 => (()),
            0xE2 => (()),
            0xE3 => (()),
            0xE4 => (()),
            0xE5 => (()),
            0xE6 => (()),
            0xE7 => (()),
            0xE8 => (()),
            0xE9 => (()),
            0xEA => (()),
            0xEB => (()),
            0xEC => (()),
            0xED => (()),
            0xEE => (()),
            0xEF => (()),
            0xF0 => (()),
            0xF1 => (()),
            0xF2 => (()),
            0xF3 => (()),
            0xF4 => (()),
            0xF5 => (()),
            0xF6 => (()),
            0xF7 => (()),
            0xF8 => (()),
            0xF9 => (()),
            0xFA => (()),
            0xFB => (()),
            0xFC => (()),
            0xFD => (()),
            0xFE => (()),
            0xFF => (()),
            _ => self.not_supported_instruction(instruction),
        };
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn nop(&mut self) {
        println!("NOP");
    }

    fn ld_bc_nn(&mut self) {
        let value = self.mmu.read_word(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(2);
        self.reg.set_bc(value);
    }

    fn ld_bc_a(&mut self) {
        self.mmu.write_byte(self.reg.bc(), self.reg.a)
    }

    fn inc_bc(&mut self) {
        self.reg.set_bc(self.reg.bc().wrapping_add(1));
    }

    fn inc_b(&mut self) {
        self.reg.b = self.reg.b.wrapping_add(1) & 0xFF;
        self.reg.set_f(CPUFlags::Z, self.reg.b == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, self.reg.b == 0);
    }

    fn dec_b(&mut self) {
        self.reg.b = self.reg.b.wrapping_sub(1) & 0xFF;
        self.reg.set_f(CPUFlags::Z, self.reg.b == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, self.reg.b == 0);
    }

    fn ld_b_n(&mut self) {
        self.reg.b = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn rlca(&mut self) {
        self.reg.a = ((self.reg.a << 1) & 0xFF) | (self.reg.a >> 7);
        self.reg.set_f(CPUFlags::Z, false);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, self.reg.a > 0x7F); 
    }

    fn ld_nn_sp(&mut self) {
        let value = self.mmu.read_word(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(2);
        self.mmu.write_word(value, self.reg.sp);
    }

    fn add_hl_bc(&mut self) {
        let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.bc());
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.set_hl(value);
    }

    fn ld_a_bc(&mut self) {
        self.reg.a = self.mmu.read_byte(self.reg.bc());
    }

    fn dec_bc(&mut self) {
        self.reg.set_bc(self.reg.bc().wrapping_sub(1));
    }

    fn inc_c(&mut self) {
        self.reg.c = self.reg.c.wrapping_add(1);
        self.reg.set_f(CPUFlags::Z, self.reg.c == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.c & 0xF) == 0);
    }

    fn dec_c(&mut self) {
        self.reg.c = self.reg.c.wrapping_sub(1);
        self.reg.set_f(CPUFlags::Z, self.reg.c == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.c & 0xF) == 0);
    }

    fn ld_c_n(&mut self) {
        self.reg.c = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn rrca(&mut self) {
        self.reg.a = (self.reg.a >> 1) | ((self.reg.a & 1) << 7);
        self.reg.set_f(CPUFlags::Z, false);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, self.reg.a > 0x7F);
    }

    fn stop(&mut self) {
        
    }

    fn ld_de_nn(&mut self) {
        let value = self.mmu.read_word(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(2);
        self.reg.set_de(value);
    }

    fn ld_de_a(&mut self) {
        self.mmu.write_byte(self.reg.de(), self.reg.a);
    }

    fn inc_de(&mut self) {
        self.reg.set_de(self.reg.de().wrapping_add(1));
    }

    fn inc_d(&mut self) {
        self.reg.d = self.reg.d.wrapping_add(1);
        self.reg.set_f(CPUFlags::Z, self.reg.d == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.d & 0xF) == 0);
    }

    fn dec_d(&mut self) {
        self.reg.d = self.reg.d.wrapping_sub(1);
        self.reg.set_f(CPUFlags::Z, self.reg.d == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.d & 0xF) == 0);
    }

    fn ld_d_n(&mut self) {
        self.reg.d = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn rla(&mut self) {

    }

    fn jn_n(&mut self) {

    }

    fn add_hl_de(&mut self) {
        let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.de());
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.set_hl(value);
    }

    fn ld_a_de(&mut self) {
        self.reg.a = self.mmu.read_byte(self.reg.de());
    }

    fn dec_de(&mut self) {
        self.reg.set_de(self.reg.de().wrapping_sub(1));
    }

    fn inc_e(&mut self) {
        self.reg.e = self.reg.e.wrapping_add(1);
        self.reg.set_f(CPUFlags::Z, self.reg.e == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.e & 0xF) == 0);
    }

    fn dec_e(&mut self) {
        self.reg.e = self.reg.e.wrapping_sub(1);
        self.reg.set_f(CPUFlags::Z, self.reg.e == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.e & 0xF) == 0);
    }

    fn ld_e_n(&mut self) {
        self.reg.e = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn rra(&mut self) {

    }

    fn jr_nz_n(&mut self) {

    }

    fn ld_hl_nn(&mut self) {
        let value = self.mmu.read_word(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(2);
        self.reg.set_hl(value);
    }

    fn ldi_hl_a(&mut self) {

    }

    fn inc_hl(&mut self) {
        self.reg.set_hl(self.reg.hl().wrapping_sub(1));
    }

    fn inc_h(&mut self) {
        self.reg.h = self.reg.h.wrapping_add(1);
        self.reg.set_f(CPUFlags::Z, self.reg.h == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.h & 0xF) == 0);
    }

    fn dec_h(&mut self) {
        self.reg.h = self.reg.h.wrapping_sub(1);
        self.reg.set_f(CPUFlags::Z, self.reg.h == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.h & 0xF) == 0);
    }

    fn ld_h_n(&mut self) {
        self.reg.h = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn daa(&mut self) {

    }

    fn jr_z_n(&mut self) {

    }

    fn add_hl_hl(&mut self) {
        let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.hl());
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > 0x7FF);
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.set_hl(value);
    }

    fn ldi_a_hl(&mut self) {

    }

    fn dec_hl(&mut self) {
        self.reg.set_hl(self.reg.hl().wrapping_sub(1));
    }

    fn inc_l(&mut self) {
        self.reg.l = self.reg.l.wrapping_add(1);
        self.reg.set_f(CPUFlags::Z, self.reg.l == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.l & 0xF) == 0);
    }

    fn dec_l(&mut self) {
        self.reg.l = self.reg.l.wrapping_sub(1);
        self.reg.set_f(CPUFlags::Z, self.reg.l == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.l & 0xF) == 0);
    }
    
    fn ld_l_n(&mut self) {
        self.reg.l = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn ld_a_n(&mut self) {
        self.reg.a = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn cpl(&mut self) {
        
    }

    
    fn not_supported_instruction(&mut self, instruction: u8) {
        println!("Instruction not supported, {:x}", instruction);

    }
}


#[cfg(test)]
mod test {
    use crate::core::cartridge::Cartridge;
    use super::CPU;

    
}