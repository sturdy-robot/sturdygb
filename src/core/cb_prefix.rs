// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use paste::paste;

use crate::core::gb::Gb;

//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
#[rustfmt::skip]
const CB_CYCLES: [usize; 256] = [
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2,
];

// DECODE CB PREFIXED INSTRUCTIONS
impl Gb {
    pub fn decode_cb_prefix(&mut self) {
        let prefix = self.read_byte(self.cpu.pc.wrapping_add(1));
        self.cpu.instruction_cycles = CB_CYCLES[prefix as usize];
        match prefix {
            0x00 => self.rlc_b(),
            0x01 => self.rlc_c(),
            0x02 => self.rlc_d(),
            0x03 => self.rlc_e(),
            0x04 => self.rlc_h(),
            0x05 => self.rlc_l(),
            0x06 => self.rlc_dhl(),
            0x07 => self.rlc_a(),
            0x08 => self.rrc_b(),
            0x09 => self.rrc_c(),
            0x0A => self.rrc_d(),
            0x0B => self.rrc_e(),
            0x0C => self.rrc_h(),
            0x0D => self.rrc_l(),
            0x0E => self.rrc_dhl(),
            0x0F => self.rrc_a(),
            0x10 => self.rl_b(),
            0x11 => self.rl_c(),
            0x12 => self.rl_d(),
            0x13 => self.rl_e(),
            0x14 => self.rl_h(),
            0x15 => self.rl_l(),
            0x16 => self.rl_dhl(),
            0x17 => self.rl_a(),
            0x18 => self.rr_b(),
            0x19 => self.rr_c(),
            0x1A => self.rr_d(),
            0x1B => self.rr_e(),
            0x1C => self.rr_h(),
            0x1D => self.rr_l(),
            0x1E => self.rr_dhl(),
            0x1F => self.rr_a(),
            0x20 => self.sla_b(),
            0x21 => self.sla_c(),
            0x22 => self.sla_d(),
            0x23 => self.sla_e(),
            0x24 => self.sla_h(),
            0x25 => self.sla_l(),
            0x26 => self.sla_dhl(),
            0x27 => self.sla_a(),
            0x28 => self.sra_b(),
            0x29 => self.sra_c(),
            0x2A => self.sra_d(),
            0x2B => self.sra_e(),
            0x2C => self.sra_h(),
            0x2D => self.sra_l(),
            0x2E => self.sra_dhl(),
            0x2F => self.sra_a(),
            0x30 => self.swap_b(),
            0x31 => self.swap_c(),
            0x32 => self.swap_d(),
            0x33 => self.swap_e(),
            0x34 => self.swap_h(),
            0x35 => self.swap_l(),
            0x36 => self.swap_dhl(),
            0x37 => self.swap_a(),
            0x38 => self.srl_b(),
            0x39 => self.srl_c(),
            0x3A => self.srl_d(),
            0x3B => self.srl_e(),
            0x3C => self.srl_h(),
            0x3D => self.srl_l(),
            0x3E => self.srl_dhl(),
            0x3F => self.srl_a(),
            0x40 => self.bit_b(0),
            0x41 => self.bit_c(0),
            0x42 => self.bit_d(0),
            0x43 => self.bit_e(0),
            0x44 => self.bit_h(0),
            0x45 => self.bit_l(0),
            0x46 => self.bit_dhl(0),
            0x47 => self.bit_a(0),
            0x48 => self.bit_b(1),
            0x49 => self.bit_c(1),
            0x4A => self.bit_d(1),
            0x4B => self.bit_e(1),
            0x4C => self.bit_h(1),
            0x4D => self.bit_l(1),
            0x4E => self.bit_dhl(1),
            0x4F => self.bit_a(1),
            0x50 => self.bit_b(2),
            0x51 => self.bit_c(2),
            0x52 => self.bit_d(2),
            0x53 => self.bit_e(2),
            0x54 => self.bit_h(2),
            0x55 => self.bit_l(2),
            0x56 => self.bit_dhl(2),
            0x57 => self.bit_a(2),
            0x58 => self.bit_b(3),
            0x59 => self.bit_c(3),
            0x5A => self.bit_d(3),
            0x5B => self.bit_e(3),
            0x5C => self.bit_h(3),
            0x5D => self.bit_l(3),
            0x5E => self.bit_dhl(3),
            0x5F => self.bit_a(3),
            0x60 => self.bit_b(4),
            0x61 => self.bit_c(4),
            0x62 => self.bit_d(4),
            0x63 => self.bit_e(4),
            0x64 => self.bit_h(4),
            0x65 => self.bit_l(4),
            0x66 => self.bit_dhl(4),
            0x67 => self.bit_a(4),
            0x68 => self.bit_b(5),
            0x69 => self.bit_c(5),
            0x6A => self.bit_d(5),
            0x6B => self.bit_e(5),
            0x6C => self.bit_h(5),
            0x6D => self.bit_l(5),
            0x6E => self.bit_dhl(5),
            0x6F => self.bit_a(5),
            0x70 => self.bit_b(6),
            0x71 => self.bit_c(6),
            0x72 => self.bit_d(6),
            0x73 => self.bit_e(6),
            0x74 => self.bit_h(6),
            0x75 => self.bit_l(6),
            0x76 => self.bit_dhl(6),
            0x77 => self.bit_a(6),
            0x78 => self.bit_b(7),
            0x79 => self.bit_c(7),
            0x7A => self.bit_d(7),
            0x7B => self.bit_e(7),
            0x7C => self.bit_h(7),
            0x7D => self.bit_l(7),
            0x7E => self.bit_dhl(7),
            0x7F => self.bit_a(7),
            0x80 => self.res_b(0),
            0x81 => self.res_c(0),
            0x82 => self.res_d(0),
            0x83 => self.res_e(0),
            0x84 => self.res_h(0),
            0x85 => self.res_l(0),
            0x86 => self.res_dhl(0),
            0x87 => self.res_a(0),
            0x88 => self.res_b(1),
            0x89 => self.res_c(1),
            0x8A => self.res_d(1),
            0x8B => self.res_e(1),
            0x8C => self.res_h(1),
            0x8D => self.res_l(1),
            0x8E => self.res_dhl(1),
            0x8F => self.res_a(1),
            0x90 => self.res_b(2),
            0x91 => self.res_c(2),
            0x92 => self.res_d(2),
            0x93 => self.res_e(2),
            0x94 => self.res_h(2),
            0x95 => self.res_l(2),
            0x96 => self.res_dhl(2),
            0x97 => self.res_a(2),
            0x98 => self.res_b(3),
            0x99 => self.res_c(3),
            0x9A => self.res_d(3),
            0x9B => self.res_e(3),
            0x9C => self.res_h(3),
            0x9D => self.res_l(3),
            0x9E => self.res_dhl(3),
            0x9F => self.res_a(3),
            0xA0 => self.res_b(4),
            0xA1 => self.res_c(4),
            0xA2 => self.res_d(4),
            0xA3 => self.res_e(4),
            0xA4 => self.res_h(4),
            0xA5 => self.res_l(4),
            0xA6 => self.res_dhl(4),
            0xA7 => self.res_a(4),
            0xA8 => self.res_b(5),
            0xA9 => self.res_c(5),
            0xAA => self.res_d(5),
            0xAB => self.res_e(5),
            0xAC => self.res_h(5),
            0xAD => self.res_l(5),
            0xAE => self.res_dhl(5),
            0xAF => self.res_a(5),
            0xB0 => self.res_b(6),
            0xB1 => self.res_c(6),
            0xB2 => self.res_d(6),
            0xB3 => self.res_e(6),
            0xB4 => self.res_h(6),
            0xB5 => self.res_l(6),
            0xB6 => self.res_dhl(6),
            0xB7 => self.res_a(6),
            0xB8 => self.res_b(7),
            0xB9 => self.res_c(7),
            0xBA => self.res_d(7),
            0xBB => self.res_e(7),
            0xBC => self.res_h(7),
            0xBD => self.res_l(7),
            0xBE => self.res_dhl(7),
            0xBF => self.res_a(7),
            0xC0 => self.set_b(0),
            0xC1 => self.set_c(0),
            0xC2 => self.set_d(0),
            0xC3 => self.set_e(0),
            0xC4 => self.set_h(0),
            0xC5 => self.set_l(0),
            0xC6 => self.set_dhl(0),
            0xC7 => self.set_a(0),
            0xC8 => self.set_b(1),
            0xC9 => self.set_c(1),
            0xCA => self.set_d(1),
            0xCB => self.set_e(1),
            0xCC => self.set_h(1),
            0xCD => self.set_l(1),
            0xCE => self.set_dhl(1),
            0xCF => self.set_a(1),
            0xD0 => self.set_b(2),
            0xD1 => self.set_c(2),
            0xD2 => self.set_d(2),
            0xD3 => self.set_e(2),
            0xD4 => self.set_h(2),
            0xD5 => self.set_l(2),
            0xD6 => self.set_dhl(2),
            0xD7 => self.set_a(2),
            0xD8 => self.set_b(3),
            0xD9 => self.set_c(3),
            0xDA => self.set_d(3),
            0xDB => self.set_e(3),
            0xDC => self.set_h(3),
            0xDD => self.set_l(3),
            0xDE => self.set_dhl(3),
            0xDF => self.set_a(3),
            0xE0 => self.set_b(4),
            0xE1 => self.set_c(4),
            0xE2 => self.set_d(4),
            0xE3 => self.set_e(4),
            0xE4 => self.set_h(4),
            0xE5 => self.set_l(4),
            0xE6 => self.set_dhl(4),
            0xE7 => self.set_a(4),
            0xE8 => self.set_b(5),
            0xE9 => self.set_c(5),
            0xEA => self.set_d(5),
            0xEB => self.set_e(5),
            0xEC => self.set_h(5),
            0xED => self.set_l(5),
            0xEE => self.set_dhl(5),
            0xEF => self.set_a(5),
            0xF0 => self.set_b(6),
            0xF1 => self.set_c(6),
            0xF2 => self.set_d(6),
            0xF3 => self.set_e(6),
            0xF4 => self.set_h(6),
            0xF5 => self.set_l(6),
            0xF6 => self.set_dhl(6),
            0xF7 => self.set_a(6),
            0xF8 => self.set_b(7),
            0xF9 => self.set_c(7),
            0xFA => self.set_d(7),
            0xFB => self.set_e(7),
            0xFC => self.set_h(7),
            0xFD => self.set_l(7),
            0xFE => self.set_dhl(7),
            0xFF => self.set_a(7),
        };
        self.cpu.advance_pc();
    }

    fn rlc_common(&mut self, value: u8) -> u8 {
        let carry = value & 0x80 == 0x80;
        let r = (value << 1) | (if carry { 1 } else { 0 });
        self.change_flags(r, carry);
        r
    }

    fn change_flags(&mut self, r: u8, carry: bool) {
        self.cpu.set_zero(r == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(carry);
    }

    fn rl_common(&mut self, value: u8) -> u8 {
        let carry = value & 0x80 == 0x80;
        let r = (value << 1) | (self.cpu.get_carry());
        self.change_flags(r, carry);
        r
    }

    fn rrc_common(&mut self, value: u8) -> u8 {
        let carry = value & 0x01 == 0x01;
        let r = (value >> 1) | (if carry { 0x80 } else { 0x00 });
        self.change_flags(r, carry);
        r
    }

    fn rr_common(&mut self, value: u8) -> u8 {
        let carry = value & 0x01 == 0x01;
        let r = (value >> 1) | if self.cpu.carry { 0x80 } else { 0 };
        self.change_flags(r, carry);
        r
    }

    fn sla_common(&mut self, value: u8) -> u8 {
        let carry = value & 0x80 == 0x80;
        let r = value << 1;
        self.change_flags(r, carry);
        r
    }

    fn sra_common(&mut self, value: u8) -> u8 {
        let carry = value & 0x01 == 0x01;
        let r = (value >> 1) | (value & 0x80);
        self.change_flags(r, carry);
        r
    }

    fn srl_common(&mut self, value: u8) -> u8 {
        let carry = value & 0x01 == 0x01;
        let r = value >> 1;
        self.change_flags(r, carry);
        r
    }

    fn swap_common(&mut self, value: u8) -> u8 {
        self.cpu.set_zero(value == 0);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(false);
        self.cpu.set_carry(false);

        (value >> 4) | (value << 4)
    }

    fn bit_common(&mut self, value: u8, bit: u8) {
        let result = value & (1 << (bit as u32)) == 0;

        self.cpu.set_zero(result);
        self.cpu.set_negative(false);
        self.cpu.set_half_carry(true);
    }

    fn rlc_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.rlc_common(value);
        self.write_byte(hl, r);
    }

    fn rl_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.rl_common(value);
        self.write_byte(hl, r);
    }

    fn rrc_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.rrc_common(value);
        self.write_byte(hl, r);
    }

    fn rr_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.rr_common(value);
        self.write_byte(hl, r);
    }

    fn sla_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.sla_common(value);
        self.write_byte(hl, r);
    }

    fn sra_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.sra_common(value);
        self.write_byte(hl, r);
    }

    fn swap_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.swap_common(value);
        self.write_byte(hl, r);
    }

    fn srl_dhl(&mut self) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = self.srl_common(value);
        self.write_byte(hl, r);
    }

    fn bit_dhl(&mut self, bit: u8) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        self.bit_common(value, bit);
    }

    fn res_dhl(&mut self, bit: u8) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = value & !(1 << bit);
        self.write_byte(hl, r);
    }

    fn set_dhl(&mut self, bit: u8) {
        let hl = self.cpu.hl();
        let value = self.read_byte(hl);
        let r = value | (1 << bit);
        self.write_byte(hl, r);
    }
}


macro_rules! create_cb_prefix_instructions {
    ($($r:ident),*) => {
        impl Gb {
            paste! {
                $(
                    fn [<rlc_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let r = self.rlc_common(value);
                        self.cpu.[<set_ $r>](r);
                    }

                    fn [<rl_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let r = self.rl_common(value);
                        self.cpu.[<set_ $r>](r);
                    }


                    fn [<rrc_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let r = self.rrc_common(value);
                        self.cpu.[<set_ $r>](r);
                    }

                    fn [<rr_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let r = self.rr_common(value);
                        self.cpu.[<set_ $r>](r);
                    }

                    fn [<sla_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let r = self.sla_common(value);
                        self.cpu.[<set_ $r>](r);
                    }

                    fn [<sra_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let r = self.sra_common(value);
                        self.cpu.[<set_ $r>](r);
                    }

                    fn [<srl_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let r = self.srl_common(value);
                        self.cpu.[<set_ $r>](r);
                    }

                    fn [<swap_ $r>](&mut self) {
                        let value = self.cpu.$r();
                        let result = self.swap_common(value);
                        self.cpu.[<set_ $r>](result);
                    }

                    fn [<bit_ $r>](&mut self, bit: u8) {
                        let value = self.cpu.$r();
                        self.bit_common(value, bit);
                    }

                    fn [<set_ $r>](&mut self, bit: u8) {
                        let value = self.cpu.$r();
                        let result = value | (1 << bit);
                        self.cpu.[<set_ $r>](result);
                    }

                    fn [<res_ $r>](&mut self, bit: u8) {
                        let value = self.cpu.$r();
                        let result = value & !(1 << bit);
                        self.cpu.[<set_ $r>](result);
                    }
                )*
            }
        }
    }
}

create_cb_prefix_instructions!(a, b, c, d, e, h, l);