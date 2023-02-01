use crate::core::gb::Gb;

// DECODE CB PREFIXED INSTRUCTIONS
impl Gb {
    pub fn decode_cb_prefix(&mut self) {
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
