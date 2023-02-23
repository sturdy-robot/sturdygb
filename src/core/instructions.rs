use crate::core::gb::Gb;
use paste::paste;

//  M-CYCLES
//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
const OPCODE_CYCLES: [usize; 256] = [
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1,
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1,
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, 
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
    2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 1, 
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, 
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4,
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, 
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4,
];


impl Gb {
    pub fn decode(&mut self) {
        let opcode = self.cpu.current_instruction;
        self.cpu.pending_cycles = OPCODE_CYCLES[opcode as usize];
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_bc_d16(),
            0x02 => self.ld_bc_a(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_d8(),
            0x07 => self.rlca(),
            0x08 => self.ld_da16_sp(),
            0x09 => self.add_hl_bc(),
            0x0A => self.ld_a_bc(),
            0x0B => self.dec_bc(),
            0x0C => self.inc_c(),
            0x0D => self.dec_c(),
            0x0E => self.ld_c_d8(),
            0x0F => self.rrca(),
            0x10 => self.stop(),
            0x11 => self.ld_de_d16(),
            0x12 => self.ld_de_a(),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x16 => self.ld_d_d8(),
            0x17 => self.rla(),
            0x18 => self.jr_r8(),
            0x19 => self.add_hl_de(),
            0x1A => self.ld_a_de(),
            0x1B => self.dec_de(),
            0x1C => self.inc_e(),
            0x1D => self.dec_e(),
            0x1E => self.ld_e_d8(),
            0x1F => self.rra(),
            0x20 => self.jr_cc_r8(opcode),
            0x21 => self.ld_hl_d16(),
            0x22 => self.ld_dhli_a(),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_d8(),
            0x27 => self.daa(),
            0x28 => self.jr_cc_r8(opcode),
            0x29 => self.add_hl_hl(),
            0x2A => self.ld_a_hli(),
            0x2B => self.dec_hl(),
            0x2C => self.inc_l(),
            0x2D => self.dec_l(),
            0x2E => self.ld_l_d8(),
            0x2F => self.cpl(),
            0x30 => self.jr_cc_r8(opcode),
            0x31 => self.ld_sp_d16(),
            0x32 => self.ld_dhld_a(),
            0x33 => self.inc_sp(),
            0x34 => self.inc_dhl(),
            0x35 => self.dec_dhl(),
            0x36 => self.ld_dhl_d8(), 
            0x37 => self.scf(),
            0x38 => self.jr_cc_r8(opcode),
            0x39 => self.add_hl_sp(),
            0x3A => self.ld_a_hld(),
            0x3B => self.dec_sp(),
            0x3C => self.inc_a(),
            0x3D => self.dec_a(),
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
            0xC0 => self.ret_cc(opcode),
            0xC1 => self.pop_bc(),
            0xC2 => self.jp_cc(opcode),
            0xC3 => self.jp(),
            0xC4 => self.call_cc(opcode),
            0xC5 => self.push_bc(),
            0xC6 => self.add_a_d8(),
            0xC7 => self.rst(opcode),
            0xC8 => self.ret_cc(opcode),
            0xC9 => self.ret(),
            0xCA => self.jp_cc(opcode),
            0xCB => self.decode_cb_prefix(),
            0xCC => self.call_cc(opcode),
            0xCD => self.call_a16(),
            0xCE => self.adc_a_d8(),
            0xCF => self.rst(opcode),
            0xD0 => self.ret_cc(opcode),
            0xD1 => self.pop_de(),
            0xD2 => self.jp_cc(opcode),
            0xD3 => self.ill(opcode),
            0xD4 => self.call_cc(opcode),
            0xD5 => self.push_de(),
            0xD6 => self.sub_a_d8(),
            0xD7 => self.rst(opcode),
            0xD8 => self.ret_cc(opcode),
            0xD9 => self.reti(),
            0xDA => self.jp_cc(opcode),
            0xDB => self.ill(opcode),
            0xDC => self.call_cc(opcode),
            0xDD => self.ill(opcode),
            0xDE => self.sbc_a_d8(),
            0xDF => self.rst(opcode),
            0xE0 => self.ldh_a8_a(),
            0xE1 => self.pop_hl(),
            0xE2 => self.ld_dc_a(),
            0xE3 => self.ill(opcode),
            0xE4 => self.ill(opcode),
            0xE5 => self.push_hl(),
            0xE6 => self.and_a_d8(),
            0xE7 => self.rst(opcode),
            0xE8 => self.add_sp_r8(),
            0xE9 => self.jp_hl(),
            0xEA => self.ld_a16_a(),
            0xEB => self.ill(opcode),
            0xEC => self.ill(opcode),
            0xED => self.ill(opcode),
            0xEE => self.xor_a_d8(),
            0xEF => self.rst(opcode),
            0xF0 => self.ldh_a_a8(),
            0xF1 => self.pop_af(),
            0xF2 => self.ld_a_dc(),
            0xF3 => self.di(),
            0xF4 => self.ill(opcode),
            0xF5 => self.push_af(),
            0xF6 => self.or_a_d8(),
            0xF7 => self.rst(opcode),
            0xF8 => self.ld_hl_sp_r8(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_da16(),
            0xFB => self.ei(),
            0xFC => self.ill(opcode),
            0xFD => self.ill(opcode),
            0xFE => self.cp_d8(),
            0xFF => self.rst(opcode),
        }
    }

    fn get_flag_condition(&self, opcode: u8) -> bool {
        match (opcode >> 3) & 0x3 {
            0 => self.cpu.zero == false,
            1 => self.cpu.zero == true,
            2 => self.cpu.carry == false,
            3 => self.cpu.carry == true,
            _ => false,
        }
    }

    // REGULAR INSTRUCTIONS
    fn nop(&mut self) {
        self.cpu.advance_pc();
    }

    fn ill(&mut self, opcode: u8) {
        panic!("ILLEGAL OPCODE: {opcode:04X}");
    }

    fn stop(&mut self) {
        // TODO: implement this! (Or not)
        //self.cpu.is_stopped = true;
        self.cpu.advance_pc();
    }

    fn halt(&mut self) {
        self.cpu.is_halted = true;
        self.cpu.advance_pc();
    }

    fn ld_sp_d16(&mut self) {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        self.cpu.sp = value;
        self.cpu.advance_pc();
    }

    fn ld_dhl_d8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        self.write_byte(self.cpu.hl(), value);
        self.cpu.advance_pc();
    }

    fn inc_sp(&mut self) {
        self.cpu.sp = self.cpu.sp.wrapping_add(1);
        self.cpu.advance_pc();
    }

    fn dec_sp(&mut self) {
        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
        self.cpu.advance_pc();
    }

    fn rlca(&mut self) {
        let value = self.cpu.a();
        let carry = value & 0x80 == 0x80;
        let r = (value << 1) | (if carry { 1 } else { 0 });
        self.cpu.set_zero(false);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(carry);
        self.cpu.set_a(r);
        self.cpu.advance_pc();
    }

    fn rla(&mut self) {
        let value = self.cpu.a();
        let carry = value & 0x80 == 0x80;
        let r = (value << 1) | (self.cpu.get_carry());
        self.cpu.set_zero(false);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(carry);
        self.cpu.set_a(r);
        self.cpu.advance_pc();
    }

    fn ld_da16_sp(&mut self) {
        let address: u16 = self.read_word(self.cpu.pc.wrapping_add(1));
        self.write_word(address, self.cpu.sp);
        self.cpu.advance_pc();
    }

    fn ld_a_hli(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        self.cpu.set_a(value);
        self.cpu.set_hl(hl.wrapping_add(1));
        self.cpu.advance_pc();
    }

    fn ld_a_hld(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        self.cpu.set_a(value);
        self.cpu.set_hl(hl.wrapping_sub(1));
        self.cpu.advance_pc();
    }

    fn rrca(&mut self) {
        let value = self.cpu.a();
        let carry = value & 0x01 == 0x01;
        let r = (value >> 1) | (if carry { 0x80 } else { 0 });
        self.cpu.set_zero(false);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(carry);
        self.cpu.set_a(r);
        self.cpu.advance_pc();
    }

    fn rra(&mut self) {
        let value = self.cpu.a();
        let carry = value & 0x01 == 0x01;
        let r = (value >> 1) | (if self.cpu.carry { 0x80 } else { 0 });
        self.cpu.set_zero(false);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(carry);
        self.cpu.set_a(r);
        self.cpu.advance_pc();
    }

    fn jr_r8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8;
        self.cpu.pc = self.cpu.pc.wrapping_add(2);
        self.cpu.pc = ((self.cpu.pc as u32 as i32) + (value as i32)) as u16;
    }

    fn jr_cc_r8(&mut self, opcode: u8) {
        if self.get_flag_condition(opcode) {
            self.cpu.pending_cycles = 3;
            self.jr_r8();
        } else {
            self.cpu.advance_pc();
        }
    }

    fn daa(&mut self) {
        let mut a = self.cpu.a();
        let mut adjust = if self.cpu.carry { 0x60 } else { 0 };
        if self.cpu.half_carry { adjust |= 0x06; };
        if !self.cpu.negative {
            if a & 0x0F > 0x09 { adjust |= 0x06; };
            if a > 0x99 { adjust |= 0x60; };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.cpu.set_carry(adjust >= 0x60);
        self.cpu.set_half_carry(false);
        self.cpu.set_zero(a == 0);
        self.cpu.set_a(a);
        self.cpu.advance_pc();
    }

    fn cpl(&mut self) {
        let a = self.cpu.a();
        self.cpu.set_a(!a);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry(true);
        self.cpu.advance_pc();
    }

    fn scf(&mut self) {
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(true);
        self.cpu.advance_pc();
    }

    fn ccf(&mut self) {
        let value = !self.cpu.carry;
        self.cpu.set_carry(value);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.advance_pc();
    }

    fn ld_dhli_a(&mut self) {
        let hl = self.cpu.hl();
        let a = self.cpu.a();
        self.write_byte(hl, a);
        self.cpu.set_hl(hl.wrapping_add(1));
        self.cpu.advance_pc();
    }

    fn ld_dhld_a(&mut self) {
        let hl = self.cpu.hl();
        let a = self.cpu.a();
        self.write_byte(hl, a);
        self.cpu.set_hl(hl.wrapping_sub(1));
        self.cpu.advance_pc();
    }

    fn inc_dhl(&mut self) {
        let hl = self.cpu.hl();
        let v = self.read_byte(hl);
        let value = v.wrapping_add(1);
        self.write_byte(hl, value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry((v & 0x0F) + 1 > 0x0F);
        self.cpu.advance_pc();
    }

    fn dec_dhl(&mut self) {
        let hl = self.cpu.hl();
        let v = self.read_byte(hl);
        let value = v.wrapping_sub(1);
        self.write_byte(hl, value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((v & 0x0F) == 0);
        self.cpu.advance_pc();
    }

    fn alu_add(&mut self, r: u8) {
        let a = self.cpu.a();
        let value = a.wrapping_add(r);
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry((a & 0xF) + (r & 0xF) > 0x0F);
        self.cpu.set_carry((a as u16) + (r as u16) > 0xFF);
        self.cpu.advance_pc();
    }

    fn alu_adc(&mut self, r: u8) {
        let a = self.cpu.a();
        let carry = self.cpu.get_carry();
        let value = a.wrapping_add(r).wrapping_add(carry);
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry((a & 0xF) + (r & 0xF) + carry > 0x0F);
        self.cpu.set_carry((a as u16) + (r as u16) + (carry as u16) > 0xFF);
        self.cpu.advance_pc();
    }

    fn alu_sub(&mut self, r: u8) {
        let a = self.cpu.a();
        let value = a.wrapping_sub(r);
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((a & 0xF) < (r & 0xF));
        self.cpu.set_carry((a as u16) < (r as u16));
        self.cpu.advance_pc();
    }

    fn alu_sbc(&mut self, r: u8) {
        let a = self.cpu.a();
        let carry = self.cpu.get_carry();
        let value = a.wrapping_sub(r).wrapping_sub(carry);
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((a & 0xF) < (r & 0xF) + carry);
        self.cpu.set_carry((a as u16) < (r as u16) + (carry as u16));
        self.cpu.advance_pc();
    }

    fn alu_xor(&mut self, r: u8) {
        let a = self.cpu.a();
        let value = a ^ r;
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(false);
        self.cpu.advance_pc();
    }

    fn alu_or(&mut self, r: u8) {
        let a = self.cpu.a();
        let value = a | r;
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(false);
        self.cpu.advance_pc();
    }

    fn alu_and(&mut self, r: u8) {
        let a = self.cpu.a();
        let value = a & r;
        self.cpu.set_a(value);
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(true);
        self.cpu.set_carry(false);
        self.cpu.advance_pc();
    }

    fn alu_cp(&mut self, r: u8) {
        let a = self.cpu.a();
        self.cpu.set_zero(a == r);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((a & 0xF) < (r & 0xF));
        self.cpu.set_carry(a < r);
        self.cpu.advance_pc();
    }

    fn ret(&mut self) {
        let value = self.read_word(self.cpu.sp);
        self.cpu.sp = self.cpu.sp.wrapping_add(2);
        self.cpu.pc = value;
    }

    fn ret_cc(&mut self, opcode: u8) {
        if self.get_flag_condition(opcode) {
            self.cpu.pending_cycles = 5;
            self.ret();
        } else {
            self.cpu.advance_pc();
        }
    }

    fn reti(&mut self) {
        self.ret();
        self.cpu.interrupt_master = true;
    }

    fn jp(&mut self) {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        self.cpu.pc = value;
    }

    fn jp_cc(&mut self, opcode: u8) {
        if self.get_flag_condition(opcode) {
            self.cpu.pending_cycles = 4;
            self.jp();
        } else {
            self.cpu.advance_pc();
        }
    }

    fn call_a16(&mut self) {
        self.cpu.sp = self.cpu.sp.wrapping_sub(2);
        self.write_word(self.cpu.sp, self.cpu.pc.wrapping_add(3));
        self.cpu.pc = self.read_word(self.cpu.pc.wrapping_add(1));
    }

    fn call_cc(&mut self, opcode: u8) {
        if self.get_flag_condition(opcode) {
            self.cpu.pending_cycles = 6;
            self.call_a16();
        } else {
            self.cpu.advance_pc();
        }
    }

    fn rst(&mut self, opcode: u8) {
        self.cpu.sp = self.cpu.sp.wrapping_sub(2);
        self.write_word(self.cpu.sp, self.cpu.pc.wrapping_add(1));
        self.cpu.pc = (opcode as u16) ^ 0xC7;
    }

    fn cp_d8(&mut self) {
        let r = self.read_byte(self.cpu.pc.wrapping_add(1));
        let a = self.cpu.a();
        self.cpu.set_zero(a == r);
        self.cpu.set_negative(true);
        self.cpu.set_half_carry((a & 0xF) < (r & 0xF));
        self.cpu.set_carry(a < r);
        self.cpu.advance_pc();
    }

    fn ei(&mut self) {
        self.cpu.ime_toggle = true;
        self.cpu.advance_pc();
    }

    fn di(&mut self) {
        self.cpu.interrupt_master = false;
        self.cpu.advance_pc();
    }

    fn ld_a_da16(&mut self) {
        let address = self.read_word(self.cpu.pc.wrapping_add(1));
        let value = self.read_byte(address);
        self.cpu.set_a(value);
        self.cpu.advance_pc();
    }

    fn ld_sp_hl(&mut self) {
        let hl = self.cpu.hl();
        self.cpu.sp = hl;
        self.cpu.advance_pc();
    }

    fn ld_hl_sp_r8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8 as i16 as u16;
        let sp_r8 = self.cpu.sp.wrapping_add(value);
        self.cpu.set_hl(sp_r8);
        self.cpu.set_zero(false);
        self.cpu.set_negative(false);
        self.cpu
            .set_half_carry((self.cpu.sp & 0xF) + (value & 0xF) > 0xF);
        self.cpu.set_carry((self.cpu.sp & 0xFF) + (value & 0xFF) > 0xFF);
        self.cpu.advance_pc();
    }

    fn add_sp_r8(&mut self) {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8 as i16 as u16;
        let sp_r8 = self.cpu.sp.wrapping_add(value);
        self.cpu.set_zero(false);
        self.cpu.set_negative(false);
        self.cpu
            .set_half_carry((self.cpu.sp & 0xF) + (value & 0xF) > 0xF);
        self.cpu.set_carry((self.cpu.sp & 0xFF) + (value & 0xFF) > 0xFF);
        self.cpu.sp = sp_r8;
        self.cpu.advance_pc();
    }

    fn jp_hl(&mut self) {
        let hl = self.cpu.hl();
        self.cpu.pc = hl;
    }

    fn ld_a_dc(&mut self) {
        let c = self.cpu.c() as u16;
        let address = 0xFF00 | c;
        let value = self.read_byte(address);
        self.cpu.set_a(value);
        self.cpu.advance_pc();
    }

    fn ld_dc_a(&mut self) {
        let c = self.cpu.c() as u16;
        let address = 0xFF00 | c;
        let a = self.cpu.a();
        self.write_byte(address, a);
        self.cpu.advance_pc();
    }

    fn ldh_a_a8(&mut self) {
        let r = self.read_byte(self.cpu.pc.wrapping_add(1)) as u16;
        let address = 0xFF00 | r;
        let value = self.read_byte(address);
        self.cpu.set_a(value);
        self.cpu.advance_pc();
    }

    fn ldh_a8_a(&mut self) {
        let r = self.read_byte(self.cpu.pc.wrapping_add(1)) as u16;
        let address = 0xFF00 | r;
        let a = self.cpu.a();
        self.write_byte(address, a);
        self.cpu.advance_pc();
    }

    fn ld_a16_a(&mut self) {
        let address = self.read_word(self.cpu.pc.wrapping_add(1));
        let a = self.cpu.a();
        self.write_byte(address, a);
        self.cpu.advance_pc();
    }
}


macro_rules! create_alu_ld_word_instructions {
    ($($rr:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<dec_ $rr>](&mut self) {
                        let value = self.cpu.$rr().wrapping_sub(1);
                        self.cpu.[<set_ $rr>](value);
                        self.cpu.advance_pc();
                    }

                    fn [<inc_ $rr>](&mut self) {
                        let value = self.cpu.$rr().wrapping_add(1);
                        self.cpu.[<set_ $rr>](value);
                        self.cpu.advance_pc();
                    }

                    fn [<add_hl_ $rr>](&mut self) {
                        let hl: u16 = self.cpu.hl();
                        let rr: u16 = self.cpu.$rr();
                        self.cpu.set_hl(hl.wrapping_add(rr));
                        self.cpu.set_negative(false);
                        self.cpu.set_half_carry((hl & 0xFFF) + (rr & 0xFFF) & 0x1000 != 0);
                        self.cpu.set_carry((hl as u32 + rr as u32) & 0x10000 != 0);
                        self.cpu.advance_pc();
                    }

                    fn [<ld_ $rr _d16>](&mut self) {
                        let value = self.read_word(self.cpu.pc.wrapping_add(1));
                        self.cpu.[<set_ $rr>](value);
                        self.cpu.advance_pc();
                    }
                )*
            }

            fn add_hl_sp(&mut self) {
                let hl: u16 = self.cpu.hl();
                let rr = self.cpu.sp;
                self.cpu.set_hl(hl.wrapping_add(rr));
                self.cpu.set_negative(false);
                self.cpu.set_half_carry((hl & 0xFFF) + (rr & 0xFFF) & 0x1000 != 0);
                self.cpu.set_carry((hl as u32 + rr as u32) & 0x10000 != 0);
                self.cpu.advance_pc();
            }
        }
    }
}


macro_rules! create_byte_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    #[allow(dead_code)]
                    fn [<ld_a_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_a(value);
                        self.cpu.advance_pc();
                    }

                    #[allow(dead_code)]
                    fn [<ld_b_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_b(value);
                        self.cpu.advance_pc();
                    }

                    #[allow(dead_code)]
                    fn [<ld_c_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_c(value);
                        self.cpu.advance_pc();
                    }

                    #[allow(dead_code)]
                    fn [<ld_d_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_d(value);
                        self.cpu.advance_pc();
                    }

                    #[allow(dead_code)]
                    fn [<ld_e_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_e(value);
                        self.cpu.advance_pc();
                    }

                    #[allow(dead_code)]
                    fn [<ld_h_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_h(value);
                        self.cpu.advance_pc();
                    }

                    #[allow(dead_code)]
                    fn [<ld_l_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        self.cpu.set_l(value);
                        self.cpu.advance_pc();
                    }

                    fn [<ld_ $r _dhl>](&mut self) {
                        let hl = self.cpu.hl();
                        let value = self.read_byte(hl);
                        self.cpu.[<set_ $r>](value);
                        self.cpu.advance_pc();
                    }

                    fn [<ld_dhl_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let hl = self.cpu.hl();
                        self.write_byte(hl, value);
                        self.cpu.advance_pc();
                    }

                    fn [<ld_ $r _d8>](&mut self) {
                        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
                        self.cpu.[<set_ $r>](value);
                        self.cpu.advance_pc();
                    }

                    fn [<inc_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let value = r.wrapping_add(1);
                        self.cpu.[<set_ $r>](value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(false);
                        self.cpu.set_half_carry((r & 0x0F) + 1 > 0x0F);
                        self.cpu.advance_pc();
                    }

                    fn [<dec_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        let value = r.wrapping_sub(1);
                        self.cpu.[<set_ $r>](value);
                        self.cpu.set_zero(value == 0);
                        self.cpu.set_negative(true);
                        self.cpu.set_half_carry((r & 0xF) == 0);
                        self.cpu.advance_pc();
                    }

                    fn [<add_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_add(r);
                    }

                    fn [<adc_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_adc(r);
                    }

                    fn [<sub_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_sub(r);
                    }

                    fn [<sbc_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_sbc(r);
                    }

                    fn [<and_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_and(r);
                    }

                    fn [<xor_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_xor(r);
                    }

                    fn [<or_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_or(r);
                    }

                    fn [<cp_ $r>](&mut self) {
                        let r = self.cpu.$r();
                        self.alu_cp(r);
                    }
                )*
            }
        }
    }
}

macro_rules! create_push_pop_instructions {
    ($($rr:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<pop_ $rr>](&mut self) {
                        let value = self.read_word(self.cpu.sp);
                        self.cpu.sp = self.cpu.sp.wrapping_add(2);
                        self.cpu.[<set_ $rr>](value);
                        self.cpu.advance_pc();
                    }

                    fn [<push_ $rr>](&mut self) {
                        let r = self.cpu.$rr();
                        self.cpu.sp = self.cpu.sp.wrapping_sub(2);
                        self.write_word(self.cpu.sp, r);
                        self.cpu.advance_pc();
                    }
                )*
            }
        }
    }
}

macro_rules! create_ld_rra_instructions {
    ($($rr:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<ld_ $rr _a>](&mut self) {
                        let a = self.cpu.a();
                        let address = self.cpu.$rr();
                        self.write_byte(address, a);
                        self.cpu.advance_pc();
                    }

                    fn [<ld_a_ $rr>](&mut self) {
                        let r = self.cpu.$rr();
                        let value = self.read_byte(r);
                        self.cpu.set_a(value);
                        self.cpu.advance_pc();
                    }
                )*
            }
        }
    }
}

macro_rules! create_alu_d8_instructions {
    ($($alu:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<$alu _a_d8>](&mut self) {
                        let d8 = self.read_byte(self.cpu.pc.wrapping_add(1));
                        self.[<alu_ $alu>](d8);
                    }
                )*
            }
        }
    }
}

macro_rules! create_alu_dhl_instructions {
    ($($alu:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<$alu _dhl>](&mut self) {
                        let hl = self.cpu.hl();
                        let r = self.read_byte(hl);
                        self.[<alu_ $alu>](r);
                    }
                )*
            }
        }
    }
}


create_alu_ld_word_instructions!(bc, de, hl);
create_byte_instructions!(a, b, c, d, e, h, l);
create_push_pop_instructions!(af, bc, de, hl);
create_ld_rra_instructions!(bc, de);
create_alu_d8_instructions!(or, xor, and, add, adc, sub, sbc);
create_alu_dhl_instructions!(or, xor, and, add, adc, sub, sbc, cp);


#[cfg(test)]
mod test {
    use super::*;
    use crate::core::gb::{Gb, GbTypes};
    use crate::core::mbc::{load_cartridge};

    fn setup_gb() -> Gb {
        let (mbc, gb_mode) = load_cartridge("roms/cpu_instrs.gb").unwrap();
        let gb_type = GbTypes::Dmg;
        Gb::new(mbc, gb_mode, gb_type)
    }

    macro_rules! test_ld_instructions {
        ($($r:ident),*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ld_a_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_a_ $r>]();
                            let a = gb.cpu.a();
                            assert_eq!(a, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }

                    #[test]
                    fn [<test_ld_b_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_b_ $r>]();
                            let b = gb.cpu.b();
                            assert_eq!(b, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }

                    #[test]
                    fn [<test_ld_c_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_c_ $r>]();
                            let c = gb.cpu.c();
                            assert_eq!(c, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }

                    #[test]
                    fn [<test_ld_d_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_d_ $r>]();
                            let d = gb.cpu.d();
                            assert_eq!(d, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }

                    #[test]
                    fn [<test_ld_e_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_e_ $r>]();
                            let e = gb.cpu.e();
                            assert_eq!(e, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }

                    #[test]
                    fn [<test_ld_h_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_h_ $r>]();
                            let h = gb.cpu.h();
                            assert_eq!(h, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }

                    #[test]
                    fn [<test_ld_l_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_l_ $r>]();
                            let l = gb.cpu.l();
                            assert_eq!(l, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }

                    #[test]
                    fn [<test_ld_ $r _dhl>]() {
                        let mut gb = setup_gb();
                        let values: [u16; (u16::MAX as usize + 1) as usize] = (0..=u16::MAX).collect::<Vec<u16>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.set_hl(*value);
                            let v = gb.read_byte(*value);
                            gb.[<ld_ $r _dhl>]();
                            let r = gb.cpu.$r();
                            assert_eq!(r, v);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }
                )*
            }
        };
    }

    macro_rules! test_ld_dhl_instructions {
        ($($r:ident),*) => {
            paste! {
                $(
                    #[test]
                    fn [<test_ld_dhl_ $r>]() {
                        let mut gb = setup_gb();
                        let values: [u8; 256] = (0..=255).collect::<Vec<u8>>().try_into().unwrap();
                        for value in values.iter() {
                            let pc = gb.cpu.pc;
                            gb.cpu.set_hl(0xC000);
                            gb.cpu.[<set_ $r>](*value);
                            gb.[<ld_dhl_ $r>]();
                            let v = gb.read_byte(0xC000);
                            assert_eq!(v, *value);
                            assert_eq!(gb.cpu.pc, pc.wrapping_add(1));
                        }
                    }
                )*
            }
        };
    }

    test_ld_instructions!(a, b, c, d, e, h, l);
    test_ld_dhl_instructions!(a, b, c, d, e);
}
