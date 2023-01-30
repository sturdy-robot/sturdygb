// Decode instructions
// The implementation here is based on the SameBoy implementation

use crate::core::gb::Gb;
use paste::paste;

impl Gb {
    pub fn decode(&mut self) {
        let opcode = self.cpu.current_instruction;
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_rr_d16(opcode),
            0x02 => self.ld_rr_a(opcode),
            0x03 => self.inc_rr(opcode),
            0x04 => self.inc_hr(opcode),
            0x05 => self.dec_hr(opcode),
            0x06 => self.ld_hr_d8(opcode),
            0x07 => self.rlca(),
            0x08 => self.ld_da16_sp(),
            0x09 => self.add_hl_rr(opcode),
            0x0A => self.ld_a_drr(opcode),
            0x0B => self.dec_rr(opcode),
            0x0C => self.inc_lr(opcode),
            0x0D => self.dec_lr(opcode),
            0x0E => self.ld_lr_d8(opcode),
            0x0F => self.rrca(),
            0x10 => self.stop(),
            0x11 => self.ld_rr_d16(opcode),
            0x12 => self.ld_rr_a(opcode),
            0x13 => self.inc_rr(opcode),
            0x14 => self.inc_hr(opcode),
            0x15 => self.dec_hr(opcode),
            0x16 => self.ld_hr_d8(opcode),
            0x17 => self.rla(),
            0x18 => self.jr_r8(),
            0x19 => self.add_hl_rr(opcode),
            0x1A => self.ld_a_drr(opcode),
            0x1B => self.dec_rr(opcode),
            0x1C => self.inc_lr(opcode),
            0x1D => self.dec_lr(opcode),
            0x1E => self.ld_a_drr(opcode),
            0x1F => self.rra(),
            0x20 => self.jr_cc_r8(opcode),
            0x21 => self.ld_rr_d16(opcode),
            0x22 => self.ld_dhli_a(),
            0x23 => self.inc_rr(opcode),
            0x24 => self.inc_hr(opcode),
            0x25 => self.dec_hr(opcode),
            0x26 => self.ld_hr_d8(opcode),
            0x27 => self.daa(),
            0x28 => self.jr_cc_r8(opcode),
            0x29 => self.add_hl_rr(opcode),
            0x2A => self.ld_a_hli(),
            0x2B => self.dec_rr(opcode),
            0x2C => self.inc_lr(opcode),
            0x2D => self.dec_lr(opcode),
            0x2E => self.ld_a_drr(opcode),
            0x2F => self.cpl(),
            0x30 => self.jr_cc_r8(opcode),
            0x31 => self.ld_rr_d16(opcode),
            0x32 => self.ld_dhld_a(),
            0x33 => self.inc_rr(opcode),
            0x34 => self.inc_dhl(),
            0x35 => self.dec_dhl(),
            0x36 => self.ld_dhl_d8(),
            0x37 => self.scf(),
            0x38 => self.jr_cc_r8(opcode),
            0x39 => self.add_hl_rr(opcode),
            0x3A => self.ld_a_hld(),
            0x3B => self.dec_rr(opcode),
            0x3C => self.inc_lr(opcode),
            0x3D => self.dec_lr(opcode),
            0x3E => self.ld_a_d8(),
            0x3F => self.ccf(),
            0x40 => self.nop(), // LD B,B
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x46 => self.ld_b_dhl(),
            0x47 => self.ld_b_a(),
            0x48 => self.ld_c_b(),
            0x49 => self.nop(), // LD C, C
            0x4A => self.ld_c_d(),
            0x4B => self.ld_c_e(),
            0x4C => self.ld_c_h(),
            0x4D => self.ld_c_l(),
            0x4E => self.ld_c_dhl(),
            0x4F => self.ld_c_a(),
            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.nop(), // LD D, D
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x56 => self.ld_d_dhl(),
            0x57 => self.ld_d_a(),
            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5A => self.ld_e_d(),
            0x5B => self.nop(), // LD E, E
            0x5C => self.ld_e_h(),
            0x5D => self.ld_e_l(),
            0x5E => self.ld_e_dhl(),
            0x5F => self.ld_e_a(),
            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => self.nop(), // LD H, H
            0x65 => self.ld_h_l(),
            0x66 => self.ld_h_dhl(),
            0x67 => self.ld_h_a(),
            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6A => self.ld_l_d(),
            0x6B => self.ld_l_e(),
            0x6C => self.ld_l_h(),
            0x6D => self.nop(), // LD L, L
            0x6E => self.ld_l_dhl(),
            0x6F => self.ld_l_a(),
            0x70 => self.ld_dhl_b(),
            0x71 => self.ld_dhl_c(),
            0x72 => self.ld_dhl_d(),
            0x73 => self.ld_dhl_e(),
            0x74 => self.ld_dhl_h(),
            0x75 => self.ld_dhl_l(),
            0x76 => self.halt(),
            0x77 => self.ld_dhl_a(),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7E => self.ld_a_dhl(),
            0x7F => self.nop(), // LD A, A
            0x80 => self.add_b(),
            0x81 => self.add_c(),
            0x82 => self.add_d(),
            0x83 => self.add_e(),
            0x84 => self.add_h(),
            0x85 => self.add_l(),
            0x86 => self.add_dhl(),
            0x87 => self.add_a(),
            0x88 => self.adc_b(),
            0x89 => self.adc_c(),
            0x8A => self.adc_d(),
            0x8B => self.adc_e(),
            0x8C => self.adc_h(),
            0x8D => self.adc_l(),
            0x8E => self.adc_dhl(),
            0x8F => self.adc_a(),
            0x90 => self.sub_b(),
            0x91 => self.sub_c(),
            0x92 => self.sub_d(),
            0x93 => self.sub_e(),
            0x94 => self.sub_h(),
            0x95 => self.sub_l(),
            0x96 => self.sub_dhl(),
            0x97 => self.sub_a(),
            0x98 => self.sbc_b(),
            0x99 => self.sbc_c(),
            0x9A => self.sbc_d(),
            0x9B => self.sbc_e(),
            0x9C => self.sbc_h(),
            0x9D => self.sbc_l(),
            0x9E => self.sbc_dhl(),
            0x9F => self.sbc_a(),
            0xA0 => self.and_b(),
            0xA1 => self.and_c(),
            0xA2 => self.and_d(),
            0xA3 => self.and_e(),
            0xA4 => self.and_h(),
            0xA5 => self.and_l(),
            0xA6 => self.and_dhl(),
            0xA7 => self.and_a(),
            0xA8 => self.xor_b(),
            0xA9 => self.xor_c(),
            0xAA => self.xor_d(),
            0xAB => self.xor_e(),
            0xAC => self.xor_h(),
            0xAD => self.xor_l(),
            0xAE => self.xor_dhl(),
            0xAF => self.xor_a(),
            0xB0 => self.or_b(),
            0xB1 => self.or_c(),
            0xB2 => self.or_d(),
            0xB3 => self.or_e(),
            0xB4 => self.or_h(),
            0xB5 => self.or_l(),
            0xB6 => self.or_dhl(),
            0xB7 => self.or_a(),
            0xB8 => self.cp_b(),
            0xB9 => self.cp_c(),
            0xBA => self.cp_d(),
            0xBB => self.cp_e(),
            0xBC => self.cp_h(),
            0xBD => self.cp_l(),
            0xBE => self.cp_dhl(),
            0xBF => self.cp_a(),
            0xC0 => (),
            0xC1 => (),
            0xC2 => (),
            0xC3 => (),
            0xC4 => (),
            0xC5 => (),
            0xC6 => (),
            0xC7 => (),
            0xC8 => (),
            0xC9 => (),
            0xCA => (),
            0xCB => self.decode_cb_prefix(),
            0xCC => (),
            0xCD => (),
            0xCE => (),
            0xCF => (),
            0xD0 => (),
            0xD1 => (),
            0xD2 => (),
            0xD3 => (),
            0xD4 => (),
            0xD5 => (),
            0xD6 => (),
            0xD7 => (),
            0xD8 => (),
            0xD9 => (),
            0xDA => (),
            0xDB => (),
            0xDC => (),
            0xDD => (),
            0xDE => (),
            0xDF => (),
            0xE0 => (),
            0xE1 => (),
            0xE2 => (),
            0xE3 => (),
            0xE4 => (),
            0xE5 => (),
            0xE6 => (),
            0xE7 => (),
            0xE8 => (),
            0xE9 => (),
            0xEA => self.ld_nn_a(),
            0xEB => (),
            0xEC => (),
            0xED => (),
            0xEE => (),
            0xEF => (),
            0xF0 => (),
            0xF1 => (),
            0xF2 => (),
            0xF3 => (),
            0xF4 => (),
            0xF5 => (),
            0xF6 => (),
            0xF7 => (),
            0xF8 => (),
            0xF9 => (),
            0xFA => (),
            0xFB => (),
            0xFC => (),
            0xFD => (),
            0xFE => (),
            0xFF => (),
        }
    }

    pub fn get_reg_index(&self, opcode: u8) -> usize {
        if opcode & 0x00 == 0x00 || opcode & 0x08 == 0x08 {
            2 // B
        } else if opcode & 0x01 == 0x01 || opcode & 0x09 == 0x09 {
            3 // C
        } else if opcode & 0x02 == 0x02 || opcode & 0x0A == 0x0A {
            4 // D
        } else if opcode & 0x03 == 0x03 || opcode & 0x0B == 0x0B {
            5 // E
        } else if opcode & 0x04 == 0x04 || opcode & 0x0C == 0x0C {
            6 // H
        } else if opcode & 0x05 == 0x05 || opcode & 0x0D == 0x0D {
            7 // L
        } else if opcode & 0x06 == 0x06 || opcode & 0x0E == 0x0E {
            8 // (HL)
        } else {
            0 // A
        }
    }

    pub fn get_byte_register_from_index(&self, reg_index: usize) -> u8 {
        if reg_index == 8 {
            self.read_byte(self.cpu.hl())
        } else {
            self.cpu.registers[reg_index]
        }
    }

    pub fn get_word_register_from_index(&self, reg_index: usize) -> u16 {
        match reg_index {
            0 => self.cpu.af(),
            1 => self.cpu.bc(),
            2 => self.cpu.de(),
            3 => self.cpu.hl(),
            4 => self.cpu.sp,
            _ => unreachable!(),
        }
    }

    pub fn set_word_register_from_index(&mut self, index: usize, value: u16) {
        match index {
            0 => self.cpu.set_af(value & 0xFFF0),
            1 => self.cpu.set_bc(value),
            2 => self.cpu.set_de(value),
            3 => self.cpu.set_hl(value),
            4 => self.cpu.sp = value,
            _ => unreachable!(),
        }
    }

    pub fn set_byte_register_from_index(&mut self, reg_index: usize, value: u8) {
        match reg_index {
            0 => self.cpu.set_a(value),
            1 => self.cpu.set_f(value & 0xF0),
            2 => self.cpu.set_b(value),
            3 => self.cpu.set_c(value),
            4 => self.cpu.set_d(value),
            5 => self.cpu.set_e(value),
            6 => self.cpu.set_h(value),
            7 => self.cpu.set_l(value),
            8 => self.write_byte(self.cpu.hl(), value),
            _ => unreachable!(),
        }
    }

    fn get_flag_condition(&self, opcode: u8) -> bool {
        match (opcode >> 3) & 0x3 {
            0 => self.cpu.f_zero == false,
            1 => self.cpu.f_zero == true,
            2 => self.cpu.f_carry == false,
            3 => self.cpu.f_carry == true,
            _ => false,
        }
    }

    // REGULAR INSTRUCTIONS
    fn nop(&mut self) {
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn stop(&mut self) {
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn halt(&mut self) {
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_rr_d16(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        self.set_word_register_from_index(register_index, value);
        self.cpu.pc = self.cpu.pc.wrapping_add(3);
    }

    fn ld_rr_a(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let a = self.cpu.a() as u16;
        if register_index == 4 {
            self.cpu.sp = a;
        } else {
            self.set_word_register_from_index(register_index, a);
        }
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_nn_a(&mut self) {
        let value: u16 = (self.cpu.a() as u16) << 8;
        let memory_value: u16 = self.read_word(self.cpu.pc.wrapping_add(1));
        self.write_word(memory_value, value);
        self.cpu.pc = self.cpu.pc.wrapping_add(3);
    }

    fn inc_rr(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let register_value: u16 = self
            .get_word_register_from_index(register_index)
            .wrapping_add(1);
        self.set_word_register_from_index(register_index, register_value);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn inc_hr(&mut self, opcode: u8) {
        let register_index: usize = ((opcode as usize >> 4) + 1) & 0x03;
        let register_value: u16 = self
            .get_word_register_from_index(register_index)
            .wrapping_add(0x100);
        self.set_word_register_from_index(register_index, register_value);
        self.cpu.set_zero(register_value == 0);
        self.cpu.set_negative(false);
        let half_carry = register_value & 0x0F00 == 0;
        self.cpu.set_half_carry(half_carry);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn dec_hr(&mut self, opcode: u8) {
        let register_index: usize = ((opcode as usize >> 4) + 1) & 0x03;
        let register_value: u16 = self
            .get_word_register_from_index(register_index)
            .wrapping_sub(0x100);
        self.set_word_register_from_index(register_index, register_value);
        self.cpu.set_zero(register_value == 0);
        self.cpu.set_negative(true);
        let half_carry = register_value & 0x0F00 == 0xF00;
        self.cpu.set_half_carry(half_carry);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_hr_d8(&mut self, opcode: u8) {
        let register_index: usize = ((opcode as usize >> 4) + 1) & 0x03;
        let register_value: u16 = self.get_word_register_from_index(register_index);
        let value = (self.read_byte(self.cpu.pc.wrapping_add(1)) as u16) << 8;
        self.set_word_register_from_index(register_index, (register_value & 0xFF) | value);
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
    }

    fn ld_dhl_d8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        self.write_byte(self.cpu.hl(), value);
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
    }

    fn rlca(&mut self) {
        let carry = self.cpu.af() & 0x8000 != 0;
        let value = self.cpu.af() & 0xFF00 << 1;
        self.cpu.set_af(value);
        if carry {
            let value = self.cpu.af() | 0x10 | 0x0100;
            self.cpu.set_af(value);
        }
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn rla(&mut self) {
        let bit7 = (self.cpu.af() & 0x8000) != 0;
        let carry = self.cpu.af() & 16 != 0;

        let value = self.cpu.af() & 0xFF00 << 1;
        self.cpu.set_af(value);
        if carry {
            let value = self.cpu.af() | 0x0100;
            self.cpu.set_af(value);
        }
        if bit7 {
            let value = self.cpu.af() | 16;
            self.cpu.set_af(value);
        }
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_da16_sp(&mut self) {
        let address: u16 = self.read_word(self.cpu.pc.wrapping_add(1));
        self.write_word(address, self.cpu.sp);
        self.cpu.pc = self.cpu.pc.wrapping_add(3);
    }

    fn add_hl_rr(&mut self, opcode: u8) {
        let hl: u16 = self.cpu.hl();
        let register_index: usize = (opcode as usize >> 4) + 1;
        let rr: u16 = self.get_word_register_from_index(register_index);
        self.cpu.set_hl(hl.wrapping_add(rr));
        self.cpu.set_negative(false);
        self.cpu
            .set_half_carry(((hl as u32) & 0xFFF) + ((rr as u32) & 0xFFF) & 0x1000 != 0);
        self.cpu.set_carry((hl as u32 + rr as u32) & 0x10000 != 0);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_a_hli(&mut self) {
        let hl = self.cpu.hl().wrapping_add(1);
        let value = self.read_byte(hl);
        self.cpu.set_a(value);
        self.cpu.set_hl(hl);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_a_hld(&mut self) {
        let hl = self.cpu.hl().wrapping_sub(1);
        let value = self.read_byte(hl);
        self.cpu.set_a(value);
        self.cpu.set_hl(hl);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_a_drr(&mut self, opcode: u8) {
        let register_index = (opcode as usize >> 4) + 1;
        let register: u16 = self.get_word_register_from_index(register_index);
        let value: u8 = self.read_byte(register);
        self.cpu.set_a(value);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn dec_rr(&mut self, opcode: u8) {
        let register_index = (opcode as usize >> 4) + 1;
        let rr: u16 = self.get_word_register_from_index(register_index);
        self.set_word_register_from_index(register_index, rr.wrapping_sub(1));
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn inc_lr(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let register_value: u16 = self.get_word_register_from_index(register_index);
        let value: u16 = (register_value & 0xFF).wrapping_add(1);
        self.set_word_register_from_index(register_index, (register_value & 0xFF00) | value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        let half_carry: bool = value & 0x0F == 0;
        self.cpu.set_half_carry(half_carry);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn dec_lr(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let register_value: u16 = self.get_word_register_from_index(register_index);
        let value: u16 = (register_value & 0xFF).wrapping_sub(1);
        self.set_word_register_from_index(register_index, (register_value & 0xFF00) | value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        let half_carry: bool = value & 0x0F == 0xF;
        self.cpu.set_half_carry(half_carry);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_lr_d8(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let mut register: u16 = self.get_word_register_from_index(register_index);
        self.set_word_register_from_index(register_index, register & 0xFF00);
        register = self.get_word_register_from_index(register_index);
        let value: u16 = self.read_byte(self.cpu.pc.wrapping_add(1)) as u16;
        self.set_word_register_from_index(register_index, register | value);
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
    }

    fn rrca(&mut self) {
        let carry: bool = self.cpu.af() & 0x100 != 0;
        let a: u8 = self.cpu.a();
        self.cpu.set_a(a >> 1);
        self.cpu.set_carry(carry);
    }

    fn rra(&mut self) {
        let bit1: bool = self.cpu.af() & 0x100 != 0;
        let carry = self.cpu.f_carry;

        let mut a: u8 = self.cpu.a();
        self.cpu.set_a(a >> 1);
        a = self.cpu.a();
        if carry {
            self.cpu.set_a(a | 0x80);
        }
        self.cpu.set_carry(bit1);
    }

    fn jr_r8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8 as i16 as u16;
        self.cpu.pc = self.cpu.pc.wrapping_add(value);
    }

    fn jr_cc_r8(&mut self, opcode: u8) {
        if self.get_flag_condition(opcode) {
            self.jr_r8();
        } else {
            self.cpu.pc = self.cpu.pc.wrapping_add(2);
        }
    }

    fn daa(&mut self) {
        let mut a = self.cpu.a();
        let mut adjust = if self.cpu.f_carry { 0x60 } else { 0 };
        if self.cpu.f_half_carry {
            adjust |= 0x06;
        }
        if !self.cpu.f_negative {
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            }
            if a > 0x99 {
                adjust |= 0x60;
            }
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.cpu.set_carry(adjust >= 0x60);
        self.cpu.set_half_carry(false);
        self.cpu.set_zero(a == 0);
        self.cpu.set_a(a);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn cpl(&mut self) {
        let a = self.cpu.a();
        self.cpu.set_a(!a);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry(true);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn scf(&mut self) {
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(true);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_a_d8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        self.cpu.set_a(value);
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
    }

    fn ccf(&mut self) {
        let value = !self.cpu.f_carry;
        self.cpu.set_carry(value);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_dhli_a(&mut self) {
        let hl = self.cpu.hl().wrapping_add(1);
        let a = self.cpu.a();
        self.write_byte(hl, a);
        self.cpu.set_hl(hl);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn ld_dhld_a(&mut self) {
        let hl = self.cpu.hl().wrapping_sub(1);
        let a = self.cpu.a();
        self.write_byte(hl, a);
        self.cpu.set_hl(hl);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn inc_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_word(hl).wrapping_add(1);
        self.write_word(hl, value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry((value & 0x0F) == 0);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn dec_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_word(hl).wrapping_sub(1);
        self.write_word(hl, value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((value & 0x0F) == 0);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn add_dhl(&mut self) {
        let hl = self.cpu.hl();
        let a = self.cpu.a();
        let r = self.read_byte(hl);
        let (value, did_overflow) = a.overflowing_add(r);
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry((value & 0xF) > 0x0F);
        self.cpu.set_carry(did_overflow);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn adc_dhl(&mut self) {
        let hl = self.cpu.hl();
        let r = self.read_byte(hl);
        let a = self.cpu.a();
        let carry = self.cpu.get_carry();
        let (mut value, mut did_overflow) = a.overflowing_add(carry);
        if did_overflow {
            value = value.wrapping_add(r);
        } else {
            (value, did_overflow) = value.overflowing_add(r);
        }
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry((value & 0xF) > 0x0F);
        self.cpu.set_carry(did_overflow);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn sub_dhl(&mut self) {
        let hl = self.cpu.hl();
        let r = self.read_byte(hl);
        let a = self.cpu.a();
        let (value, did_overflow) = a.overflowing_sub(r);
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((a & 0xF) < (r & 0xF));
        self.cpu.set_carry(did_overflow);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn sbc_dhl(&mut self) {
        let hl = self.cpu.hl();
        let r = self.read_byte(hl);
        let a = self.cpu.a();
        let carry = self.cpu.get_carry();
        let (mut value, mut did_overflow) = a.overflowing_sub(carry);
        if did_overflow {
            value = value.wrapping_sub(r);
        } else {
            (value, did_overflow) = value.overflowing_sub(r);
        }
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((a & 0xF) < (value & 0xF) + carry);
        self.cpu.set_carry(did_overflow);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn and_dhl(&mut self) {
        let hl = self.cpu.hl();
        let r = self.read_byte(hl);
        let a = self.cpu.a();
        let value = a & r;
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(true);
        self.cpu.set_carry(false);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn xor_dhl(&mut self) {
        let hl = self.cpu.hl();
        let r = self.read_byte(hl);
        let a = self.cpu.a();
        let value = a ^ r;
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(false);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn or_dhl(&mut self) {
        let hl = self.cpu.hl();
        let r = self.read_byte(hl);
        let a = self.cpu.a();
        let value = a | r;
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(false);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn cp_dhl(&mut self) {
        let hl = self.cpu.hl();
        let r = self.read_byte(hl);
        let a = self.cpu.a();
        self.cpu.set_zero(a == r);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((a & 0xF) < (r & 0xF));
        self.cpu.set_carry(a < r);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }
}

// Create all simple LD instructions using macros
macro_rules! create_ld_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<ld_a_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_a(value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_b_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_b(value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_c_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_c(value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_d_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_d(value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_e_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_e(value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_h_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_h(value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_l_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_l(value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_ $r _dhl>](&mut self) {
                        let hl = self.cpu.hl();
                        let value = self.read_byte(hl);
                        self.cpu.[<set_ $r>](value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }

                    fn [<ld_dhl_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let hl = self.cpu.hl();
                        self.write_byte(hl, value);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_add_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<add_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        let (value, did_overflow) = a.overflowing_add(r);
                        self.cpu.set_a(value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(false);
                        self.cpu.set_half_carry((value & 0xF) > 0x0F);
                        self.cpu.set_carry(did_overflow);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_adc_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<adc_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        let carry = self.cpu.get_carry();
                        let (mut value, mut did_overflow) = a.overflowing_add(carry);
                        if did_overflow {
                            value = value.wrapping_add(r);
                        } else {
                            (value, did_overflow) = value.overflowing_add(r);
                        }
                        self.cpu.set_a(value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(false);
                        self.cpu.set_half_carry((value & 0xF) > 0x0F);
                        self.cpu.set_carry(did_overflow);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_sub_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<sub_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        let (value, did_overflow) = a.overflowing_sub(r);
                        self.cpu.set_a(value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(true);
                        self.cpu.set_half_carry((a & 0xF) < (r & 0xF));
                        self.cpu.set_carry(did_overflow);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_sbc_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<sbc_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        let carry = self.cpu.get_carry();
                        let (mut value, mut did_overflow) = a.overflowing_sub(carry);
                        if did_overflow {
                            value = value.wrapping_sub(r);
                        } else {
                            (value, did_overflow) = value.overflowing_sub(r);
                        }
                        self.cpu.set_a(value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(true);
                        self.cpu.set_half_carry((a & 0xF) < (value & 0xF) + carry);
                        self.cpu.set_carry(did_overflow);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_and_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<and_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        let value = a & r;
                        self.cpu.set_a(value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(false);
                        self.cpu.set_half_carry(true);
                        self.cpu.set_carry(false);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_xor_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<xor_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        let value = a ^ r;
                        self.cpu.set_a(value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(false);
                        self.cpu.set_half_carry(false);
                        self.cpu.set_carry(false);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_or_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<or_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        let value = a | r;
                        self.cpu.set_a(value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(false);
                        self.cpu.set_half_carry(false);
                        self.cpu.set_carry(false);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}

macro_rules! create_cp_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<cp_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let a = self.cpu.a();
                        self.cpu.set_zero(a == r);
                        self.cpu.set_negative(true);
                        self.cpu.set_half_carry((a & 0xF) < (r & 0xF));
                        self.cpu.set_carry(a < r);
                        self.cpu.pc = self.cpu.pc.wrapping_add(1);
                    }
                )*
            }
        }
    }
}



create_ld_instructions!(a, b, c, d, e, h, l);
create_add_byte_instructions!(a, b, c, d, e, h, l);
create_adc_byte_instructions!(a, b, c, d, e, h, l);
create_sub_byte_instructions!(a, b, c, d, e, h, l);
create_sbc_byte_instructions!(a, b, c, d, e, h, l);
create_and_byte_instructions!(a, b, c, d, e, h, l);
create_xor_byte_instructions!(a, b, c, d, e, h, l);
create_or_byte_instructions!(a, b, c, d, e, h, l);
create_cp_byte_instructions!(a, b, c, d, e, h, l);


#[cfg(test)]
mod test {
    use super::*;
    use crate::core::cpu;
    use crate::core::gb::Gb;
}
