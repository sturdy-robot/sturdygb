use crate::core::mmu::Mmu;
use crate::core::registers::{FFlags, Registers};


pub const CYCLES: [u8; 256] = [
    0x04, 0x0C, 0x08, 0x08, 0x04, 0x04, 0x08, 0x04, 0x14, 0x08, 0x08, 0x08, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x0C, 0x08, 0x08, 0x04, 0x04, 0x08, 0x04, 0x0C, 0x08, 0x08, 0x08, 0x04, 0x04, 0x08, 0x04,
    0x0C, 0x0C, 0x08, 0x08, 0x04, 0x04, 0x08, 0x04, 0x0C, 0x08, 0x08, 0x08, 0x04, 0x04, 0x08, 0x04,
    0x0C, 0x0C, 0x08, 0x08, 0x0C, 0x0C, 0x0C, 0x04, 0x0C, 0x08, 0x08, 0x08, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x08, 0x04,
    0x08, 0x0C, 0x0C, 0x10, 0x0C, 0x10, 0x08, 0x10, 0x08, 0x10, 0x0C, 0x04, 0x0C, 0x18, 0x08, 0x10,
    0x08, 0x0C, 0x0C, 0x00, 0x0C, 0x10, 0x08, 0x10, 0x08, 0x10, 0x0C, 0x00, 0x0C, 0x00, 0x08, 0x10,
    0x0C, 0x0C, 0x08, 0x00, 0x00, 0x10, 0x08, 0x10, 0x10, 0x04, 0x10, 0x00, 0x00, 0x00, 0x08, 0x10,
    0x0C, 0x0C, 0x08, 0x04, 0x00, 0x10, 0x08, 0x10, 0x0C, 0x08, 0x10, 0x04, 0x00, 0x00, 0x08, 0x10,
];

pub const CB_CYCLES: [u8; 256] = [
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x08,
];

pub struct Opcode<'a> {
    pub opcode: u8,
    pub reg: &'a mut Registers,
    pub mmu: &'a mut Mmu,
    pub cb_inst: u8,
    pub is_cb: bool,
    pub is_halted: bool,
}

impl<'a> Opcode<'a> {
    pub fn new(opcode: u8, reg: &'a mut Registers, mmu: &'a mut Mmu) -> Self {
        let mut is_cb: bool = false;
        if opcode == 0xCB {
            is_cb = true;
        }
        
        Self {
            opcode,
            reg,
            mmu,
            cb_inst: 0,
            is_cb,
            is_halted: false,
        }
    }

    pub fn get_cycles(&mut self) -> u32 {
        if self.is_cb {
            CB_CYCLES[self.cb_inst as usize] as u32
        } else {
            CYCLES[self.opcode as usize] as u32
        }
    }

    pub fn decode(&mut self) {
        self.debug_registers();
        // MACROS FOR OPCODES
        
        // LD r, r
        macro_rules! ld_r_r {
            ($reg:ident, $reg2:ident) => {{
                self.reg.$reg = self.reg.$reg2;
            }};
        }

        // INC r
        macro_rules! inc_r {
            ($reg:ident) => {{
                self.reg.$reg = self.reg.$reg.wrapping_add(1);
                self.reg.set_f(FFlags::Z, self.reg.$reg == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, (self.reg.$reg & 0xF)  == 0);
            }};
        }

        // DEC r
        macro_rules! dec_r {
            ($reg:ident) => {{
                self.reg.$reg = self.reg.$reg.wrapping_sub(1);
                self.reg.set_f(FFlags::Z, self.reg.$reg == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, (self.reg.$reg & 0xF) == 0);
            }};
        }

        // LD (HL), r
        macro_rules! ld_hl_r {
            ($reg:ident) => {{
                self.mmu.write_byte(self.reg.hl(), self.reg.$reg);
            }};
        }

        // LD r, (HL)
        macro_rules! ld_r_hl {
            ($reg:ident) => {{
                self.reg.$reg = self.mmu.read_byte(self.reg.hl());
            }};
        }

        // ADD r
        macro_rules! add {
            ($reg:ident) => {{
                let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.$reg);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, (value & 0xF) < (self.reg.a & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
            ($hl:expr) => {{
                let hl: u8 = $hl;
                let (value, did_overflow) = self.reg.a.overflowing_add(hl);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, (value & 0xF) < (self.reg.a & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
        }

        // ADC r
        macro_rules! adc {
            ($reg:ident) => {{
                let flag: u8 = self.reg.get_flag(FFlags::C);
                let (mut value, mut did_overflow) = self.reg.$reg.overflowing_add(flag);
                if did_overflow {
                    value = self.reg.a.wrapping_add(value);
                } else {
                    (value, did_overflow) = self.reg.a.overflowing_add(value);
                }
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, ((self.reg.a & 0xF) + (self.reg.$reg & 0xF) + flag) > 0xF);
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
            ($hl:expr) => {{
                let flag = self.reg.get_flag(FFlags::C);
                let hl = $hl;
                let (mut value, mut did_overflow) = hl.overflowing_add(flag);
                if did_overflow {
                    value = self.reg.a.wrapping_add(value);
                } else {
                    (value, did_overflow) = self.reg.a.overflowing_add(value);
                }
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, ((self.reg.a & 0xF) + (hl & 0xF) + flag) > 0xF);
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
        }
        
        // SUB r
        macro_rules! sub {
            ($reg:ident) => {{
                let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.$reg);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, (self.reg.a & 0xF) < (value & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
            ($hl:expr) => {{
                let hl = $hl;
                let (value, did_overflow) = self.reg.a.overflowing_sub(hl);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, (self.reg.a & 0xF) < (value & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
        }

        // SBC r
        macro_rules! sbc {
            ($reg:ident) => {{
                let flag = self.reg.get_flag(FFlags::C);
                let value = self.reg.a.wrapping_sub(self.reg.$reg).wrapping_sub(flag);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, (self.reg.a & 0xF) < (self.reg.$reg & 0xF) + flag);
                self.reg.set_f(FFlags::C, (self.reg.a as u16) < ((self.reg.$reg + flag) as u16));
                self.reg.a = value;
            }};
            ($hl:expr) => {{
                let flag = self.reg.get_flag(FFlags::C);
                let hl = $hl;
                let value = self.reg.a.wrapping_sub(hl).wrapping_sub(flag);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, (self.reg.a & 0xF) < (hl & 0xF) + flag);
                self.reg.set_f(FFlags::C, (self.reg.a as u16) < (hl as u16) + (flag as u16));
                self.reg.a = value;
            }};
        }

        // AND r
        macro_rules! and {
            ($reg:ident) => {{
                let value = self.reg.a & self.reg.$reg;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, true);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
            ($hl:expr) => {{
                let value = self.reg.a & $hl;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, true);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
        }

        // XOR r
        macro_rules! xor {
            ($reg:ident) => {{
                let value = self.reg.a ^ self.reg.$reg;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
            ($hl:expr) => {{
                let value = self.reg.a ^ $hl;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
        }

        // OR r
        macro_rules! or {
            ($reg:ident) => {{
                let value = self.reg.a | self.reg.$reg;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
            ($hl:expr) => {{
                let value = self.reg.a | $hl;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
        }

        // CP r
        macro_rules! cp {
            ($reg:ident) => {{
                let value = self.reg.a.wrapping_sub(self.reg.$reg);
                self.reg.set_f(FFlags::Z, self.reg.a == self.reg.$reg);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, value > self.reg.a);
                self.reg.set_f(FFlags::C, self.reg.a < self.reg.$reg);
            }};
            ($hl:expr) => {{
                let hl = $hl;
                let value = self.reg.a.wrapping_sub(hl);
                self.reg.set_f(FFlags::Z, self.reg.a == hl);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, value > self.reg.a);
                self.reg.set_f(FFlags::C, self.reg.a < hl);
            }};
        }
        
        // DECODING OPCODES
        match self.opcode {
            // NOP
            0x00 => {
                // Do nothing
            }
            // LD BC, u16
            0x01 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.reg.set_bc(value);
            }
            // LD BC, A
            0x02 => {
                self.mmu.write_byte(self.reg.bc(), self.reg.a);
            }
            // INC BC
            0x03 => {
                self.reg.set_bc(self.reg.bc().wrapping_add(1));
            }
            // INC B
            0x04 => inc_r!(b),
            // DEC B
            0x05 => dec_r!(b),
            // LD B, U8
            0x06 => {
                self.reg.b = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RLCA
            0x07 => {
                self.reg.a = (self.reg.a << 1) | (self.reg.a >> 7);
                self.reg.set_f(FFlags::Z, self.reg.a == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, self.reg.a > 0x7F);
            }
            // LD u16, SP
            0x08 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.mmu.write_word(value, self.reg.sp);
            }
            // ADD HL, BC
            0x09 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.bc());
                self.reg.set_f(FFlags::N, false);
                self.reg
                    .set_f(FFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.set_hl(value);
            }
            // LD A, BC
            0x0A => {
                self.reg.a = self.mmu.read_byte(self.reg.bc());
            }
            // DEC BC
            0x0B => {
                self.reg.set_bc(self.reg.bc().wrapping_sub(1));
            }
            // INC C
            0x0C => inc_r!(c),
            // DEC C
            0x0D => dec_r!(c),
            // LD C, u8
            0x0E => {
                self.reg.c = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RRCA
            0x0F => {
                self.reg.a = (self.reg.a >> 1) | ((self.reg.a & 1) << 7);
                self.reg.set_f(FFlags::Z, self.reg.a == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, self.reg.a > 0x7F);
            }
            // STOP
            0x10 => self.stop(),
            // LD DE, u16
            0x11 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.reg.set_de(value);
            }
            // LD DE, A
            0x12 => {
                self.mmu.write_byte(self.reg.de(), self.reg.a);
            }
            // INC DE
            0x13 => {
                self.reg.set_de(self.reg.de().wrapping_add(1));
            }
            // INC D
            0x14 => inc_r!(d),
            // DEC D
            0x15 => dec_r!(d),
            // LD D, u8
            0x16 => {
                self.reg.d = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RLA
            0x17 => {
                let c = self.reg.f & FFlags::C as u8;
                self.reg.set_f(FFlags::Z, self.reg.a == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, self.reg.a > 0x7F);
                self.reg.a = (self.reg.a << 1) | c;
            }
            // JR u8
            0x18 => {
                self.jr();
            }
            // ADD HL, DE
            0x19 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.de());
                self.reg.set_f(FFlags::N, false);
                self.reg
                    .set_f(FFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.set_hl(value);
            }
            // LD A, DE
            0x1A => {
                self.reg.a = self.mmu.read_byte(self.reg.de());
            }
            // DEC DE
            0x1B => {
                self.reg.set_de(self.reg.de().wrapping_sub(1));
            }
            // INC E
            0x1C => inc_r!(e),
            // DEC E
            0x1D => dec_r!(e),
            // LD E, u8
            0x1E => {
                self.reg.e = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RRA
            0x1F => {
                let c = self.reg.f & FFlags::C as u8;
                self.reg.set_f(FFlags::Z, self.reg.a == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (self.reg.a & 1) == 1);
                self.reg.a = (self.reg.a >> 1) | c;
            }
            // JR NZ, r8
            0x20 => {
                self.jr_nf(FFlags::Z as u8);
            }
            // LD HL, u16
            0x21 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.reg.set_hl(value);
            }
            // LDI HL, A
            0x22 => {
                ld_hl_r!(a);
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
            }
            // INC HL
            0x23 => {
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
            }
            // INC H
            0x24 => inc_r!(h),
            // DEC H
            0x25 => dec_r!(h),
            // LD H, u8
            0x26 => {
                self.reg.h = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // DAA
            0x27 => self.daa(),
            // JR Z, r8
            0x28 => {
                self.jr_if(FFlags::Z as u8);
            }
            // ADD HL, HL
            0x29 => {
                // Do nothing
            }
            // LDI A, HL
            0x2A => {
                ld_r_hl!(a);
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
            }
            // DEC HL
            0x2B => {
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
            }
            // INC L
            0x2C => inc_r!(l),
            // DEC L
            0x2D => dec_r!(l),
            // LD L, u8
            0x2E => {
                self.reg.l = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // CPL
            0x2F => {
                self.reg.a = !self.reg.a;
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, true);
            }
            // JR NC, r8
            0x30 => {
                self.jr_nf(FFlags::C as u8);
            }
            // LD SP, u16
            0x31 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.reg.sp = value;
            }
            // LDD HL, A
            0x32 => {
                ld_hl_r!(a);
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
            }
            // INC SP
            0x33 => {
                self.reg.sp = self.reg.sp.wrapping_add(1);
            }
            // INC (HL)
            0x34 => {
                let value = self.mmu.read_byte(self.reg.hl()).wrapping_add(1);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, (value & 0xF) == 0);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // DEC (HL)
            0x35 => {
                let value = self.mmu.read_byte(self.reg.hl().wrapping_sub(1));
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, (value & 0xF) == 0);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // LDD HL, u8
            0x36 => {
                self.mmu.write_word(self.reg.hl(), self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // SCF
            0x37 => {
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, true);
            }
            // JR C, r8
            0x38 => self.jr_if(FFlags::C as u8),
            // ADD HL, SP
            0x39 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.sp);
                self.reg.set_f(FFlags::N, false);
                self.reg
                    .set_f(FFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.set_hl(value);
            }
            // LDD A, HL
            0x3A => {
                ld_r_hl!(a);
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
            },
            // DEC SP
            0x3B => {
                self.reg.sp = self.reg.sp.wrapping_sub(1);
            }
            // INC A
            0x3C => inc_r!(a),
            // DEC A
            0x3D => dec_r!(a),
            // LD A, u8
            0x3E => {
                self.reg.a = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // CCF
            0x3F => {
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                if self.reg.f & FFlags::C as u8 == 0x10 {
                    self.reg.set_f(FFlags::C, false);
                } else {
                    self.reg.set_f(FFlags::C, true);
                }
            }
            // LD B, B
            0x40 => {
                // Do nothing
            }
            // LD B, C
            0x41 => ld_r_r!(b, c),
            // LD B, D
            0x42 => ld_r_r!(b, d),
            // LD B, E
            0x43 => ld_r_r!(b, e),
            // LD B, H
            0x44 => ld_r_r!(b, h),
            // LD B, L
            0x45 => ld_r_r!(b, l),
            // LD B, (HL)
            0x46 => ld_r_hl!(b),
            // LD B, A
            0x47 => ld_r_r!(b, a),
            // LD C, B
            0x48 => ld_r_r!(c, b),
            // LD C, C
            0x49 => {
                // Do nothing
            }
            // LD C, D
            0x4A => ld_r_r!(c, d),
            // LD C, E
            0x4B => ld_r_r!(c, e),
            // LD C, H
            0x4C => ld_r_r!(c, h),
            // LD C, L
            0x4D => ld_r_r!(c, l),
            // LD C, (HL)
            0x4E => ld_r_hl!(c),
            // LD C, A
            0x4F => ld_r_r!(c, a),
            // LD D, B
            0x50 => ld_r_r!(d, b),
            // LD D, C
            0x51 => ld_r_r!(d, c),
            // LD D, D
            0x52 => {
                // Do nothing
            }
            // LD D, E
            0x53 => ld_r_r!(d, e),
            // LD D, H
            0x54 => ld_r_r!(d, h),
            // LD D, L
            0x55 => ld_r_r!(d, l),
            // LD D, (HL)
            0x56 => ld_r_hl!(d),
            // LD D, A
            0x57 => ld_r_r!(d, a),
            // LD E, B
            0x58 => ld_r_r!(e, b),
            // LD E, C
            0x59 => ld_r_r!(e, c),
            // LD E, D
            0x5A => ld_r_r!(e, d),
            // LD E, E
            0x5B => {
                // Do nothing
            }
            // LD E, H
            0x5C => ld_r_r!(e, h),
            // LD E, L
            0x5D => ld_r_r!(e, l),
            // LD E, (HL)
            0x5E => ld_r_hl!(e),
            // LD E, A
            0x5F => ld_r_r!(e, a),
            // LD H, B
            0x60 => ld_r_r!(h, b),
            // LD H, C
            0x61 => ld_r_r!(h, c),
            // LD H, D
            0x62 => ld_r_r!(h, d),
            // LD H, E
            0x63 => ld_r_r!(h, e),
            // LD H, H
            0x64 => {
                // Do nothing
            }
            // LD H, L
            0x65 => ld_r_r!(h, l),
            // LD H, HL
            0x66 => ld_r_hl!(h),
            // LD H, A
            0x67 => ld_r_r!(h, a),
            // LD L, B
            0x68 => ld_r_r!(l, b),
            // LD L, C
            0x69 => ld_r_r!(l, c),
            // LD L, D
            0x6A => ld_r_r!(l, d),
            // LD L, E
            0x6B => ld_r_r!(l, e),
            // LD L, H
            0x6C => ld_r_r!(l, h),
            // LD L, L
            0x6D => {
                // Do nothing
            }
            // LD L, (HL)
            0x6E => ld_r_hl!(l),
            // LD L, A
            0x6F => ld_r_r!(l, a),
            // LD (HL), B
            0x70 => ld_hl_r!(b),
            // LD (HL), C
            0x71 => ld_hl_r!(c),
            // LD (HL), D
            0x72 => ld_hl_r!(d),
            // LD (HL), E
            0x73 => ld_hl_r!(e),
            // LD (HL), H
            0x74 => ld_hl_r!(h),
            // LD (HL), L
            0x75 => ld_hl_r!(l),
            // HALT
            0x76 => self.halt(),
            // LD (HL), A
            0x77 => ld_hl_r!(a),
            // LD A, B
            0x78 => ld_r_r!(a, b),
            // LD A, C
            0x79 => ld_r_r!(a, c),
            // LD A, D
            0x7A => ld_r_r!(a, d),
            // LD A, E
            0x7B => ld_r_r!(a, e),
            // LD A, H
            0x7C => ld_r_r!(a, h),
            // LD A, L
            0x7D => ld_r_r!(a, l),
            // LD A, (HL)
            0x7E => ld_r_hl!(a),
            // LD A, A
            0x7F => {
                // Do nothing
            }
            // ADD A, B
            0x80 => add!(b),
            // ADD A, C
            0x81 => add!(c),
            // ADD A, D
            0x82 => add!(d),
            // ADD A, E
            0x83 => add!(e),
            // ADD A, H
            0x84 => add!(h),
            // ADD A, L
            0x85 => add!(l),
            // ADD A, (HL)
            0x86 => add!(self.mmu.read_byte(self.reg.hl())),
            // ADD A, A
            0x87 => add!(a),
            // ADC A, B
            0x88 => adc!(b),
            // ADC A, C
            0x89 => adc!(c),
            // ADC A, D
            0x8A => adc!(d),
            // ADC A, E
            0x8B => adc!(e),
            // ADC A, H
            0x8C => adc!(h),
            // ADC A, L
            0x8D => adc!(l),
            // ADC A, (HL)
            0x8E => adc!(self.mmu.read_byte(self.reg.hl())),
            // ADC A, A
            0x8F => adc!(a),
            // SUB A, B
            0x90 => sub!(b),
            // SUB A, C
            0x91 => sub!(c),
            // SUB A, D
            0x92 => sub!(d),
            // SUB A, B
            0x93 => sub!(e),
            // SUB A, H
            0x94 => sub!(h),
            // SUB A, L
            0x95 => sub!(l),
            // SUB A, (HL)
            0x96 => sub!(self.mmu.read_byte(self.reg.hl())),
            // SUB A, A
            0x97 => sub!(a),
            // SBC A, B
            0x98 => sbc!(b),
            // SBC A, C
            0x99 => sbc!(c),
            // SBC A, D
            0x9A => sbc!(d),
            // SBC A, E
            0x9B => sbc!(e),
            // SBC A, H
            0x9C => sbc!(h),
            // SBC A, L
            0x9D => sbc!(l),
            // SBC A, (HL)
            0x9E => sbc!(self.mmu.read_byte(self.reg.hl())),
            // SBC A, A
            0x9F => sbc!(a),
            // AND B
            0xA0 => and!(b),
            // AND C
            0xA1 => and!(c),
            // AND D
            0xA2 => and!(d),
            // AND E
            0xA3 => and!(e),
            // AND H
            0xA4 => and!(h),
            // AND L
            0xA5 => and!(l),
            // AND (HL)
            0xA6 => and!(self.mmu.read_byte(self.reg.hl())),
            // AND A
            0xA7 => and!(a),
            // XOR B
            0xA8 => xor!(b),
            // XOR C
            0xA9 => xor!(c),
            // XOR D
            0xAA => xor!(d),
            // XOR E
            0xAB => xor!(e),
            // XOR H
            0xAC => xor!(h),
            // XOR L
            0xAD => xor!(l),
            // XOR (HL)
            0xAE => xor!(self.mmu.read_byte(self.reg.hl())),
            // XOR A
            0xAF => xor!(a),
            // OR B
            0xB0 => or!(b),
            // OR C
            0xB1 => or!(c),
            // OR D
            0xB2 => or!(b),
            // OR E
            0xB3 => or!(e),
            // OR H
            0xB4 => or!(h),
            // OR L
            0xB5 => or!(l),
            // OR (HL)
            0xB6 => or!(self.mmu.read_byte(self.reg.hl())),
            // OR A
            0xB7 => or!(a),
            // CP B
            0xB8 => cp!(b),
            // CP C
            0xB9 => cp!(c),
            // CP D
            0xBA => cp!(d),
            // CP E
            0xBB => cp!(e),
            // CP H
            0xBC => cp!(h),
            // CP L
            0xBD => cp!(l),
            // CP (HL)
            0xBE => cp!(self.mmu.read_byte(self.reg.hl())),
            // CP A
            0xBF => cp!(a),
            // RET NZ
            0xC0 => {
                if (self.reg.f & FFlags::Z as u8) != 0x80 {
                    self.ret();
                }
            }
            // POP BC
            0xC1 => {
                let sp = self.pop_stack();
                self.reg.set_bc(sp);
            }
            // JP NZ, u16
            0xC2 => {
                if (self.reg.f & FFlags::Z as u8) != 0x80 {
                    self.reg.pc = self.jp();
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // JP u16
            0xC3 => {
                self.reg.pc = self.jp();
            }
            // CALL NZ, a16
            0xC4 => {
                if (self.reg.f & FFlags::Z as u8) != 0x80 {
                    self.push_stack(self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc);
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // PUSH BC
            0xC5 => self.push_stack(self.reg.bc()),
            // ADD A, u8
            0xC6 => {
                add!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // RST 0x00
            0xC7 => self.rst(0x00),
            // RET Z
            0xC8 => {
                if (self.reg.f & FFlags::Z as u8) == FFlags::Z as u8 {
                    self.ret();
                }
            }
            // RET
            0xC9 => self.ret(),
            // JP Z, u16
            0xCA => {
                if (self.reg.f & FFlags::Z as u8) == FFlags::Z as u8 {
                    self.reg.pc = self.jp();
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            0xCB => self.decode_cb(),
            // CALL Z, u16
            0xCC => {
                if (self.reg.f & FFlags::Z as u8) == FFlags::Z as u8 {
                    self.push_stack(self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc);
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // CALL u16
            0xCD => {
                self.push_stack(self.reg.pc.wrapping_add(2));
                self.reg.pc = self.mmu.read_word(self.reg.pc);
            }
            // ADC A, u8
            0xCE => {
                adc!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RST 08h
            0xCF => self.rst(0x08),
            // RET NC
            0xD0 => {
                if (self.reg.f & FFlags::C as u8) != 0x10 {
                    self.ret();
                }
            }
            // POP DE
            0xD1 => {
                let sp = self.pop_stack();
                self.reg.set_de(sp);
            }
            // JP NC, u16
            0xD2 => {
                if (self.reg.f & FFlags::C as u8) != 0x10 {
                    self.reg.pc = self.jp();
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // CALL NC, u16
            0xD4 => {
                if self.reg.f & FFlags::C as u8 != 0x10 {
                    self.push_stack(self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc);
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // PUSH DE
            0xD5 => {
                self.push_stack(self.reg.de());
            }
            // SUB A, u8
            0xD6 => {
                sub!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // RST 10h
            0xD7 => self.rst(0x10),
            // RET C
            0xD8 => {
                if (self.reg.f & FFlags::C as u8) == 0x10 {
                    self.ret();
                }
            }
            // RETI
            0xD9 => {
                self.ret();
                self.reg.ime = true;
            }
            // JP C, u16
            0xDA => {
                if (self.reg.f & FFlags::C as u8) == 0x10 {
                    self.reg.pc = self.jp();
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // CALL C, u16
            0xDC => {
                if self.reg.f & FFlags::C as u8 == 0x10 {
                    self.push_stack(self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc);
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // SBC A, u8
            0xDE => {
                sbc!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RST 18h
            0xDF => self.rst(0x18),
            // LD u8, A
            0xE0 => {
                let value = 0xFF00 | self.mmu.read_byte(self.reg.pc) as u16;
                self.reg.pc = self.reg.pc.wrapping_add(1);
                self.mmu.write_byte(value, self.reg.a);
            }
            // POP HL
            0xE1 => {
                let sp = self.pop_stack();
                self.reg.set_hl(sp);
            }
            // LD (C), A
            0xE2 => self.mmu.write_byte(0xFF00 | self.reg.c as u16, self.reg.a),
            // PUSH HL
            0xE5 => self.push_stack(self.reg.hl()),
            // AND A, u8
            0xE6 => {
                and!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RST 20h
            0xE7 => self.rst(0x20),
            // ADD SP, i8
            0xE8 => {
                let value = self.mmu.read_byte(self.reg.pc) as i8 as i16 as u16;
                self.reg.pc = self.reg.pc.wrapping_add(1);
                self.reg.set_f(FFlags::Z, false);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(
                    FFlags::H,
                    (self.reg.pc & 0x000F).wrapping_add(value & 0x000F) > 0x000F,
                );
                self.reg.set_f(
                    FFlags::C,
                    (self.reg.sp & 0x00FF).wrapping_add(value & 0x00FF) > 0x00FF,
                );
                self.reg.sp = self.reg.sp.wrapping_add(value);
            }
            // JP (HL)
            0xE9 => self.reg.pc = self.reg.hl(),
            // LD u16, A
            0xEA => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.mmu.write_byte(value, self.reg.a);
            }
            // XOR u8
            0xEE => {
                xor!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // RST 28h
            0xEF => self.rst(0x28),
            // LD A, u8
            0xF0 => {
                let value = 0xFF00 | self.mmu.read_byte(self.reg.pc) as u16;
                self.reg.pc = self.reg.pc.wrapping_add(1);
                self.reg.a = self.mmu.read_byte(value);
            }
            // POP AF
            0xF1 => {
                let sp = self.pop_stack();
                self.reg.set_af(sp);
                self.reg.set_f(FFlags::Z, sp > 0x7F);
                self.reg.set_f(FFlags::N, (sp & 0x40) == 0x40);
                self.reg.set_f(FFlags::H, (sp & 0x20) == 0x20);
                self.reg.set_f(FFlags::C, (sp & 0x10) == 0x10);
            }
            // LD A, (C)
            0xF2 => {
                self.reg.a = self.mmu.read_byte(0xFF00 | self.reg.c as u16);
            }
            // DI
            0xF3 => self.reg.ime = false,
            // PUSH AF
            0xF5 => {
                self.push_stack(self.reg.af());
            }
            // OR A, u8
            0xF6 => {
                or!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RST 30h
            0xF7 => self.rst(0x30),
            // LD HL, SP + u8
            0xF8 => {
                let value = self.mmu.read_byte(self.reg.pc) as i8 as i16 as u16;
                self.reg.pc = self.reg.pc.wrapping_add(1);
                self.reg.set_f(FFlags::Z, false);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(
                    FFlags::H,
                    (self.reg.pc & 0x000F).wrapping_add(value & 0x000F) > 0x000F,
                );
                self.reg.set_f(
                    FFlags::C,
                    (self.reg.sp & 0x00FF).wrapping_add(value & 0x00FF) > 0x00FF,
                );
                let sum_value = self.reg.sp.wrapping_add(value);
                self.reg.set_hl(sum_value);
            }
            // LD SP, HL
            0xF9 => {
                self.reg.sp = self.reg.hl();
            }
            // LD A, u8
            0xFA => {
                self.reg.a = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
            }
            // EI
            0xFB => self.reg.ime = true,
            // CP u8
            0xFE => {
                cp!(self.mmu.read_byte(self.reg.pc));
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RST 38h
            0xFF => self.rst(0x38),
            _ => self.not_supported_instruction(),
        };
    }

    fn push_stack(&mut self, address: u16) {
        self.reg.sp = self.mmu.push_stack(self.reg.sp, address)
    }

    fn pop_stack(&mut self) -> u16 {
        let res: u16;
        (self.reg.sp, res) = self.mmu.pop_stack(&mut self.reg.sp);
        res
    }

    fn stop(&mut self) {
        self.mmu.write_byte(0xFF04, 0); // Resets DIV register
        self.reg.pc = self.reg.pc.wrapping_add(1); // stop instruction skips a byte
    }

    fn halt(&mut self) {
        if self.reg.ime {
            self.is_halted = true;
        } else {
            if (self.mmu.ieflag & self.mmu.io.ifflag & self.mmu.read_byte(0x1F)) == 0 {
                self.is_halted = true;
            } else {
                self.reg.pc = self.reg.pc.wrapping_sub(1);  // HALT BUG
            }
        }   
    }

    fn daa(&mut self) {
        if self.reg.f & FFlags::N as u8 != 0x40 {
            if (self.reg.f & FFlags::C as u8 == 0x10) || self.reg.a > 0x99 {
                self.reg.a = self.reg.a.wrapping_add(0x60);
                self.reg.set_f(FFlags::C, true);
            }
            if (self.reg.f & FFlags::H as u8 == 0x20) || (self.reg.a & 0xF) > 0x9 {
                self.reg.a = self.reg.a.wrapping_add(0x06);
                self.reg.set_f(FFlags::C, false);
            }
        } else if (self.reg.f & FFlags::C as u8 == 0x10) && (self.reg.f & FFlags::H as u8 == 0x20) {
            self.reg.a = self.reg.a.wrapping_add(0x9A);
            self.reg.set_f(FFlags::H, false);
        } else if self.reg.f & FFlags::C as u8 == 0x10 {
            self.reg.a = self.reg.a.wrapping_add(0xFA);
            self.reg.set_f(FFlags::H, false);
        }
        self.reg.set_f(FFlags::Z, self.reg.a == 0);
    }

    fn jr(&mut self) {
        let value = self.mmu.read_byte(self.reg.pc) as i8;
        self.reg.pc = self.reg.pc.wrapping_add(1);
        self.reg.pc = ((self.reg.pc as i32) + (value as i32)) as u16 ;
    }

    fn jr_if(&mut self, flag: u8) {
        if (self.reg.f & flag) == flag {
            self.jr();
        } else {
            self.reg.pc = self.reg.pc.wrapping_add(1);
        }
    }

    fn jr_nf(&mut self, flag: u8) {
        if (self.reg.f & flag) != flag {
            self.jr();
        } else {
            self.reg.pc = self.reg.pc.wrapping_add(1);
        }
    }

    fn jp(&mut self) -> u16 {
        let value = self.mmu.read_word(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(2);
        value
    }

    fn ret(&mut self) {
        self.reg.pc = self.pop_stack();
    }

    fn rst(&mut self, n: u8) {
        self.push_stack(self.reg.pc);
        self.reg.pc = n as u16;
    }

    fn decode_cb(&mut self) {
        // Macros for decoding CB instructions
        
        // BIT
        macro_rules! bitn {
            ($n:expr, $reg:ident) => {{
                let bit: u8 = match $n {
                    0 => 0x01,
                    1 => 0x02,
                    2 => 0x04,
                    3 => 0x08,
                    4 => 0x10,
                    5 => 0x20,
                    6 => 0x40,
                    7 => 0x80,
                    _ => 0x01,
                };
                self.reg.set_f(FFlags::Z, (self.reg.$reg & bit) == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, true);
            }};
            ($hl:expr, $n:expr) => {{
                let byte_hl = $hl;
                let bit: u8 = match $n {
                    0 => 0x01,
                    1 => 0x02,
                    2 => 0x04,
                    3 => 0x08,
                    4 => 0x10,
                    5 => 0x20,
                    6 => 0x40,
                    7 => 0x80,
                    _ => 0x01,
                };
                self.reg.set_f(FFlags::Z, (byte_hl & bit) == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, true);
            }};
        }

        // SWAP
        macro_rules! swapn {
            ($reg:ident) => {{
                let value = ((self.reg.$reg & 0xF) << 4) | (self.reg.$reg >> 4);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = ((byte_hl & 0xF) << 4) | (byte_hl >> 4);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        // RL
        macro_rules! rln {
            ($reg:ident) => {{
                let flag = match self.reg.$reg > 0x7F {
                    true => 1,
                    false => 0,
                };
                let value = (self.reg.$reg << 1) | flag;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, self.reg.$reg > 0x7F);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag = match byte_hl > 0x7F {
                    true => 1,
                    false => 0,
                };
                let value = (byte_hl << 1) | flag;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, byte_hl > 0x7F);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }


        // RR
        macro_rules! rrn {
            ($reg:ident) => {{
                let flag = match (self.reg.$reg & 0x01) == 0x01 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (self.reg.$reg >> 1);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (self.reg.$reg & 0x01) == 0x01);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag = match (byte_hl & 0x01) == 0x01 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (byte_hl >> 1);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (byte_hl & 0x01) == 0x01);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        // RLC
        macro_rules! rlcn {
            ($reg:ident) => {{
                let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
                    true => 1,
                    false => 0,
                };
                let value = ((self.reg.$reg << 1) & 0xFF) | flag;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, value > 0x7F);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
                    true => 1,
                    false => 0,
                };
                let value = ((byte_hl << 1) & 0xFF) | flag;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, value > 0x7F);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        // RRC
        macro_rules! rrcn {
            ($reg:ident) => {{
                let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (self.reg.$reg >> 1);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (byte_hl >> 1);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        // RES
        macro_rules! resn {
            ($n:expr, $reg:ident) => {{
                let bit: u8 = match $n {
                    0 => 0xFE,
                    1 => 0xFD,
                    2 => 0xFB,
                    3 => 0xF7,
                    4 => 0xEF,
                    5 => 0xDF,
                    6 => 0xBF,
                    7 => 0x7F,
                    _ => 0xFE,
                };
                self.reg.$reg &= bit;
            }};
            ($n:expr, $hl:expr) => {{
                let byte_hl = $hl;
                let bit: u8 = match $n {
                    0 => 0xFE,
                    1 => 0xFD,
                    2 => 0xFB,
                    3 => 0xF7,
                    4 => 0xEF,
                    5 => 0xDF,
                    6 => 0xBF,
                    7 => 0x7F,
                    _ => 0xFE,
                };
                self.mmu.write_byte(self.reg.hl(), byte_hl & bit);
            }};
        }

        // SET
        macro_rules! setn {
            ($n:expr, $reg:ident) => {{
                let bit: u8 = match $n {
                    0 => 0x01,
                    1 => 0x02,
                    2 => 0x04,
                    3 => 0x08,
                    4 => 0x10,
                    5 => 0x20,
                    6 => 0x40,
                    7 => 0x80,
                    _ => 0x01,
                };
                self.reg.$reg |= bit;
            }};
            ($n:expr, $hl:expr) => {{
                let byte_hl = $hl;
                let bit: u8 = match $n {
                    0 => 0x01,
                    1 => 0x02,
                    2 => 0x04,
                    3 => 0x08,
                    4 => 0x10,
                    5 => 0x20,
                    6 => 0x40,
                    7 => 0x80,
                    _ => 0x01,
                };
                self.mmu.write_byte(self.reg.hl(), byte_hl | bit);
            }};
        }

        // SLA
        macro_rules! slan {
            ($reg:ident) => {{
                let value = self.reg.$reg << 1;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, value > 0x7F);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = byte_hl << 1;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, value > 0x7F);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        // SRA
        macro_rules! sran {
            ($reg:ident) => {{
                let value = (self.reg.$reg & 0x80) | (self.reg.$reg >> 1);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = (byte_hl & 0x80) | (byte_hl >> 1);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }
        
        // SRL
        macro_rules! srln {
            ($reg:ident) => {{
                let value = self.reg.$reg >> 1;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = byte_hl >> 1;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        let instruction: u8;
        (instruction, self.reg.pc) = self.mmu.fetch_instruction(&mut self.reg.pc);
        match instruction {
            // RLC B
            0x00 => rlcn!(b),
            // RLC C
            0x01 => rlcn!(c),
            // RLC D
            0x02 => rlcn!(d),
            // RLC E
            0x03 => rlcn!(e),
            // RLC H
            0x04 => rlcn!(h),
            // RLC L
            0x05 => rlcn!(l),
            // RLC (HL)
            0x06 => rlcn!(self.mmu.read_byte(self.reg.hl())),
            // RLC A
            0x07 => rlcn!(a),
            // RRC B
            0x08 => rrcn!(b),
            // RRC C
            0x09 => rrcn!(c),
            // RRC D
            0x0A => rrcn!(d),
            // RRC E
            0x0B => rrcn!(e),
            // RRC H
            0x0C => rrcn!(h),
            // RRC L
            0x0D => rrcn!(l),
            // RRC (HL)
            0x0E => rrcn!(self.mmu.read_byte(self.reg.hl())),
            // RRC A
            0x0F => rrcn!(a),
            // RL B
            0x10 => rln!(b),
            // RL C
            0x11 => rln!(c),
            // RL D
            0x12 => rln!(d),
            // RL E
            0x13 => rln!(e),
            // RL H
            0x14 => rln!(h),
            // RL L
            0x15 => rln!(l),
            // RL (HL)
            0x16 => rln!(self.mmu.read_byte(self.reg.hl())),
            // RL A
            0x17 => rln!(a),
            // RR B
            0x18 => rrn!(b),
            // RR C
            0x19 => rrn!(c),
            // RR D
            0x1A => rrn!(d),
            // RR E
            0x1B => rrn!(e),
            // RR H
            0x1C => rrn!(h),
            // RR L
            0x1D => rrn!(l),
            // RR (HL)
            0x1E => rrn!(self.mmu.read_byte(self.reg.hl())),
            // RR A
            0x1F => rrn!(a),
            // SLA B
            0x20 => slan!(b),
            // SLA C
            0x21 => slan!(c),
            // SLA D
            0x22 => slan!(d),
            // SLA E
            0x23 => slan!(e),
            // SLA H
            0x24 => slan!(h),
            // SLA L
            0x25 => slan!(l),
            // SLA (HL)
            0x26 => slan!(self.mmu.read_byte(self.reg.hl())),
            // SLA A
            0x27 => slan!(a),
            // SRA B
            0x28 => sran!(b),
            // SRA C
            0x29 => sran!(c),
            // SRA D
            0x2A => sran!(d),
            // SRA E
            0x2B => sran!(e),
            // SRA H
            0x2C => sran!(h),
            // SRA L
            0x2D => sran!(l),
            // SRA (HL)
            0x2E => sran!(self.mmu.read_byte(self.reg.hl())),
            // SRA A
            0x2F => sran!(a),
            // SWAP B
            0x30 => swapn!(b),
            // SWAP C
            0x31 => swapn!(c),
            // SWAP D
            0x32 => swapn!(d),
            // SWAP E
            0x33 => swapn!(e),
            // SWAP H
            0x34 => swapn!(h),
            // SWAP L
            0x35 => swapn!(l),
            // SWAP (HL)
            0x36 => swapn!(self.mmu.read_byte(self.reg.hl())),
            // SWAP A
            0x37 => swapn!(a),
            // SRL B
            0x38 => srln!(b),
            // SRL C
            0x39 => srln!(c),
            // SRL D
            0x3A => srln!(d),
            // SRL E
            0x3B => srln!(e),
            // SRL H
            0x3C => srln!(h),
            // SRL L
            0x3D => srln!(l),
            // SRL (HL)
            0x3E => srln!(self.mmu.read_byte(self.reg.hl())),
            // SRL A
            0x3F => srln!(a),
            // BIT 0,B
            0x40 => bitn!(0, b),
            // BIT 0,C
            0x41 => bitn!(0, c),
            // BIT 0,D
            0x42 => bitn!(0, d),
            // BIT 0,E
            0x43 => bitn!(0, e),
            // BIT 0,H
            0x44 => bitn!(0, h),
            // BIT 0,L
            0x45 => bitn!(0, l),
            // BIT 0,(HL)
            0x46 => bitn!(self.mmu.read_byte(self.reg.hl()), 0),
            // BIT 0,A
            0x47 => bitn!(0, a),
            // BIT 1,B
            0x48 => bitn!(1, b),
            // BIT 1,C
            0x49 => bitn!(1, c),
            // BIT 1,D
            0x4A => bitn!(1, d),
            // BIT 1,E
            0x4B => bitn!(1, e),
            // BIT 1,H
            0x4C => bitn!(1, h),
            // BIT 1,L
            0x4D => bitn!(1, l),
            // BIT 1,(HL)
            0x4E => bitn!(self.mmu.read_byte(self.reg.hl()), 1),
            // BIT 1,A
            0x4F => bitn!(1, a),
            // BIT 2,B
            0x50 => bitn!(2, b),
            // BIT 2,C
            0x51 => bitn!(2, c),
            // BIT 2,D
            0x52 => bitn!(2, d),
            // BIT 2,E
            0x53 => bitn!(2, e),
            // BIT 2,H
            0x54 => bitn!(2, h),
            // BIT 2,L
            0x55 => bitn!(2, l),
            // BIT 2,(HL)
            0x56 => bitn!(self.mmu.read_byte(self.reg.hl()), 2),
            // BIT 2,A
            0x57 => bitn!(2, a),
            // BIT 3,B
            0x58 => bitn!(3, b),
            // BIT 3,C
            0x59 => bitn!(3, c),
            // BIT 3,D
            0x5A => bitn!(3, d),
            // BIT 3,E
            0x5B => bitn!(3, e),
            // BIT 3,H
            0x5C => bitn!(3, h),
            // BIT 3,L
            0x5D => bitn!(3, l),
            // BIT 3,(HL)
            0x5E => bitn!(self.mmu.read_byte(self.reg.hl()), 3),
            // BIT 3,A
            0x5F => bitn!(3, a),
            // BIT 4,B
            0x60 => bitn!(4, b),
            // BIT 4,C
            0x61 => bitn!(4, c),
            // BIT 4,D
            0x62 => bitn!(4, d),
            // BIT 4,E
            0x63 => bitn!(4, e),
            // BIT 4,H
            0x64 => bitn!(4, h),
            // BIT 4,L
            0x65 => bitn!(4, l),
            // BIT 4,(HL)
            0x66 => bitn!(self.mmu.read_byte(self.reg.hl()), 4),
            // BIT 4,A
            0x67 => bitn!(4, a),
            // BIT 5,B
            0x68 => bitn!(5, b),
            // BIT 5,C
            0x69 => bitn!(5, c),
            // BIT 5,D
            0x6A => bitn!(5, d),
            // BIT 5,E
            0x6B => bitn!(5, e),
            // BIT 5,H
            0x6C => bitn!(5, h),
            // BIT 5,L
            0x6D => bitn!(5, l),
            // BIT 5,(HL)
            0x6E => bitn!(self.mmu.read_byte(self.reg.hl()), 5),
            // BIT 5,A
            0x6F => bitn!(5, a),
            // BIT 6,B
            0x70 => bitn!(6, b),
            // BIT 6,C
            0x71 => bitn!(6, c),
            // BIT 6,D
            0x72 => bitn!(6, d),
            // BIT 6,E
            0x73 => bitn!(6, e),
            // BIT 6,H
            0x74 => bitn!(6, h),
            // BIT 6,L
            0x75 => bitn!(6, l),
            // BIT 6,(HL)
            0x76 => bitn!(self.mmu.read_byte(self.reg.hl()), 6),
            // BIT 6,A
            0x77 => bitn!(6, a),
            // BIT 7,B
            0x78 => bitn!(7, b),
            // BIT 7,C
            0x79 => bitn!(7, c),
            // BIT 7,D
            0x7A => bitn!(7, d),
            // BIT 7,E
            0x7B => bitn!(7, e),
            // BIT 7,H
            0x7C => bitn!(7, h),
            // BIT 7,L
            0x7D => bitn!(7, l),
            // BIT 7,(HL)
            0x7E => bitn!(self.mmu.read_byte(self.reg.hl()), 7),
            // BIT 7,A
            0x7F => bitn!(7, a),
            // RES 0,B
            0x80 => resn!(0, b),
            // RES 0,C
            0x81 => resn!(0, c),
            // RES 0,D
            0x82 => resn!(0, d),
            // RES 0,E
            0x83 => resn!(0, e),
            // RES 0,H
            0x84 => resn!(0, h),
            // RES 0,L
            0x85 => resn!(0, l),
            // RES 0,(HL)
            0x86 => resn!(0, self.mmu.read_byte(self.reg.hl())), 
            // RES 0,A
            0x87 => resn!(0, a),
            // RES 1,B
            0x88 => resn!(1, b),
            // RES 1,C
            0x89 => resn!(1, c),
            // RES 1,D
            0x8A => resn!(1, d),
            // RES 1,E
            0x8B => resn!(1, e),
            // RES 1,H
            0x8C => resn!(1, h),
            // RES 1,L
            0x8D => resn!(1, l),
            // RES 1,(HL)
            0x8E => resn!(1, self.mmu.read_byte(self.reg.hl())),
            // RES 1,A
            0x8F => resn!(1, a),
            // RES 2,B
            0x90 => resn!(2, b),
            // RES 2,C
            0x91 => resn!(2, c),
            // RES 2,D
            0x92 => resn!(2, d),
            // RES 2,E
            0x93 => resn!(2, e),
            // RES 2,H
            0x94 => resn!(2, h),
            // RES 2,L
            0x95 => resn!(2, l),
            // RES 2,(HL)
            0x96 => resn!(2, self.mmu.read_byte(self.reg.hl())),
            // RES 2,A
            0x97 => resn!(2, a),
            // RES 3,B
            0x98 => resn!(3, b),
            // RES 3,C
            0x99 => resn!(3, c),
            // RES 3,D
            0x9A => resn!(3, d),
            // RES 3,E
            0x9B => resn!(3, e),
            // RES 3,H
            0x9C => resn!(3, h),
            // RES 3,L
            0x9D => resn!(3, l),
            // RES 3,(HL)
            0x9E => resn!(3, self.mmu.read_byte(self.reg.hl())),
            // RES 3,A
            0x9F => resn!(3, a),
            // RES 4,B
            0xA0 => resn!(4, b),
            // RES 4,C
            0xA1 => resn!(4, c),
            // RES 4,D
            0xA2 => resn!(4, d),
            // RES 4,E
            0xA3 => resn!(4, e),
            // RES 4,H
            0xA4 => resn!(4, h),
            // RES 4,L
            0xA5 => resn!(4, l),
            // RES 4,(HL)
            0xA6 => resn!(4, self.mmu.read_byte(self.reg.hl())),
            // RES 4,A
            0xA7 => resn!(4, a),
            // RES 5,B
            0xA8 => resn!(5, b),
            // RES 5,C
            0xA9 => resn!(5, c),
            // RES 5,D
            0xAA => resn!(5, d),
            // RES 5,E
            0xAB => resn!(5, e),
            // RES 5,H
            0xAC => resn!(5, h),
            // RES 5,L
            0xAD => resn!(5, l),
            // RES 5,(HL)
            0xAE => resn!(5, self.mmu.read_byte(self.reg.hl())),
            // RES 5,A
            0xAF => resn!(5, a),
            // RES 6,B
            0xB0 => resn!(6, b),
            // RES 6,C
            0xB1 => resn!(6, c),
            // RES 6,D
            0xB2 => resn!(6, d),
            // RES 6,E
            0xB3 => resn!(6, e),
            // RES 6,H
            0xB4 => resn!(6, h),
            // RES 6,L
            0xB5 => resn!(6, l),
            // RES 6,(HL)
            0xB6 => resn!(6, self.mmu.read_byte(self.reg.hl())),
            // RES 6,A
            0xB7 => resn!(6, a),
            // RES 7,B
            0xB8 => resn!(7, b),
            // RES 7,C
            0xB9 => resn!(7, c),
            // RES 7,D
            0xBA => resn!(7, d),
            // RES 7,E
            0xBB => resn!(7, e),
            // RES 7,H
            0xBC => resn!(7, h),
            // RES 7,L
            0xBD => resn!(7, l),
            // RES 7,(HL)
            0xBE => resn!(7, self.mmu.read_byte(self.reg.hl())),
            // RES 7,A
            0xBF => resn!(7, a),
            // SET 0,B
            0xC0 => setn!(0, b),
            // SET 0,C
            0xC1 => setn!(0, c),
            // SET 0,D
            0xC2 => setn!(0, d),
            // SET 0,E
            0xC3 => setn!(0, e),
            // SET 0,H
            0xC4 => setn!(0, h),
            // SET 0,L
            0xC5 => setn!(0, l),
            // SET 0,(HL)
            0xC6 => setn!(0, self.mmu.read_byte(self.reg.hl())),
            // SET 0,A
            0xC7 => setn!(0, a),
            // SET 1,B
            0xC8 => setn!(1, b),
            // SET 1,C
            0xC9 => setn!(1, c),
            // SET 1,D
            0xCA => setn!(1, d),
            // SET 1,E
            0xCB => setn!(1, e),
            // SET 1,H
            0xCC => setn!(1, h),
            // SET 1,L
            0xCD => setn!(1, l),
            // SET 1,(HL)
            0xCE => setn!(1, self.mmu.read_byte(self.reg.hl())),
            // SET 1,A
            0xCF => setn!(1, a),
            // SET 2,B
            0xD0 => setn!(2, b),
            // SET 2,C
            0xD1 => setn!(2, c),
            // SET 2,D
            0xD2 => setn!(2, d),
            // SET 2,E
            0xD3 => setn!(2, e),
            // SET 2,H
            0xD4 => setn!(2, h),
            // SET 2,L
            0xD5 => setn!(2, l),
            // SET 2,(HL)
            0xD6 => setn!(2, self.mmu.read_byte(self.reg.hl())),
            // SET 2,A
            0xD7 => setn!(2, a),
            // SET 3,B
            0xD8 => setn!(3, b),
            // SET 3,C
            0xD9 => setn!(3, c),
            // SET 3,D
            0xDA => setn!(3, d),
            // SET 3,E
            0xDB => setn!(3, e),
            // SET 3,H
            0xDC => setn!(3, h),
            // SET 3,L
            0xDD => setn!(3, l),
            // SET 3,(HL)
            0xDE => setn!(3, self.mmu.read_byte(self.reg.hl())),
            // SET 3,A
            0xDF => setn!(3, a),
            // SET 4,B
            0xE0 => setn!(4, b),
            // SET 4,C
            0xE1 => setn!(4, c),
            // SET 4,D
            0xE2 => setn!(4, d),
            // SET 4,E
            0xE3 => setn!(4, e),
            // SET 4,H
            0xE4 => setn!(4, h),
            // SET 4,L
            0xE5 => setn!(4, l),
            // SET 4,(HL)
            0xE6 => setn!(4, self.mmu.read_byte(self.reg.hl())),
            // SET 4,A
            0xE7 => setn!(4, a),
            // SET 5,B
            0xE8 => setn!(5, b),
            // SET 5,C
            0xE9 => setn!(5, c),
            // SET 5,D
            0xEA => setn!(5, d),
            // SET 5,E
            0xEB => setn!(5, e),
            // SET 5,H
            0xEC => setn!(5, h),
            // SET 5,L
            0xED => setn!(5, l),
            // SET 5,(HL)
            0xEE => setn!(5, self.mmu.read_byte(self.reg.hl())),
            // SET 5,A
            0xEF => setn!(5, a),
            // SET 6,B
            0xF0 => setn!(6, b),
            // SET 6,C
            0xF1 => setn!(6, c),
            // SET 6,D
            0xF2 => setn!(6, d),
            // SET 6,E
            0xF3 => setn!(6, e),
            // SET 6,H
            0xF4 => setn!(6, h),
            // SET 6,L
            0xF5 => setn!(6, l),
            // SET 6,(HL)
            0xF6 => setn!(6, self.mmu.read_byte(self.reg.hl())),
            // SET 6,A
            0xF7 => setn!(6, a),
            // SET 7,B
            0xF8 => setn!(7, b),
            // SET 7,C
            0xF9 => setn!(7, c),
            // SET 7,D
            0xFA => setn!(7, d),
            // SET 7,E
            0xFB => setn!(7, e),
            // SET 7,H
            0xFC => setn!(7, h),
            // SET 7,L
            0xFD => setn!(7, l),
            // SET 7,(HL)
            0xFE => setn!(7, self.mmu.read_byte(self.reg.hl())),
            // SET 7,A
            0xFF => setn!(7, a),
        };
        self.cb_inst = instruction;
    }

    fn not_supported_instruction(&mut self) {
        panic!("Instruction not supported, 0x{:02X}", self.opcode);
    }
}

#[cfg(test)]
mod test {
    use crate::core::cpu::Cpu;
    use crate::core::mmu::Mmu;
    use crate::core::cartridge::{load_cartridge};
    use crate::core::opcodes::Opcode;
    use crate::core::registers::Registers;

    fn set_up() -> Cpu {
        match load_cartridge("roms/cpu_instrs.gb") {
            Ok(cartridge) => {
                let mmu = Mmu::new(cartridge);
                Cpu::new(Registers::new(0x01, 0x00, 0xFF, 0x13, 0x00, 0xC1, 0x84, 0x03), mmu)
            },
            Err(_) => panic!("Failed to load roms/cpu_instrs.gb"),
        }   
    }

    #[test]
    fn test_add_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 9] = [0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0xC6];
        let expected_reg_a: [u8; 9] = [0x01, 0x14, 0x14, 0xEC, 0xED, 0x3A, 0x75, 0xEA, 0xEA];
        let expected_reg_f: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x20, 0x00, 0x00];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_adc_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 9] = [0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F, 0xCE];
        let expected_reg_a: [u8; 9] = [0x02, 0x15, 0x15, 0xED, 0xEE, 0x3B, 0x77, 0xEE, 0xEE];
        let expected_reg_f: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x20, 0x00, 0x00];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_sub_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 9] = [0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0xD6];
        let expected_reg_a: [u8; 9] = [0x01, 0xEE, 0xEE, 0x16, 0x15, 0xC8, 0x8D, 0x00, 0x00];
        let expected_reg_f: [u8; 9] = [0x40, 0x70, 0x40, 0x40, 0x40, 0x70, 0x60, 0xC0, 0xC0];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_sbc_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 8] = [0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F];
        let expected_reg_a: [u8; 8] = [0x00, 0xED, 0xEC, 0x14, 0x13, 0xC6, 0x8A, 0x00];
        let expected_reg_f: [u8; 8] = [0xC0, 0x70, 0x40, 0x40, 0x40, 0x70, 0x60, 0xC0];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_or_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 9] = [0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xF6];
        let expected_reg_a: [u8; 9] = [0x01, 0x13, 0x13, 0xDB, 0xDB, 0xDF, 0xFF, 0xFF, 0xFF];
        let expected_reg_f: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_xor_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 9] = [0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF, 0xEE];
        let expected_reg_a: [u8; 9] = [0x01, 0x12, 0x12, 0xCA, 0xCB, 0x86, 0xBD, 0x00, 0x00];
        let expected_reg_f: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_and_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 9] = [0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xE6];
        let expected_reg_a: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected_reg_f: [u8; 9] = [0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0, 0xA0];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_cp_instructions() {
        let mut cpu = set_up();
        let opcodes: [u8; 9] = [0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF, 0xFE];
        let expected_reg_a: [u8; 9] = [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
        let expected_reg_f: [u8; 9] = [0x40, 0x70, 0x40, 0x70, 0xC0, 0x70, 0x70, 0xC0, 0x40];
        for i in 0..opcodes.len() {
            let mut opcode = Opcode::new(opcodes[i], &mut cpu.reg, &mut cpu.mmu);
            opcode.decode();
            assert_eq!(cpu.reg.a, expected_reg_a[i]);
            assert_eq!(cpu.reg.f, expected_reg_f[i]);
        }
    }

    #[test]
    fn test_inc_instructions() {
        let mut cpu = set_up();
        let mut opcode = Opcode::new(0x3C, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x02);
        assert_eq!(cpu.reg.f, 0x10);
        opcode = Opcode::new(0x04, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.b, 0x01);
        assert_eq!(cpu.reg.f, 0x10);
        opcode = Opcode::new(0x0C, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.c, 0x14);
        assert_eq!(cpu.reg.f, 0x10);
        opcode = Opcode::new(0x14, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.d, 0x01);
        assert_eq!(cpu.reg.f, 0x10);
        opcode = Opcode::new(0x1C, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.e, 0xD9);
        assert_eq!(cpu.reg.f, 0x10);
        opcode = Opcode::new(0x24, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.h, 0x02);
        assert_eq!(cpu.reg.f, 0x10);
        opcode = Opcode::new(0x2C, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.l, 0x4E);
        assert_eq!(cpu.reg.f, 0x10);
        opcode = Opcode::new(0x34, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.hl(), 0x024E);
        assert_eq!(cpu.reg.f, 0x10);
    }

    #[test]
    fn test_dec_instructions() {
        let mut cpu = set_up();
        let mut opcode = Opcode::new(0x3D, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, 0xF0);
        opcode = Opcode::new(0x05, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.b, 0xFF);
        assert_eq!(cpu.reg.f, 0x50);
        opcode = Opcode::new(0x0D, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.c, 0x12);
        assert_eq!(cpu.reg.f, 0x50);
        opcode = Opcode::new(0x15, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.d, 0xFF);
        assert_eq!(cpu.reg.f, 0x50);
        opcode = Opcode::new(0x1D, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.e, 0xD7);
        assert_eq!(cpu.reg.f, 0x50);
        opcode = Opcode::new(0x25, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.h, 0x00);
        assert_eq!(cpu.reg.f, 0xF0);
        opcode = Opcode::new(0x2D, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.l, 0x4C);
        assert_eq!(cpu.reg.f, 0x50);
        opcode = Opcode::new(0x35, &mut cpu.reg, &mut cpu.mmu);
        opcode.decode();
        assert_eq!(cpu.reg.hl(), 0x4C);
        assert_eq!(cpu.reg.f, 0xF0);
    }
}
