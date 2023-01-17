// Decode instructions
// The implementation here is based on the SameBoy implementation

use crate::core::gb::Gb;
use crate::core::memorybus;

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
            0x08 => (),
            0x09 => (),
            0x0A => (),
            0x0B => (),
            0x0C => self.inc_lr(opcode),
            0x0D => self.dec_lr(opcode),
            0x0E => (),
            0x0F => (),
            0x10 => (),
            0x11 => self.ld_rr_d16(opcode),
            0x12 => self.ld_rr_a(opcode),
            0x13 => self.inc_rr(opcode),
            0x14 => self.inc_hr(opcode),
            0x15 => self.dec_hr(opcode),
            0x16 => self.ld_hr_d8(opcode),
            0x17 => self.rla(),
            0x18 => self.jr_r8(),
            0x19 => (),
            0x1A => (),
            0x1B => (),
            0x1C => self.inc_lr(opcode),
            0x1D => self.dec_lr(opcode),
            0x1E => (),
            0x1F => (),
            0x20 => self.jr_cc_r8(),
            0x21 => self.ld_rr_d16(opcode),
            0x22 => self.ld_rr_a(opcode),
            0x23 => self.inc_rr(opcode),
            0x24 => self.inc_hr(opcode),
            0x25 => self.dec_hr(opcode),
            0x26 => self.ld_hr_d8(opcode),
            0x27 => (),
            0x28 => self.jr_cc_r8(),
            0x29 => (),
            0x2A => (),
            0x2B => (),
            0x2C => self.inc_lr(opcode),
            0x2D => self.dec_lr(opcode),
            0x2E => (),
            0x2F => (),
            0x30 => self.jr_cc_r8(),
            0x31 => self.ld_rr_d16(opcode),
            0x32 => (),
            0x33 => self.inc_rr(opcode),
            0x34 => self.inc_hr(opcode),
            0x35 => self.dec_hr(opcode),
            0x36 => self.ld_dhl_d8(),
            0x37 => (),
            0x38 => self.jr_cc_r8(),
            0x39 => (),
            0x3A => (),
            0x3B => (),
            0x3C => self.inc_lr(opcode),
            0x3D => self.dec_lr(opcode),
            0x3E => (),
            0x3F => (),
            0x40 => (),
            0x41 => (),
            0x42 => (),
            0x43 => (),
            0x44 => (),
            0x45 => (),
            0x46 => (),
            0x47 => (),
            0x48 => (),
            0x49 => (),
            0x4A => (),
            0x4B => (),
            0x4C => (),
            0x4D => (),
            0x4E => (),
            0x4F => (),
            0x50 => (),
            0x51 => (),
            0x52 => (),
            0x53 => (),
            0x54 => (),
            0x55 => (),
            0x56 => (),
            0x57 => (),
            0x58 => (),
            0x59 => (),
            0x5A => (),
            0x5B => (),
            0x5C => (),
            0x5D => (),
            0x5E => (),
            0x5F => (),
            0x60 => (),
            0x61 => (),
            0x62 => (),
            0x63 => (),
            0x64 => (),
            0x65 => (),
            0x66 => (),
            0x67 => (),
            0x68 => (),
            0x69 => (),
            0x6A => (),
            0x6B => (),
            0x6C => (),
            0x6D => (),
            0x6E => (),
            0x6F => (),
            0x70 => (),
            0x71 => (),
            0x72 => (),
            0x73 => (),
            0x74 => (),
            0x75 => (),
            0x76 => (),
            0x77 => self.ld_rr_a(opcode),
            0x78 => (),
            0x79 => (),
            0x7A => (),
            0x7B => (),
            0x7C => (),
            0x7D => (),
            0x7E => (),
            0x7F => (),
            0x80 => (),
            0x81 => (),
            0x82 => (),
            0x83 => (),
            0x84 => (),
            0x85 => (),
            0x86 => (),
            0x87 => (),
            0x88 => (),
            0x89 => (),
            0x8A => (),
            0x8B => (),
            0x8C => (),
            0x8D => (),
            0x8E => (),
            0x8F => (),
            0x90 => (),
            0x91 => (),
            0x92 => (),
            0x93 => (),
            0x94 => (),
            0x95 => (),
            0x96 => (),
            0x97 => (),
            0x98 => (),
            0x99 => (),
            0x9A => (),
            0x9B => (),
            0x9C => (),
            0x9D => (),
            0x9E => (),
            0x9F => (),
            0xA0 => (),
            0xA1 => (),
            0xA2 => (),
            0xA3 => (),
            0xA4 => (),
            0xA5 => (),
            0xA6 => (),
            0xA7 => (),
            0xA8 => (),
            0xA9 => (),
            0xAA => (),
            0xAB => (),
            0xAC => (),
            0xAD => (),
            0xAE => (),
            0xAF => (),
            0xB0 => (),
            0xB1 => (),
            0xB2 => (),
            0xB3 => (),
            0xB4 => (),
            0xB5 => (),
            0xB6 => (),
            0xB7 => (),
            0xB8 => (),
            0xB9 => (),
            0xBA => (),
            0xBB => (),
            0xBC => (),
            0xBD => (),
            0xBE => (),
            0xBF => (),
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
            0xCB => self.decode_prefix(),
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

    fn decode_prefix(&mut self) {
        let prefix = self.read_byte(self.cpu.pc.wrapping_add(1));
        match prefix >> 3 {
            0 => self.rlc_r(prefix),
            1 => self.rrc_r(prefix),
            2 => self.rl_r(prefix),
            3 => self.rr_r(prefix),
            4 => self.sla_r(prefix),
            5 => self.sra_r(prefix),
            6 => self.swap_r(prefix),
            7 => self.srl_r(prefix),
            _ => self.bit_res_set_r(prefix),
        };
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
    }

    fn get_reg_index(&self, opcode: u8) -> usize {
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

    fn get_byte_register_from_index(&self, reg_index: usize) -> u8 {
        if reg_index == 8 {
            self.read_byte(self.cpu.hl())
        } else {
            self.cpu.registers[reg_index]
        }
    }

    fn get_word_register_from_index(&self, reg_index: usize) -> u16 {
        match reg_index {
            0 => self.cpu.af(),
            1 => self.cpu.bc(),
            2 => self.cpu.de(),
            3 => self.cpu.hl(),
            4 => self.cpu.sp,
            _ => unreachable!(),
        }
    }

    fn set_word_register_from_index(&mut self, index: usize, value: u16) {
        match index {
            0 => self.cpu.set_af(value & 0xFFF0),
            1 => self.cpu.set_bc(value),
            2 => self.cpu.set_de(value),
            3 => self.cpu.set_hl(value),
            4 => self.cpu.sp = value,
            _ => unreachable!(),
        }
    }

    fn set_byte_register_from_index(&mut self, reg_index: usize, value: u8) {
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
        let register_value: u16 = self.get_word_register_from_index(register_index).wrapping_add(0x100);
        self.set_word_register_from_index(register_index, register_value);
        self.cpu.set_zero(register_value == 0);
        self.cpu.set_negative(false);
        let half_carry = register_value & 0x0F00 == 0;
        self.cpu.set_half_carry(half_carry);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn dec_hr(&mut self, opcode: u8) {
        let register_index: usize = ((opcode as usize >> 4) + 1) & 0x03;
        let register_value: u16 = self.get_word_register_from_index(register_index).wrapping_sub(0x100);
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
    }

    fn inc_lr(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let register_value: u16 = self.get_word_register_from_index(register_index);
        let value = (register_value & 0xFF).wrapping_add(1);
        self.set_word_register_from_index(register_index, (register_value & 0xFF00) | value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        let half_carry = value & 0x0F == 0;
        self.cpu.set_half_carry(half_carry);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn dec_lr(&mut self, opcode: u8) {
        let register_index: usize = (opcode as usize >> 4) + 1;
        let register_value: u16 = self.get_word_register_from_index(register_index);
        let value = (register_value & 0xFF).wrapping_sub(1);
        self.set_word_register_from_index(register_index, (register_value & 0xFF00) | value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        let half_carry = value & 0x0F == 0xF;
        self.cpu.set_half_carry(half_carry);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
    }

    fn jr_r8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8 as i16 as u16;
        self.cpu.pc = self.cpu.pc.wrapping_add(value);
    }

    fn jr_cc_r8(&mut self) {
        if self.get_flag_condition(self.cpu.current_instruction) {
            self.jr_r8();
        } else {
            self.cpu.pc = self.cpu.pc.wrapping_add(2);
        }
    }

    // DECODE CB SECTION
    fn rlc_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        self.cpu.set_carry((value & 0x01) != 0);
        let carry_value: u8 = self.cpu.get_carry();
        value = (value << 1) | carry_value;
        self.set_byte_register_from_index(reg_index, value);
        self.cpu.set_zero(value == 0);
    }

    fn rrc_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        self.cpu.set_carry((value & 0x01) != 0);
        let carry_value: u8 = self.cpu.get_carry();
        value = (value >> 1) | (carry_value << 7);
        self.set_byte_register_from_index(reg_index, value);
        self.cpu.set_zero(value == 0);
    }

    fn rl_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        self.cpu.set_carry((value & 0x80) != 0);
        let carry_value: u8 = self.cpu.get_carry();
        value = (value << 1) | carry_value;
        self.set_byte_register_from_index(reg_index, value);
        self.cpu.set_zero(value == 0);
    }

    fn rr_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        self.cpu.set_carry((value & 0x1) != 0);
        let carry_value: u8 = self.cpu.get_carry();
        value = (value >> 1) | (carry_value << 7);
        self.set_byte_register_from_index(reg_index, value);
        self.cpu.set_zero(value == 0);
    }

    fn sla_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        self.cpu.set_carry((value & 0x80) != 0);
        value = value << 1;
        self.set_byte_register_from_index(reg_index, value);
        self.cpu.set_zero((value & 0x7F) == 0);
    }

    fn sra_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        self.cpu.set_carry(value & 1 == 1);
        value = (value >> 1) | (value & 0x80);
        self.set_byte_register_from_index(reg_index, value);
        self.cpu.set_zero(value == 0);
    }

    fn swap_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        value = (value >> 4) | (value << 4);
        self.cpu.set_zero(value == 0);
    }

    fn srl_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let mut value: u8 = self.get_byte_register_from_index(reg_index);
        self.cpu.set_f(0);
        self.cpu.set_carry(value & 1 == 1);
        value = value >> 1;
        self.set_byte_register_from_index(reg_index, value);
        self.cpu.set_zero(value == 0)
    }

    fn bit_res_set_r(&mut self, prefix: u8) {
        let reg_index = self.get_reg_index(prefix);
        let value: u8 = self.get_byte_register_from_index(reg_index);
        let result = 1 << ((prefix >> 3) & 7);
        if (prefix & 0xC0) == 0x40 {
            // BIT
            self.cpu.set_half_carry(true);
            self.cpu.set_negative(false);
            self.cpu.set_zero((result & value) == 0);
        } else if (prefix & 0xC0) == 0x80 {
            // RES
            self.set_byte_register_from_index(reg_index, value & !result);
        } else if (prefix & 0xC0) == 0xC0 {
            // SET
            self.set_byte_register_from_index(reg_index, value | result);
        }
    }
}
