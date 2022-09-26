use crate::{Cartridge};
use crate::core::registers::{Registers, CPUFlags};
use crate::core::mmu::MMU;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    pub is_halted: bool,
    pub cycles: u8,
    pub is_cgb: bool,
}

impl CPU {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            reg: Registers::new(&is_cgb),
            mmu: MMU::new(cartridge),
            is_halted: false,
            cycles: 0,
            is_cgb,
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
            0x18 => self.jr_n(),
            0x19 => self.add_hl_de(),
            0x1A => self.ld_a_de(),
            0x1B => self.dec_de(),
            0x1C => self.inc_e(),
            0x1D => self.dec_e(),
            0x1E => self.ld_e_n(),
            0x1F => self.rra(),
            0x20 => self.jr_nz_n(),
            0x21 => self.ld_hl_nn(),
            0x22 => self.ldi_hl_a(),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_n(),
            0x27 => self.daa(),
            0x28 => self.jr_z_n(),
            0x29 => self.add_hl_hl(),
            0x2A => self.ldi_a_hl(),
            0x2B => self.dec_hl(),
            0x2C => self.inc_l(),
            0x2D => self.dec_l(),
            0x2E => self.ld_l_n(),
            0x2F => self.cpl(),
            0x30 => self.jr_nc_n(),
            0x31 => self.ld_sp_nn(),
            0x32 => self.ldd_hl_a(),
            0x33 => self.inc_sp(),
            0x34 => self.inc_hl_(),
            0x35 => self.dec_hl_(),
            0x36 => self.ldd_hl_n(),
            0x37 => self.scf(),
            0x38 => self.jr_c_n(),
            0x39 => self.add_hl_sp(),
            0x3A => self.ldd_a_hl(),
            0x3B => self.dec_sp(),
            0x3C => self.inc_a(),
            0x3D => self.dec_a(),
            0x3E => self.ld_a_n(),
            0x3F => self.ccf(),
            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x46 => self.ld_b_hl(),
            0x47 => self.ld_b_a(),
            0x48 => self.ld_c_b(),
            0x49 => self.ld_c_c(),
            0x4A => self.ld_c_d(),
            0x4B => self.ld_c_e(),
            0x4C => self.ld_c_h(),
            0x4D => self.ld_c_l(),
            0x4E => self.ld_c_hl(),
            0x4F => self.ld_c_a(),
            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.ld_d_d(),
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x56 => self.ld_d_hl(),
            0x57 => self.ld_d_a(),
            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5A => self.ld_e_d(),
            0x5B => self.ld_e_e(),
            0x5C => self.ld_e_h(),
            0x5D => self.ld_e_l(),
            0x5E => self.ld_e_hl(),
            0x5F => self.ld_e_a(),
            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => self.ld_h_h(),
            0x65 => self.ld_h_l(),
            0x66 => self.ld_h_hl(),
            0x67 => self.ld_h_a(),
            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6A => self.ld_l_d(),
            0x6B => self.ld_l_e(),
            0x6C => self.ld_l_h(),
            0x6D => self.ld_l_l(),
            0x6E => self.ld_l_hl(),
            0x6F => self.ld_l_a(),
            0x70 => self.ld_hl_b(),
            0x71 => self.ld_hl_c(),
            0x72 => self.ld_hl_d(),
            0x73 => self.ld_hl_e(),
            0x74 => self.ld_hl_h(),
            0x75 => self.ld_hl_l(),
            0x76 => self.halt(),
            0x77 => self.ld_hl_a(),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7E => self.ld_a_hl(),
            0x7F => self.ld_a_a(),
            0x80 => self.add_a_b(),
            0x81 => self.add_a_c(),
            0x82 => self.add_a_d(),
            0x83 => self.add_a_e(),
            0x84 => self.add_a_h(),
            0x85 => self.add_a_l(),
            0x86 => self.add_a_hl(),
            0x87 => self.add_a_a(),
            0x88 => self.adc_a_b(),
            0x89 => self.adc_a_c(),
            0x8A => self.adc_a_d(),
            0x8B => self.adc_a_e(),
            0x8C => self.adc_a_h(),
            0x8D => self.adc_a_l(),
            0x8E => self.adc_a_hl(),
            0x8F => self.adc_a_a(),
            0x90 => self.sub_a_b(),
            0x91 => self.sub_a_c(),
            0x92 => self.sub_a_d(),
            0x93 => self.sub_a_e(),
            0x94 => self.sub_a_h(),
            0x95 => self.sub_a_l(),
            0x96 => self.sub_a_hl(),
            0x97 => self.sub_a_a(),
            0x98 => self.sbc_a_b(),
            0x99 => self.sbc_a_c(),
            0x9A => self.sbc_a_d(),
            0x9B => self.sbc_a_e(),
            0x9C => self.sbc_a_h(),
            0x9D => self.sbc_a_l(),
            0x9E => self.sbc_a_hl(),
            0x9F => self.sbc_a_a(),
            0xA0 => self.and_a_b(),
            0xA1 => self.and_a_c(),
            0xA2 => self.and_a_d(),
            0xA3 => self.and_a_e(),
            0xA4 => self.and_a_h(),
            0xA5 => self.and_a_l(),
            0xA6 => self.and_a_hl(),
            0xA7 => self.and_a_a(),
            0xA8 => self.xor_a_b(),
            0xA9 => self.xor_a_c(),
            0xAA => self.xor_a_d(),
            0xAB => self.xor_a_e(),
            0xAC => self.xor_a_h(),
            0xAD => self.xor_a_l(),
            0xAE => self.xor_a_hl(),
            0xAF => self.xor_a_a(),
            0xB0 => self.or_a_b(),
            0xB1 => self.or_a_c(),
            0xB2 => self.or_a_d(),
            0xB3 => self.or_a_e(),
            0xB4 => self.or_a_h(),
            0xB5 => self.or_a_l(),
            0xB6 => self.or_a_hl(),
            0xB7 => self.or_a_a(),
            0xB8 => self.cp_a_b(),
            0xB9 => self.cp_a_c(),
            0xBA => self.cp_a_d(),
            0xBB => self.cp_a_e(),
            0xBC => self.cp_a_h(),
            0xBD => self.cp_a_l(),
            0xBE => self.cp_a_hl(),
            0xBF => self.cp_a_a(),
            0xC0 => self.ret_nz(),
            0xC1 => self.pop_bc(),
            0xC2 => self.jp_nz_nn(),
            0xC3 => self.jp_nn(),
            0xC4 => self.call_nz_nn(),
            0xC5 => self.push_bc(),
            0xC6 => self.add_a_n(),
            0xC7 => self.rst_00h(),
            0xC8 => self.ret_z(),
            0xC9 => self.ret(),
            0xCA => self.jp_z_nn(),
            0xCB => self.decode_cb(),
            0xCC => self.call_z_nn(),
            0xCD => self.call_nn(),
            0xCE => self.add_a_n(),
            0xCF => self.rst_08h(),
            0xD0 => self.ret_nc(),
            0xD1 => self.pop_de(),
            0xD2 => self.jp_nc_nn(),
            0xD3 => self.not_supported_instruction(instruction),
            0xD4 => self.call_nc_nn(),
            0xD5 => self.push_de(),
            0xD6 => self.sub_a_n(),
            0xD7 => self.rst_10h(),
            0xD8 => self.ret_c(),
            0xD9 => self.reti(),
            0xDA => self.jp_c_nn(),
            0xDB => self.not_supported_instruction(instruction),
            0xDC => self.call_c_nn(),
            0xDD => self.not_supported_instruction(instruction),
            0xDE => self.sbc_a_nn(),
            0xDF => self.rst_18h(),
            0xE0 => self.ld_ff00_a(),
            0xE1 => self.pop_hl(),
            0xE2 => self.ld_ff00_c(),
            0xE3 => self.not_supported_instruction(instruction),
            0xE4 => self.not_supported_instruction(instruction),
            0xE5 => self.push_hl(),
            0xE6 => self.and_a_n(),
            0xE7 => self.rst_20h(),
            0xE8 => self.add_sp_i(),
            0xE9 => self.jp_hl(),
            0xEA => self.ld_nn_a(),
            0xEB => self.not_supported_instruction(instruction),
            0xEC => self.not_supported_instruction(instruction),
            0xED => self.not_supported_instruction(instruction),
            0xEE => self.xor_a_n(),
            0xEF => self.rst_28h(),
            0xF0 => self.ld_a_ff0(),
            0xF1 => self.pop_af(),
            0xF2 => self.ld_a_ff00c(),
            0xF3 => self.di(),
            0xF4 => self.not_supported_instruction(instruction),
            0xF5 => self.push_af(),
            0xF6 => self.or_a_n(),
            0xF7 => self.rst_30h(),
            0xF8 => self.ld_hl_spi8(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_nn(),
            0xFB => self.ei(),
            0xFC => self.not_supported_instruction(instruction),
            0xFD => self.not_supported_instruction(instruction),
            0xFE => self.cp_a_n(),
            0xFF => self.rst_38h(),
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
        self.mmu.write_byte(self.reg.bc(), self.reg.a);
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
        if self.is_cgb {
            
        }
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

    fn jr_n(&mut self) {

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

    fn jr_nc_n(&mut self) {

    }

    fn ld_sp_nn(&mut self) {

    }

    fn ldd_hl_a(&mut self) {

    }
    
    fn inc_sp(&mut self) {
        self.reg.sp = self.reg.sp.wrapping_add(1);
    }

    fn inc_hl_(&mut self) {
        self.reg.set_hl(self.reg.hl().wrapping_add(1));
        self.reg.set_f(CPUFlags::Z, self.reg.hl() == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, self.reg.hl() == 0);
    }

    fn dec_hl_(&mut self) {
        self.reg.set_hl(self.reg.hl().wrapping_sub(1));
        self.reg.set_f(CPUFlags::Z, self.reg.hl() == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, self.reg.hl() == 0);
    }

    fn ldd_hl_n(&mut self) {
        self.mmu.write_word(self.reg.hl(), self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn scf(&mut self) {

    }

    fn jr_c_n(&mut self) {

    }

    fn add_hl_sp(&mut self) {
        let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.sp);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.set_hl(value);
    }

    fn ldd_a_hl(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.a);
    }

    fn dec_sp(&mut self) {
        self.reg.sp = self.reg.sp.wrapping_sub(1) & 0xFFFF;
    }

    fn inc_a(&mut self) {
        self.reg.a = self.reg.a.wrapping_sub(1) & 0xFF;
        self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) == 0);
    }

    fn dec_a(&mut self) {
        self.reg.a = self.reg.a.wrapping_sub(1) & 0xFF;
        self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) == 0);
    }

    fn ccf(&mut self) {

    }

    fn ld_b_b(&mut self) {
        // Do nothing
    }

    fn ld_b_c(&mut self) {
        self.reg.b = self.reg.c;
    }

    fn ld_b_d(&mut self) {
        self.reg.b = self.reg.d;
    }

    fn ld_b_e(&mut self) {
        self.reg.b = self.reg.e;
    }

    fn ld_b_h(&mut self) {
        self.reg.b = self.reg.h;
    }

    fn ld_b_l(&mut self) {
        self.reg.b = self.reg.l;
    }
    
    fn ld_b_hl(&mut self) {
        self.reg.b = self.mmu.read_byte(self.reg.hl());
    }

    fn ld_b_a(&mut self) {
        self.reg.b = self.reg.a;
    }

    fn ld_c_b(&mut self) {
        self.reg.c = self.reg.b;
    }
    
    fn ld_c_c(&mut self) {
        // Do nothing
    }
    
    fn ld_c_d(&mut self) {
        self.reg.c = self.reg.d;
    }
    
    fn ld_c_e(&mut self) {
        self.reg.c = self.reg.e;
    }
    
    fn ld_c_h(&mut self) {
        self.reg.c = self.reg.h;
    }
    
    fn ld_c_l(&mut self) {
        self.reg.c = self.reg.l;
    }
    
    fn ld_c_hl(&mut self) {
        self.reg.c = self.mmu.read_byte(self.reg.hl());
    }

    fn ld_c_a(&mut self) {
        self.reg.c = self.reg.a;
    }

    fn ld_d_b(&mut self) {
        self.reg.d = self.reg.b;
    }

    fn ld_d_c(&mut self) {
        self.reg.d = self.reg.c;
    }

    fn ld_d_d(&mut self) {
        // Do nothing
    }

    fn ld_d_e(&mut self) {
        self.reg.d = self.reg.e;
    }

    fn ld_d_h(&mut self) {
        self.reg.d = self.reg.h;
    }

    fn ld_d_l(&mut self) {
        self.reg.d = self.reg.l;
    }

    fn ld_d_hl(&mut self) {
        self.reg.d = self.mmu.read_byte(self.reg.hl());
    }

    fn ld_d_a(&mut self) {
        self.reg.d = self.reg.a;
    }

    fn ld_e_b(&mut self) {
        self.reg.e = self.reg.b;
    }

    fn ld_e_c(&mut self) {
        self.reg.e = self.reg.c;
    }

    fn ld_e_d(&mut self) {
        self.reg.e = self.reg.d;
    }

    fn ld_e_e(&mut self) {
        // Do nothing
    }

    fn ld_e_h(&mut self) {
        self.reg.e = self.reg.h;
    }

    fn ld_e_l(&mut self) {
        self.reg.e = self.reg.l;
    }

    fn ld_e_hl(&mut self) {
        self.reg.e = self.mmu.read_byte(self.reg.hl());
    }

    fn ld_e_a(&mut self) {
        self.reg.e = self.reg.a;
    }

    fn ld_h_b(&mut self) {
        self.reg.h = self.reg.b;
    }
    
    fn ld_h_c(&mut self) {
        self.reg.h = self.reg.c;
    }
    
    fn ld_h_d(&mut self) {
        self.reg.h = self.reg.d;
    }
    
    fn ld_h_e(&mut self) {
        self.reg.h = self.reg.e;
    }
    
    fn ld_h_h(&mut self) {
        // Do nothing
    }
    
    fn ld_h_l(&mut self) {
        self.reg.h = self.reg.l;
    }
    
    fn ld_h_hl(&mut self) {
        self.reg.h = self.mmu.read_byte(self.reg.hl());
    }
    
    fn ld_h_a(&mut self) {
        self.reg.h = self.reg.a;
    }
    
    fn ld_l_b(&mut self) {
        self.reg.l = self.reg.b;
    }
    
    fn ld_l_c(&mut self) {
        self.reg.l = self.reg.c;
    }
    
    fn ld_l_d(&mut self) {
        self.reg.l = self.reg.d;
    }
    
    fn ld_l_e(&mut self) {
        self.reg.l = self.reg.e;
    }
    
    fn ld_l_h(&mut self) {
        self.reg.l = self.reg.h;
    }
    
    fn ld_l_l(&mut self) {
        // Do nothing
    }
    
    fn ld_l_hl(&mut self) {
        self.reg.l = self.mmu.read_byte(self.reg.hl());
    }
    
    fn ld_l_a(&mut self) {
        self.reg.l = self.reg.a;
    }
    
    fn ld_hl_b(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.b);
    }
    
    fn ld_hl_c(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.c);
    }
    
    fn ld_hl_d(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.d);
    }
    
    fn ld_hl_e(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.e);
    }
    
    fn ld_hl_h(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.h);
    }
    
    fn ld_hl_l(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.l);
    }
    
    fn halt(&mut self) {
        self.is_halted = true;
    }
    
    fn ld_hl_a(&mut self) {
        self.mmu.write_byte(self.reg.hl(), self.reg.a);
    }
    
    fn ld_a_b(&mut self) {
        self.reg.a = self.reg.b;
    }
    
    fn ld_a_c(&mut self) {
        self.reg.a = self.reg.c;
    }
    
    fn ld_a_d(&mut self) {
        self.reg.a = self.reg.d;
    }
    
    fn ld_a_e(&mut self) {
        self.reg.a = self.reg.e;
    }
    
    fn ld_a_h(&mut self) {
        self.reg.a = self.reg.h;
    }
    
    fn ld_a_l(&mut self) {
        self.reg.a = self.reg.l;
    }
    
    fn ld_a_hl(&mut self) {
        self.reg.a = self.mmu.read_byte(self.reg.hl());
    }
    
    fn ld_a_a(&mut self) {
        // Do nothing
    }
    
    fn add_a_b(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.b);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn add_a_c(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.c);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn add_a_d(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.d);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn add_a_e(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.e);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn add_a_h(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.h);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn add_a_l(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.l);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn add_a_hl(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.mmu.read_byte(self.reg.hl()));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn add_a_a(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.a);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_b(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.b.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_c(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.c.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_d(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.d.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_e(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.e.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_h(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.h.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_l(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.l.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_hl(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.mmu.read_byte(self.reg.hl().wrapping_add(CPUFlags::C as u16)));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn adc_a_a(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.a.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_b(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.b);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_c(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.c);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_d(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.d);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_e(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.e);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_h(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.h);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_l(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.l);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_hl(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.mmu.read_byte(self.reg.hl()));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sub_a_a(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.a);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_b(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.b.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_c(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.c.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_d(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.d.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_e(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.e.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_h(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.h.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_l(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.l.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_hl(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.mmu.read_byte(self.reg.hl().wrapping_sub(CPUFlags::C as u16)));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn sbc_a_a(&mut self) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.a.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn and_a_b(&mut self) {
        let value = self.reg.a & self.reg.b;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn and_a_c(&mut self) {
        let value = self.reg.a & self.reg.c;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn and_a_d(&mut self) {
        let value = self.reg.a & self.reg.d;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn and_a_e(&mut self) {
        let value = self.reg.a & self.reg.e;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn and_a_h(&mut self) {
        let value = self.reg.a & self.reg.h;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn and_a_l(&mut self) {
        let value = self.reg.a & self.reg.l;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn and_a_hl(&mut self) {
        let value = self.reg.a & self.mmu.read_byte(self.reg.hl());
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn and_a_a(&mut self) {
        self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
    }

    fn xor_a_b(&mut self) {
        let value = self.reg.a ^ self.reg.b;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn xor_a_c(&mut self) {
        let value = self.reg.a ^ self.reg.c;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn xor_a_d(&mut self) {
        let value = self.reg.a ^ self.reg.d;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn xor_a_e(&mut self) {
        let value = self.reg.a ^ self.reg.e;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn xor_a_h(&mut self) {
        let value = self.reg.a ^ self.reg.h;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn xor_a_l(&mut self) {
        let value = self.reg.a ^ self.reg.l;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn xor_a_hl(&mut self) {
        let value = self.reg.a ^ self.mmu.read_byte(self.reg.hl());
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn xor_a_a(&mut self) {
        let value = self.reg.a ^ self.reg.a;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_b(&mut self) {
        let value = self.reg.a | self.reg.b;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_c(&mut self) {
        let value = self.reg.a | self.reg.c;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_d(&mut self) {
        let value = self.reg.a | self.reg.d;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_e(&mut self) {
        let value = self.reg.a | self.reg.e;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_h(&mut self) {
        let value = self.reg.a | self.reg.h;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_l(&mut self) {
        let value = self.reg.a | self.reg.l;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_hl(&mut self) {
        let value = self.reg.a | self.mmu.read_byte(self.reg.hl());
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn or_a_a(&mut self) {
        let value = self.reg.a | self.reg.a;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn cp_a_b(&mut self) {
        
    }

    fn cp_a_c(&mut self) {

    }

    fn cp_a_d(&mut self) {

    }

    fn cp_a_e(&mut self) {

    }


    fn cp_a_h(&mut self) {

    }


    fn cp_a_l(&mut self) {

    }


    fn cp_a_hl(&mut self) {

    }


    fn cp_a_a(&mut self) {

    }


    fn ret_nz(&mut self) {

    }


    fn pop_bc(&mut self) {

    }


    fn jp_nz_nn(&mut self) {

    }


    fn jp_nn(&mut self) {

    }


    fn call_nz_nn(&mut self) {

    }


    fn push_bc(&mut self) {

    }


    fn add_a_n(&mut self) {

    }


    fn rst_00h(&mut self) {

    }


    fn ret_z(&mut self) {

    }


    fn ret(&mut self) {

    }


    fn jp_z_nn(&mut self) {

    }


    fn decode_cb(&mut self) {

    }


    fn call_z_nn(&mut self) {

    }


    fn call_nn(&mut self) {

    }


    fn rst_08h(&mut self) {

    }


    fn ret_nc(&mut self) {

    }


    fn pop_de(&mut self) {

    }


    fn jp_nc_nn(&mut self) {

    }


    fn call_nc_nn(&mut self) {

    }


    fn push_de(&mut self) {

    }


    fn sub_a_n(&mut self) {

    }


    fn rst_10h(&mut self) {

    }

    fn ret_c(&mut self) {

    }
    
    fn reti(&mut self) {

    }

    fn jp_c_nn(&mut self) {

    }

    fn call_c_nn(&mut self) {

    }


    fn sbc_a_nn(&mut self) {

    }


    fn rst_18h(&mut self) {

    }


    fn ld_ff00_a(&mut self) {

    }


    fn pop_hl(&mut self) {

    }


    fn ld_ff00_c(&mut self) {

    }


    fn push_hl(&mut self) {

    }


    fn and_a_n(&mut self) {

    }


    fn rst_20h(&mut self) {

    }


    fn add_sp_i(&mut self) {

    }


    fn jp_hl(&mut self) {

    }


    fn ld_nn_a(&mut self) {

    }


    fn xor_a_n(&mut self) {
        let value = self.reg.a | self.mmu.read_byte(self.reg.pc);
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }


    fn rst_28h(&mut self) {

    }


    fn ld_a_ff0(&mut self) {

    }


    fn pop_af(&mut self) {

    }


    fn ld_a_ff00c(&mut self) {

    }


    fn di(&mut self) {

    }


    fn push_af(&mut self) {

    }


    fn or_a_n(&mut self) {

    }


    fn rst_30h(&mut self) {

    }


    fn ld_hl_spi8(&mut self) {

    }


    fn ld_sp_hl(&mut self) {

    }

    fn ld_a_nn(&mut self) {
        self.reg.a = self.mmu.read_byte(self.reg.pc);
    }

    fn ei(&mut self) {

    }

    fn cp_a_n(&mut self) {

    }

    fn rst_38h(&mut self) {

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