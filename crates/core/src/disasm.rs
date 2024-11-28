// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::gb::Gb;

// Register name constants for better maintainability and consistency
const REGISTER_NAMES: [&str; 8] = ["b", "c", "d", "e", "h", "l", "a", "f"];
const REGISTER_PAIRS: [&str; 4] = ["bc", "de", "hl", "sp"];

/// Formats a byte value as a hexadecimal string with proper padding
fn format_byte(value: u8) -> String {
    format!("{:02x}h", value)
}

/// Formats a word value as a hexadecimal string with proper padding
fn format_word(value: u16) -> String {
    format!("{:04x}h", value)
}

impl Gb {
    /// Disassembles a load instruction with an immediate word value
    fn d_ld_rr_nn(&self, target: &str) -> String {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        format!("ld {}, {}", target, format_word(value))
    }

    /// Disassembles a load instruction that adds SP to an immediate byte
    fn d_ld_hl_sp_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8;
        format!("ld hl, sp + {}", format_byte(value as u8))
    }

    /// Disassembles a store SP to memory instruction
    fn d_ld_nn_sp(&self) -> String {
        let addr = self.read_word(self.cpu.pc.wrapping_add(1));
        format!("ld [{}], sp", format_word(addr))
    }

    /// Disassembles a load immediate byte to register instruction
    fn d_ld_r_n(&self, target: &str) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("ld {}, {}", target, format_byte(value))
    }

    /// Disassembles a load register to register instruction
    fn d_ld_r_r(&self, target: &str, value: &str) -> String {
        format!("ld {}, {}", target, value)
    }

    /// Disassembles a load immediate word to register pair instruction
    fn d_ld_nn_rr(&self, target: &str) -> String {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        format!("ld {}, {}", format_word(value), target)
    }

    /// Disassembles a load high memory instruction with register
    fn d_ldh_n_r(&self, target: &str) -> String {
        let addr = 0xFF00 | self.read_byte(self.cpu.pc.wrapping_add(1)) as u16;
        format!("ldh [{}], {}", format_word(addr), target)
    }

    /// Disassembles a load register from high memory instruction
    fn d_ldh_r_n(&self, target: &str) -> String {
        let addr = 0xFF00 | self.read_byte(self.cpu.pc.wrapping_add(1)) as u16;
        format!("ldh {}, [{}]", target, format_word(addr))
    }

    /// Disassembles a push register pair instruction
    fn d_push_nn(&self, target: &str) -> String {
        format!("push {}", target)
    }

    /// Disassembles a pop register pair instruction
    fn d_pop_nn(&self, target: &str) -> String {
        format!("pop {}", target)
    }

    /// Disassembles an increment register instruction
    fn d_inc(&self, target: &str) -> String {
        format!("inc {}", target)
    }

    /// Disassembles a decrement register instruction
    fn d_dec(&self, target: &str) -> String {
        format!("dec {}", target)
    }

    /// Disassembles an add HL to register pair instruction
    fn d_add_hl_r(&self, target: &str) -> String {
        format!("add hl, {}", target)
    }

    /// Disassembles an add SP to immediate byte instruction
    fn d_add_sp_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8;
        format!("add sp, {}", format_byte(value as u8))
    }

    /// Disassembles an add A to immediate byte instruction
    fn d_add_a_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("add a, {}", format_byte(value))
    }

    /// Disassembles an add A to register instruction
    fn d_add_a_r(&self, value: &str) -> String {
        format!("add a, {}", value)
    }

    /// Disassembles an ADC A to immediate byte instruction
    fn d_adc_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("adc {}", format_byte(value))
    }

    /// Disassembles an ADC A to register instruction
    fn d_adc_r(&self, value: &str) -> String {
        format!("adc {}", value)
    }

    /// Disassembles a SUB register instruction
    fn d_sub_r(&self, value: &str) -> String {
        format!("sub {}", value)
    }

    /// Disassembles a SUB immediate byte instruction
    fn d_sub_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("sub {}", format_byte(value))
    }

    /// Disassembles a SBC register instruction
    fn d_sbc_r(&self, value: &str) -> String {
        format!("sbc {}", value)
    }

    /// Disassembles a SBC immediate byte instruction
    fn d_sbc_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("sbc {}", format_byte(value))
    }

    /// Disassembles an AND register instruction
    fn d_and_r(&self, value: &str) -> String {
        format!("and {}", value)
    }

    /// Disassembles an AND immediate byte instruction
    fn d_and_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("and {}", format_byte(value))
    }

    /// Disassembles an OR register instruction
    fn d_or_r(&self, value: &str) -> String {
        format!("or {}", value)
    }

    /// Disassembles an OR immediate byte instruction
    fn d_or_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("or {}", format_byte(value))
    }

    /// Disassembles an XOR register instruction
    fn d_xor_r(&self, value: &str) -> String {
        format!("xor {}", value)
    }

    /// Disassembles an XOR immediate byte instruction
    fn d_xor_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("xor {}", format_byte(value))
    }

    /// Disassembles a CP register instruction
    fn d_cp_r(&self, value: &str) -> String {
        format!("cp {}", value)
    }

    /// Disassembles a CP immediate byte instruction
    fn d_cp_n(&self) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1));
        format!("cp {}", format_byte(value))
    }

    /// Disassembles a JP immediate word instruction
    fn d_jp_nn(&self) -> String {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        format!("jp {}", format_word(value))
    }

    /// Disassembles a JP flag immediate word instruction
    fn d_jp_f_nn(&self, flag: &str) -> String {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        format!("jp {}, {}", flag, format_word(value))
    }

    /// Disassembles a JR immediate byte instruction
    fn d_jr_n(&self) -> String {
        let value = ((self.cpu.pc as u32 as i32)
            + (self.read_byte(self.cpu.pc.wrapping_add(1)) as i8 as i32))
            as u16;
        format!("jr {}", format_word(value))
    }

    /// Disassembles a JR flag immediate byte instruction
    fn d_jr_f_n(&self, flag: &str) -> String {
        let value = self.read_byte(self.cpu.pc.wrapping_add(1)) as i8;
        format!("jr {}, {}", flag, format_byte(value as u8))
    }

    /// Disassembles a CALL flag immediate word instruction
    fn d_call_f_nn(&self, flag: &str) -> String {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        format!("call {}, {}", flag, format_word(value))
    }

    /// Disassembles a CALL immediate word instruction
    fn d_call_nn(&self) -> String {
        let value = self.read_word(self.cpu.pc.wrapping_add(1));
        format!("call {}", format_word(value))
    }

    /// Disassembles the current instruction
    pub fn disassemble(&self) -> String {
        match self.cpu.current_instruction {
            0x00 => "nop".to_string(),
            0x01 => self.d_ld_rr_nn("bc"),
            0x02 => self.d_ld_r_r("(bc)", "a"),
            0x03 => self.d_inc("bc"),
            0x04 => self.d_inc("b"),
            0x05 => self.d_dec("b"),
            0x06 => self.d_ld_r_n("b"),
            0x07 => "rlca".to_string(),
            0x08 => self.d_ld_nn_sp(),
            0x09 => self.d_add_hl_r("bc"),
            0x0A => self.d_ld_r_r("a", "(bc)"),
            0x0B => self.d_dec("bc"),
            0x0C => self.d_inc("c"),
            0x0D => self.d_dec("c"),
            0x0E => self.d_ld_r_n("c"),
            0x0F => "rrca".to_string(),
            0x10 => "stop".to_string(),
            0x11 => self.d_ld_rr_nn("de"),
            0x12 => self.d_ld_r_r("(de)", "a"),
            0x13 => self.d_inc("de"),
            0x14 => self.d_inc("d"),
            0x15 => self.d_dec("d"),
            0x16 => self.d_ld_r_n("d"),
            0x17 => "rla".to_string(),
            0x18 => self.d_jr_n(),
            0x19 => self.d_add_hl_r("de"),
            0x1A => self.d_ld_r_r("a", "(de)"),
            0x1B => self.d_dec("de"),
            0x1C => self.d_inc("e"),
            0x1D => self.d_dec("e"),
            0x1E => self.d_ld_r_n("e"),
            0x1F => "rra".to_string(),
            0x20 => self.d_jr_f_n("nz"),
            0x21 => self.d_ld_rr_nn("hl"),
            0x22 => self.d_ld_r_r("(hl+)", "a"),
            0x23 => self.d_inc("hl"),
            0x24 => self.d_inc("h"),
            0x25 => self.d_dec("h"),
            0x26 => self.d_ld_r_n("h"),
            0x27 => "daa".to_string(),
            0x28 => self.d_jr_f_n("z"),
            0x29 => self.d_add_hl_r("hl"),
            0x2A => self.d_ld_r_r("a", "(hl+)"),
            0x2B => self.d_dec("hl"),
            0x2C => self.d_inc("l"),
            0x2D => self.d_dec("l"),
            0x2E => self.d_ld_r_n("l"),
            0x2F => "cpl".to_string(),
            0x30 => self.d_jr_f_n("nc"),
            0x31 => self.d_ld_rr_nn("sp"),
            0x32 => self.d_ld_r_r("(hl-)", "a"),
            0x33 => self.d_inc("sp"),
            0x34 => self.d_inc("(hl)"),
            0x35 => self.d_dec("(hl)"),
            0x36 => self.d_ld_r_n("(hl)"),
            0x37 => "scf".to_string(),
            0x38 => self.d_jr_f_n("c"),
            0x39 => self.d_add_hl_r("sp"),
            0x3A => self.d_ld_r_r("a", "(hl-)"),
            0x3B => self.d_dec("sp"),
            0x3C => self.d_inc("a"),
            0x3D => self.d_dec("a"),
            0x3E => self.d_ld_r_n("a"),
            0x3F => "ccf".to_string(),
            0x40 => self.d_ld_r_r("b", "b"),
            0x41 => self.d_ld_r_r("b", "c"),
            0x42 => self.d_ld_r_r("b", "d"),
            0x43 => self.d_ld_r_r("b", "e"),
            0x44 => self.d_ld_r_r("b", "h"),
            0x45 => self.d_ld_r_r("b", "l"),
            0x46 => self.d_ld_r_r("b", "(hl)"),
            0x47 => self.d_ld_r_r("b", "a"),
            0x48 => self.d_ld_r_r("c", "b"),
            0x49 => self.d_ld_r_r("c", "c"),
            0x4A => self.d_ld_r_r("c", "d"),
            0x4B => self.d_ld_r_r("c", "e"),
            0x4C => self.d_ld_r_r("c", "h"),
            0x4D => self.d_ld_r_r("c", "l"),
            0x4E => self.d_ld_r_r("c", "(hl)"),
            0x4F => self.d_ld_r_r("c", "a"),
            0x50 => self.d_ld_r_r("d", "b"),
            0x51 => self.d_ld_r_r("d", "c"),
            0x52 => self.d_ld_r_r("d", "d"),
            0x53 => self.d_ld_r_r("d", "e"),
            0x54 => self.d_ld_r_r("d", "h"),
            0x55 => self.d_ld_r_r("d", "l"),
            0x56 => self.d_ld_r_r("d", "(hl)"),
            0x57 => self.d_ld_r_r("d", "a"),
            0x58 => self.d_ld_r_r("e", "b"),
            0x59 => self.d_ld_r_r("e", "c"),
            0x5A => self.d_ld_r_r("e", "d"),
            0x5B => self.d_ld_r_r("e", "e"),
            0x5C => self.d_ld_r_r("e", "h"),
            0x5D => self.d_ld_r_r("e", "l"),
            0x5E => self.d_ld_r_r("e", "(hl)"),
            0x5F => self.d_ld_r_r("e", "a"),
            0x60 => self.d_ld_r_r("h", "b"),
            0x61 => self.d_ld_r_r("h", "c"),
            0x62 => self.d_ld_r_r("h", "d"),
            0x63 => self.d_ld_r_r("h", "e"),
            0x64 => self.d_ld_r_r("h", "h"),
            0x65 => self.d_ld_r_r("h", "l"),
            0x66 => self.d_ld_r_r("h", "(hl)"),
            0x67 => self.d_ld_r_r("h", "a"),
            0x68 => self.d_ld_r_r("l", "b"),
            0x69 => self.d_ld_r_r("l", "c"),
            0x6A => self.d_ld_r_r("l", "d"),
            0x6B => self.d_ld_r_r("l", "e"),
            0x6C => self.d_ld_r_r("l", "h"),
            0x6D => self.d_ld_r_r("l", "l"),
            0x6E => self.d_ld_r_r("l", "(hl)"),
            0x6F => self.d_ld_r_r("l", "a"),
            0x70 => self.d_ld_r_r("(hl)", "b"),
            0x71 => self.d_ld_r_r("(hl)", "c"),
            0x72 => self.d_ld_r_r("(hl)", "d"),
            0x73 => self.d_ld_r_r("(hl)", "e"),
            0x74 => self.d_ld_r_r("(hl)", "h"),
            0x75 => self.d_ld_r_r("(hl)", "l"),
            0x76 => "halt".to_string(),
            0x77 => self.d_ld_r_r("(hl)", "a"),
            0x78 => self.d_ld_r_r("a", "b"),
            0x79 => self.d_ld_r_r("a", "c"),
            0x7A => self.d_ld_r_r("a", "d"),
            0x7B => self.d_ld_r_r("a", "e"),
            0x7C => self.d_ld_r_r("a", "h"),
            0x7D => self.d_ld_r_r("a", "l"),
            0x7E => self.d_ld_r_r("a", "(hl)"),
            0x7F => self.d_ld_r_r("a", "a"),
            0x80 => self.d_add_a_r("b"),
            0x81 => self.d_add_a_r("c"),
            0x82 => self.d_add_a_r("d"),
            0x83 => self.d_add_a_r("e"),
            0x84 => self.d_add_a_r("h"),
            0x85 => self.d_add_a_r("l"),
            0x86 => self.d_add_a_r("(hl)"),
            0x87 => self.d_add_a_r("a"),
            0x88 => self.d_adc_r("b"),
            0x89 => self.d_adc_r("c"),
            0x8A => self.d_adc_r("d"),
            0x8B => self.d_adc_r("e"),
            0x8C => self.d_adc_r("h"),
            0x8D => self.d_adc_r("l"),
            0x8E => self.d_adc_r("(hl)"),
            0x8F => self.d_adc_r("a"),
            0x90 => self.d_sub_r("b"),
            0x91 => self.d_sub_r("c"),
            0x92 => self.d_sub_r("d"),
            0x93 => self.d_sub_r("e"),
            0x94 => self.d_sub_r("h"),
            0x95 => self.d_sub_r("l"),
            0x96 => self.d_sub_r("(hl)"),
            0x97 => self.d_sub_r("a"),
            0x98 => self.d_sbc_r("b"),
            0x99 => self.d_sbc_r("c"),
            0x9A => self.d_sbc_r("d"),
            0x9B => self.d_sbc_r("e"),
            0x9C => self.d_sbc_r("h"),
            0x9D => self.d_sbc_r("l"),
            0x9E => self.d_sbc_r("(hl)"),
            0x9F => self.d_sbc_r("a"),
            0xA0 => self.d_and_r("b"),
            0xA1 => self.d_and_r("c"),
            0xA2 => self.d_and_r("d"),
            0xA3 => self.d_and_r("e"),
            0xA4 => self.d_and_r("h"),
            0xA5 => self.d_and_r("l"),
            0xA6 => self.d_and_r("(hl)"),
            0xA7 => self.d_and_r("a"),
            0xA8 => self.d_xor_r("b"),
            0xA9 => self.d_xor_r("c"),
            0xAA => self.d_xor_r("d"),
            0xAB => self.d_xor_r("e"),
            0xAC => self.d_xor_r("h"),
            0xAD => self.d_xor_r("l"),
            0xAE => self.d_xor_r("(hl)"),
            0xAF => self.d_xor_r("a"),
            0xB0 => self.d_or_r("b"),
            0xB1 => self.d_or_r("c"),
            0xB2 => self.d_or_r("d"),
            0xB3 => self.d_or_r("e"),
            0xB4 => self.d_or_r("h"),
            0xB5 => self.d_or_r("l"),
            0xB6 => self.d_or_r("(hl)"),
            0xB7 => self.d_or_r("a"),
            0xB8 => self.d_cp_r("b"),
            0xB9 => self.d_cp_r("c"),
            0xBA => self.d_cp_r("d"),
            0xBB => self.d_cp_r("e"),
            0xBC => self.d_cp_r("h"),
            0xBD => self.d_cp_r("l"),
            0xBE => self.d_cp_r("(hl)"),
            0xBF => self.d_cp_r("a"),
            0xC0 => "ret nz".to_string(),
            0xC1 => self.d_pop_nn("bc"),
            0xC2 => self.d_jp_f_nn("nz"),
            0xC3 => self.d_jp_nn(),
            0xC4 => self.d_call_f_nn("nz"),
            0xC5 => self.d_push_nn("bc"),
            0xC6 => self.d_add_a_n(),
            0xC7 => "rst 00h".to_string(),
            0xC8 => "ret z".to_string(),
            0xC9 => "ret".to_string(),
            0xCA => self.d_jp_f_nn("z"),
            0xCB => self.disassemble_cb(),
            0xCC => self.d_call_f_nn("z"),
            0xCD => self.d_call_nn(),
            0xCE => self.d_adc_n(),
            0xCF => "rst 08h".to_string(),
            0xD0 => "ret nc".to_string(),
            0xD1 => self.d_pop_nn("de"),
            0xD2 => self.d_jp_f_nn("nc"),
            0xD4 => self.d_call_f_nn("nc"),
            0xD5 => self.d_push_nn("de"),
            0xD6 => self.d_sub_n(),
            0xD7 => "rst 10h".to_string(),
            0xD8 => "ret c".to_string(),
            0xD9 => "reti".to_string(),
            0xDA => self.d_jp_f_nn("c"),
            0xDC => self.d_call_f_nn("c"),
            0xDE => self.d_sbc_n(),
            0xDF => "rst 18h".to_string(),
            0xE0 => self.d_ldh_n_r("a"),
            0xE1 => self.d_pop_nn("hl"),
            0xE2 => self.d_ld_r_r("0xff00 + c", "a"),
            0xE5 => self.d_push_nn("hl"),
            0xE6 => self.d_and_n(),
            0xE7 => "rst 20h".to_string(),
            0xE8 => self.d_add_sp_n(),
            0xE9 => "jp (hl)".to_string(),
            0xEA => self.d_ld_nn_rr("a"),
            0xEE => self.d_xor_n(),
            0xEF => "rst 28h".to_string(),
            0xF0 => self.d_ldh_r_n("a"),
            0xF1 => self.d_pop_nn("af"),
            0xF2 => self.d_ld_r_r("a", "0xff00 + c"),
            0xF3 => "di".to_string(),
            0xF5 => self.d_push_nn("af"),
            0xF6 => self.d_or_n(),
            0xF7 => "rst 30h".to_string(),
            0xF8 => self.d_ld_hl_sp_n(),
            0xF9 => "ld sp, hl".to_string(),
            0xFA => self.d_ld_rr_nn("a"),
            0xFB => "ei".to_string(),
            0xFE => self.d_cp_n(),
            0xFF => "rst 38h".to_string(),
            _ => "ill".to_string(),
        }
    }

    /// Gets the register name based on the opcode bits
    fn get_reg_name(&self, opcode: u8) -> &str {
        let reg_idx = (opcode & 0x07) as usize;
        if reg_idx < REGISTER_NAMES.len() {
            REGISTER_NAMES[reg_idx]
        } else {
            "invalid"
        }
    }

    /// Gets the register pair name based on the opcode bits
    fn get_reg_pair_name(&self, opcode: u8) -> &str {
        let pair_idx = ((opcode >> 4) & 0x03) as usize;
        if pair_idx < REGISTER_PAIRS.len() {
            REGISTER_PAIRS[pair_idx]
        } else {
            "invalid"
        }
    }

    /// Disassembles a RLC register instruction
    fn d_rlc_r(&self, prefix: u8) -> String {
        let mut result = "rlc ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a RRC register instruction
    fn d_rrc_r(&self, prefix: u8) -> String {
        let mut result = "rrc ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a RL register instruction
    fn d_rl_r(&self, prefix: u8) -> String {
        let mut result = "rl ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a RR register instruction
    fn d_rr_r(&self, prefix: u8) -> String {
        let mut result = "rr ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a SLA register instruction
    fn d_sla_r(&self, prefix: u8) -> String {
        let mut result = "sla ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a SRA register instruction
    fn d_sra_r(&self, prefix: u8) -> String {
        let mut result = "sra ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a SWAP register instruction
    fn d_swap_r(&self, prefix: u8) -> String {
        let mut result = "swap ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a SRL register instruction
    fn d_srl_r(&self, prefix: u8) -> String {
        let mut result = "srl ".to_string();
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a BIT/RES/SET register instruction
    fn d_bit_res_set_r(&self, prefix: u8) -> String {
        let bit = (prefix >> 3) & 7;
        let mut result: String;
        if prefix & 0xC0 == 0x40 {
            result = "bit ".to_string();
        } else if prefix & 0xC0 == 0x80 {
            result = "res ".to_string();
        } else {
            result = "set ".to_string();
        }
        result.push_str(&format!("{:#x}, ", bit));
        let register: &str = self.get_reg_name(prefix);
        result.push_str(register);
        result
    }

    /// Disassembles a CB prefix instruction
    fn disassemble_cb(&self) -> String {
        let prefix = self.read_byte(self.cpu.pc.wrapping_add(1));
        match prefix >> 3 {
            0 => self.d_rlc_r(prefix),
            1 => self.d_rrc_r(prefix),
            2 => self.d_rl_r(prefix),
            3 => self.d_rr_r(prefix),
            4 => self.d_sla_r(prefix),
            5 => self.d_sra_r(prefix),
            6 => self.d_swap_r(prefix),
            7 => self.d_srl_r(prefix),
            _ => self.d_bit_res_set_r(prefix),
        }
    }
}
