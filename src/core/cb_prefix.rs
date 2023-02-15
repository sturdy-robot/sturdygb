// Totally inspired on SameBoy's decoding prefix algorithm
use crate::core::gb::Gb;

//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
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
        self.cpu.pending_cycles = CB_CYCLES[prefix as usize];
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
        self.cpu.advance_pc();
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
