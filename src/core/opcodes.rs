use crate::core::mmu::MMU;
use crate::core::registers::{FFlags, Registers};

pub struct Opcode<'a> {
    opcode: u8,
    reg: &'a mut Registers,
    mmu: &'a mut MMU,
    pub is_halted: bool,
}

impl<'a> Opcode<'a> {
    pub fn new(opcode: u8, reg: &'a mut Registers, mmu: &'a mut MMU) -> Self {
        Self {
            opcode,
            reg,
            mmu,
            is_halted: false,
        }
    }

    pub fn decode(&mut self) {
        macro_rules! ld_r_r {
            ($reg:ident, $reg2:ident) => {{
                self.reg.$reg = self.reg.$reg;
            }};
        }

        macro_rules! inc_r {
            ($reg:ident) => {{
                self.reg.$reg = self.reg.$reg.wrapping_add(1);
                self.reg.set_f(FFlags::Z, self.reg.b == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, self.reg.b == 0);
            }};
        }

        macro_rules! dec_r {
            ($reg:ident) => {{
                self.reg.$reg = self.reg.$reg.wrapping_sub(1);
                self.reg.set_f(FFlags::Z, self.reg.b == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, self.reg.b == 0);
            }};
        }

        macro_rules! ld_hl_r {
            ($reg:ident) => {{
                self.mmu.write_byte(self.reg.hl(), self.reg.$reg);
            }};
        }

        macro_rules! ld_r_hl {
            ($reg:ident) => {{
                self.reg.$reg = self.mmu.read_byte(self.reg.hl());
            }};
        }

        macro_rules! add {
            ($reg:ident) => {{
                let (value, did_overflow) = self.reg.a.overflowing_add(self.reg.$reg);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg
                    .set_f(FFlags::H, (value & 0xF) < (self.reg.a & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
        }

        macro_rules! adc {
            ($reg:ident) => {{
                let (value, did_overflow) = self
                    .reg
                    .a
                    .overflowing_add(self.reg.$reg.wrapping_add(FFlags::C as u8));
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg
                    .set_f(FFlags::H, (value & 0xF) < (self.reg.a & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
        }

        macro_rules! sub {
            ($reg:ident) => {{
                let (value, did_overflow) = self.reg.a.overflowing_sub(self.reg.$reg);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg
                    .set_f(FFlags::H, (self.reg.a & 0xF) > (value & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
        }

        macro_rules! sbc {
            ($reg:ident) => {{
                let (value, did_overflow) = self
                    .reg
                    .a
                    .overflowing_sub(self.reg.$reg.wrapping_sub(FFlags::C as u8));
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg
                    .set_f(FFlags::H, (self.reg.a & 0xF) > (value & 0xF));
                self.reg.set_f(FFlags::C, did_overflow);
                self.reg.a = value;
            }};
        }

        macro_rules! and {
            ($reg:ident) => {{
                let value = self.reg.a & self.reg.$reg;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, true);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
        }

        macro_rules! xor {
            ($reg:ident) => {{
                let value = self.reg.a ^ self.reg.$reg;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
        }

        macro_rules! or {
            ($reg:ident) => {{
                let value = self.reg.a | self.reg.$reg;
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.a = value;
            }};
        }

        macro_rules! cp {
            ($reg:ident) => {{
                let value = self.reg.a.wrapping_sub(self.reg.$reg);
                self.reg.set_f(FFlags::Z, self.reg.a == self.reg.$reg);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, value > self.reg.a);
                self.reg.set_f(FFlags::C, self.reg.a < self.reg.$reg);
            }};
        }

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
                self.reg.a = ((self.reg.a << 1) & 0xFF) | (self.reg.a >> 7);
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
                self.reg.a = ((self.reg.a << 1) & 0xFF) | c;
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
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                self.mmu.write_byte(self.reg.hl(), self.reg.a);
            }
            // INC HL
            0x23 => {
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
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
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                ld_r_hl!(a);
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
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
                self.mmu.write_byte(self.reg.hl(), self.reg.a);
            }
            // INC SP
            0x33 => {
                self.reg.sp = self.reg.sp.wrapping_add(1);
            }
            // INC (HL)
            0x34 => {
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                self.reg.set_f(FFlags::Z, self.reg.hl() == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, self.reg.hl() == 0);
            }
            // DEC (HL)
            0x35 => {
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
                self.reg.set_f(FFlags::Z, self.reg.hl() == 0);
                self.reg.set_f(FFlags::N, true);
                self.reg.set_f(FFlags::H, self.reg.hl() == 0);
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
            0x38 => {
                self.jr_if(FFlags::C as u8);
            }
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
            0x3A => ld_hl_r!(a),
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
            0x86 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_add(value);
            }
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
            0x8E => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_adc(value);
            }
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
            0x96 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_sub(value);
            }
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
            0x9E => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_sbc(value);
            }
            // SBC A, A
            0x9F => sbc!(a),
            // OR B
            0xA0 => and!(b),
            // OR C
            0xA1 => and!(c),
            // OR D
            0xA2 => and!(d),
            // OR E
            0xA3 => and!(e),
            // OR H
            0xA4 => and!(h),
            // OR L
            0xA5 => and!(l),
            // OR (HL)
            0xA6 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_and(value);
            }
            // OR A
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
            0xAE => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_xor(value);
            }
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
            0xB6 => {
                let value = self.mmu.read_byte(self.reg.hl());
                or!(b);
            }
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
            0xBE => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_cp(value);
            }
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
                    self.reg.pc = self.mmu.read_word(self.reg.pc).wrapping_add(2);
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // PUSH BC
            0xC5 => {
                self.push_stack(self.reg.bc());
            }
            // ADD A, u8
            0xC6 => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_add(value);
            }
            // RST 0x00
            0xC7 => self.rst(0x00),
            // RET Z
            0xC8 => {
                if (self.reg.f & FFlags::Z as u8) == 0x80 {
                    self.ret();
                }
            }
            // RET
            0xC9 => {
                self.ret();
            }
            // JP Z, u16
            0xCA => {
                if (self.reg.f & FFlags::Z as u8) == 0x80 {
                    self.reg.pc = self.jp();
                }
            }
            0xCB => self.decode_cb(),
            // CALL Z, u16
            0xCC => {
                if (self.reg.f & FFlags::Z as u8) == 0x80 {
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
                let value = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
                self.alu_adc(value);
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
                }
            }
            // CALL NC, u16
            0xD4 => {
                if self.reg.f & FFlags::C as u8 != 0x10 {
                    self.push_stack(self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc).wrapping_add(2);
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
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_sub(value);
            }
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
                }
            }
            // CALL C, u16
            0xDC => {
                if self.reg.f & FFlags::C as u8 == 0x10 {
                    self.push_stack(self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc).wrapping_add(2);
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            // SBC A, u16
            0xDE => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.alu_sbc(value);
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
            0xE2 => {
                self.mmu.write_byte(0xFF00 | self.reg.c as u16, self.reg.a);
            }
            // PUSH HL
            0xE5 => {
                self.push_stack(self.reg.hl());
            }
            // AND A, u8
            0xE6 => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_and(value);
            }
            // RST 20h
            0xE7 => self.rst(0x20),
            // ADD SP, u8
            0xE8 => {
                let value = self.mmu.read_byte(self.reg.pc) as u16;
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
            0xE9 => {
                self.reg.pc = self.reg.hl();
            }
            // LD u16, A
            0xEA => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.mmu.write_byte(value, self.reg.a);
            }
            // XOR u8
            0xEE => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_xor(value);
            }
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
            0xF3 => {
                self.reg.ime = false;
            }
            // PUSH AF
            0xF5 => {
                self.push_stack(self.reg.af());
            }
            // OR A, u8
            0xF6 => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_or(value);
            }
            // RST 30h
            0xF7 => self.rst(0x30),
            // LD HL, SP + u8
            0xF8 => {
                let value = self.mmu.read_byte(self.reg.pc) as u16;
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
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // EI
            0xFB => {
                self.reg.ime = true;
            }
            // CP u8
            0xFE => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_cp(value);
            }
            // RST 38h
            0xFF => self.rst(0x38),
            _ => self.not_supported_instruction(),
        };
    }

    fn push_stack(&mut self, address: u16) {
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.mmu.write_word(self.reg.sp, address)
    }

    fn pop_stack(&mut self) -> u16 {
        let sp = self.mmu.read_word(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(2);
        sp
    }

    fn stop(&mut self) {
        self.reg.pc = self.reg.pc.wrapping_add(1); // stop instruction skips a byte
    }

    fn halt(&mut self) {
        self.is_halted = true;
    }

    fn alu_add(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_add(reg);
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg
            .set_f(FFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(FFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_adc(&mut self, reg: u8) {
        let (value, did_overflow) = self
            .reg
            .a
            .overflowing_add(reg.wrapping_add(FFlags::C as u8));
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg
            .set_f(FFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(FFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_sub(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(reg);
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, true);
        self.reg
            .set_f(FFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(FFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_sbc(&mut self, reg: u8) {
        let (value, did_overflow) = self
            .reg
            .a
            .overflowing_sub(reg.wrapping_sub(FFlags::C as u8));
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, true);
        self.reg
            .set_f(FFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(FFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_and(&mut self, reg: u8) {
        let value = self.reg.a & reg;
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, true);
        self.reg.set_f(FFlags::C, false);
        self.reg.a = value;
    }

    fn alu_xor(&mut self, reg: u8) {
        let value = self.reg.a ^ reg;
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, false);
        self.reg.a = value;
    }

    fn alu_or(&mut self, reg: u8) {
        let value = self.reg.a | reg;
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, false);
        self.reg.a = value;
    }

    fn alu_cp(&mut self, reg: u8) {
        let value = self.reg.a.wrapping_sub(reg);
        self.reg.set_f(FFlags::Z, self.reg.a == reg);
        self.reg.set_f(FFlags::N, true);
        self.reg.set_f(FFlags::H, value > self.reg.a);
        self.reg.set_f(FFlags::C, self.reg.a < reg);
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
        } else if (self.reg.f & FFlags::C as u8 == 0x10)
            && (self.reg.f & FFlags::H as u8 == 0x20)
        {
            self.reg.a = self.reg.a.wrapping_add(0x9A);
            self.reg.set_f(FFlags::H, false);
        } else if self.reg.f & FFlags::C as u8 == 0x10 {
            self.reg.a = self.reg.a.wrapping_add(0xFA);
            self.reg.set_f(FFlags::H, false);
        }
        self.reg.set_f(FFlags::Z, self.reg.a == 0);
    }

    fn jr(&mut self) {
        let value;
        (value, self.reg.pc) = self.mmu.fetch_instruction(&mut self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(value as u16);
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

    fn decode_cb(&mut self) {
        macro_rules! bitn {
            ($reg:ident, $n:ident) => {{
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
        }

        macro_rules! swapn {
            ($reg:ident) => {{
                let value = ((self.reg.$reg & 0xF) << 4) | (self.reg.$reg >> 4);
                self.reg.set_f(FFlags::Z, value == 0);
                self.reg.set_f(FFlags::N, false);
                self.reg.set_f(FFlags::H, false);
                self.reg.set_f(FFlags::C, false);
                self.reg.$reg = value;
            }};
        }

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
        }

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
        }

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
                self.reg.$reg = self.reg.$reg & bit
            }};
            () => {{}};
        }

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
                self.reg.$reg = self.reg.$reg | bit
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
            0x06 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.rlcn(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0x0E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.rrcn(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RRC A
            0x0F => rrcn!(a),
            // RL B
            0x10 => {
                self.reg.b = self.rln(self.reg.b);
            }
            // RL C
            0x11 => {
                self.reg.c = self.rln(self.reg.c);
            }
            // RL D
            0x12 => {
                self.reg.d = self.rln(self.reg.d);
            }
            // RL E
            0x13 => {
                self.reg.e = self.rln(self.reg.e);
            }
            // RL H
            0x14 => {
                self.reg.h = self.rln(self.reg.h);
            }
            // RL L
            0x15 => {
                self.reg.l = self.rln(self.reg.l);
            }
            // RL (HL)
            0x16 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.rln(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RL A
            0x17 => {
                self.reg.a = self.rln(self.reg.a);
            }
            // RR B
            0x18 => {
                self.reg.b = self.rrn(self.reg.b);
            }
            // RR C
            0x19 => {
                self.reg.c = self.rrn(self.reg.c);
            }
            // RR D
            0x1A => {
                self.reg.d = self.rrn(self.reg.d);
            }
            // RR E
            0x1B => {
                self.reg.e = self.rrn(self.reg.e);
            }
            // RR H
            0x1C => {
                self.reg.h = self.rrn(self.reg.h);
            }
            // RR L
            0x1D => {
                self.reg.l = self.rrn(self.reg.l);
            }
            // RR (HL)
            0x1E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.rrn(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RR A
            0x1F => {
                self.reg.a = self.rrn(self.reg.a);
            }
            // SLA B
            0x20 => {
                self.reg.b = self.slan(self.reg.b);
            }
            // SLA C
            0x21 => {
                self.reg.c = self.slan(self.reg.c);
            }
            // SLA D
            0x22 => {
                self.reg.d = self.slan(self.reg.d);
            }
            // SLA E
            0x23 => {
                self.reg.e = self.slan(self.reg.e);
            }
            // SLA H
            0x24 => {
                self.reg.h = self.slan(self.reg.h);
            }
            // SLA L
            0x25 => {
                self.reg.l = self.slan(self.reg.l);
            }
            // SLA (HL)
            0x26 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.slan(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SLA A
            0x27 => {
                self.reg.a = self.slan(self.reg.a);
            }
            // SRA B
            0x28 => {
                self.reg.b = self.sran(self.reg.b);
            }
            // SRA C
            0x29 => {
                self.reg.c = self.sran(self.reg.c);
            }
            // SRA D
            0x2A => {
                self.reg.d = self.sran(self.reg.d);
            }
            // SRA E
            0x2B => {
                self.reg.e = self.sran(self.reg.e);
            }
            // SRA H
            0x2C => {
                self.reg.h = self.sran(self.reg.h);
            }
            // SRA L
            0x2D => {
                self.reg.l = self.sran(self.reg.l);
            }
            // SRA (HL)
            0x2E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.sran(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SRA A
            0x2F => {
                self.reg.a = self.sran(self.reg.a);
            }
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
            0x36 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.swapn(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SWAP A
            0x37 => swapn!(a),
            // SRL B
            0x38 => {
                self.reg.b = self.srln(self.reg.b);
            }
            // SRL C
            0x39 => {
                self.reg.c = self.srln(self.reg.c);
            }
            // SRL D
            0x3A => {
                self.reg.d = self.srln(self.reg.d);
            }
            // SRL E
            0x3B => {
                self.reg.e = self.srln(self.reg.e);
            }
            // SRL H
            0x3C => {
                self.reg.h = self.srln(self.reg.h);
            }
            // SRL L
            0x3D => {
                self.reg.l = self.srln(self.reg.l);
            }
            // SRL (HL)
            0x3E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.srln(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SRL A
            0x3F => {
                self.reg.a = self.srln(self.reg.a);
            }
            // BIT 0,B
            0x40 => {
                self.bitn(0, self.reg.b);
            }
            // BIT 0,C
            0x41 => {
                self.bitn(0, self.reg.c);
            }
            // BIT 0,D
            0x42 => {
                self.bitn(0, self.reg.d);
            }
            // BIT 0,E
            0x43 => {
                self.bitn(0, self.reg.e);
            }
            // BIT 0,H
            0x44 => {
                self.bitn(0, self.reg.h);
            }
            // BIT 0,L
            0x45 => {
                self.bitn(0, self.reg.l);
            }
            // BIT 0,(HL)
            0x46 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(0, byte_hl);
            }
            // BIT 0,A
            0x47 => {
                self.bitn(0, self.reg.a);
            }
            // BIT 1,B
            0x48 => {
                self.bitn(1, self.reg.b);
            }
            // BIT 1,C
            0x49 => {
                self.bitn(1, self.reg.c);
            }
            // BIT 1,D
            0x4A => {
                self.bitn(1, self.reg.d);
            }
            // BIT 1,E
            0x4B => {
                self.bitn(1, self.reg.e);
            }
            // BIT 1,H
            0x4C => {
                self.bitn(1, self.reg.h);
            }
            // BIT 1,L
            0x4D => {
                self.bitn(1, self.reg.l);
            }
            // BIT 1,(HL)
            0x4E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(1, byte_hl);
            }
            // BIT 1,A
            0x4F => {
                self.bitn(1, self.reg.a);
            }
            // BIT 2,B
            0x50 => {
                self.bitn(2, self.reg.b);
            }
            // BIT 2,C
            0x51 => {
                self.bitn(2, self.reg.c);
            }
            // BIT 2,D
            0x52 => {
                self.bitn(2, self.reg.d);
            }
            // BIT 2,E
            0x53 => {
                self.bitn(2, self.reg.e);
            }
            // BIT 2,H
            0x54 => {
                self.bitn(2, self.reg.h);
            }
            // BIT 2,L
            0x55 => {
                self.bitn(2, self.reg.l);
            }
            // BIT 2,(HL)
            0x56 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(2, byte_hl);
            }
            // BIT 2,A
            0x57 => {
                self.bitn(2, self.reg.a);
            }
            // BIT 3,B
            0x58 => {
                self.bitn(3, self.reg.b);
            }
            // BIT 3,C
            0x59 => {
                self.bitn(3, self.reg.c);
            }
            // BIT 3,D
            0x5A => {
                self.bitn(3, self.reg.d);
            }
            // BIT 3,E
            0x5B => {
                self.bitn(3, self.reg.e);
            }
            // BIT 3,H
            0x5C => {
                self.bitn(3, self.reg.h);
            }
            // BIT 3,L
            0x5D => {
                self.bitn(3, self.reg.l);
            }
            // BIT 3,(HL)
            0x5E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(3, byte_hl);
            }
            // BIT 3,A
            0x5F => {
                self.bitn(3, self.reg.a);
            }
            // BIT 4,B
            0x60 => {
                self.bitn(4, self.reg.b);
            }
            // BIT 4,C
            0x61 => {
                self.bitn(4, self.reg.c);
            }
            // BIT 4,D
            0x62 => {
                self.bitn(4, self.reg.d);
            }
            // BIT 4,E
            0x63 => {
                self.bitn(4, self.reg.e);
            }
            // BIT 4,H
            0x64 => {
                self.bitn(4, self.reg.h);
            }
            // BIT 4,L
            0x65 => {
                self.bitn(4, self.reg.l);
            }
            // BIT 4,(HL)
            0x66 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(4, byte_hl);
            }
            // BIT 4,A
            0x67 => {
                self.bitn(4, self.reg.a);
            }
            // BIT 5,B
            0x68 => {
                self.bitn(5, self.reg.b);
            }
            // BIT 5,C
            0x69 => {
                self.bitn(5, self.reg.c);
            }
            // BIT 5,D
            0x6A => {
                self.bitn(5, self.reg.d);
            }
            // BIT 5,E
            0x6B => {
                self.bitn(5, self.reg.e);
            }
            // BIT 5,H
            0x6C => {
                self.bitn(5, self.reg.h);
            }
            // BIT 5,L
            0x6D => {
                self.bitn(5, self.reg.l);
            }
            // BIT 5,(HL)
            0x6E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(5, byte_hl);
            }
            // BIT 5,A
            0x6F => {
                self.bitn(5, self.reg.a);
            }
            // BIT 6,B
            0x70 => {
                self.bitn(6, self.reg.b);
            }
            // BIT 6,C
            0x71 => {
                self.bitn(6, self.reg.c);
            }
            // BIT 6,D
            0x72 => {
                self.bitn(6, self.reg.d);
            }
            // BIT 6,E
            0x73 => {
                self.bitn(6, self.reg.e);
            }
            // BIT 6,H
            0x74 => {
                self.bitn(6, self.reg.h);
            }
            // BIT 6,L
            0x75 => {
                self.bitn(6, self.reg.l);
            }
            // BIT 6,(HL)
            0x76 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(6, byte_hl);
            }
            // BIT 6,A
            0x77 => {
                self.bitn(6, self.reg.a);
            }
            // BIT 7,B
            0x78 => {
                self.bitn(7, self.reg.b);
            }
            // BIT 7,C
            0x79 => {
                self.bitn(7, self.reg.c);
            }
            // BIT 7,D
            0x7A => {
                self.bitn(7, self.reg.d);
            }
            // BIT 7,E
            0x7B => {
                self.bitn(7, self.reg.e);
            }
            // BIT 7,H
            0x7C => {
                self.bitn(7, self.reg.h);
            }
            // BIT 7,L
            0x7D => {
                self.bitn(7, self.reg.l);
            }
            // BIT 7,(HL)
            0x7E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                self.bitn(7, byte_hl);
            }
            // BIT 7,A
            0x7F => {
                self.bitn(7, self.reg.a);
            }
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
            0x86 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(0, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0x8E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(1, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0x96 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(2, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0x9E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(3, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0xA6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(4, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0xAE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(5, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0xB6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(6, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
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
            0xBE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(7, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 7,A
            0xBF => resn!(7, a),
            // SET 0,B
            0xC0 => {
                self.reg.b = self.setn(0, self.reg.b);
            }
            // SET 0,C
            0xC1 => {
                self.reg.c = self.setn(0, self.reg.c);
            }
            // SET 0,D
            0xC2 => {
                self.reg.d = self.setn(0, self.reg.d);
            }
            // SET 0,E
            0xC3 => {
                self.reg.e = self.setn(0, self.reg.e);
            }
            // SET 0,H
            0xC4 => {
                self.reg.h = self.setn(0, self.reg.h);
            }
            // SET 0,L
            0xC5 => {
                self.reg.l = self.setn(0, self.reg.l);
            }
            // SET 0,(HL)
            0xC6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(0, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 0,A
            0xC7 => {
                self.reg.a = self.setn(0, self.reg.a);
            }
            // SET 1,B
            0xC8 => {
                self.reg.b = self.setn(1, self.reg.b);
            }
            // SET 1,C
            0xC9 => {
                self.reg.c = self.setn(1, self.reg.c);
            }
            // SET 1,D
            0xCA => {
                self.reg.d = self.setn(1, self.reg.d);
            }
            // SET 1,E
            0xCB => {
                self.reg.e = self.setn(1, self.reg.e);
            }
            // SET 1,H
            0xCC => {
                self.reg.h = self.setn(1, self.reg.h);
            }
            // SET 1,L
            0xCD => {
                self.reg.l = self.setn(1, self.reg.l);
            }
            // SET 1,(HL)
            0xCE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(1, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 1,A
            0xCF => {
                self.reg.a = self.setn(1, self.reg.a);
            }
            // SET 2,B
            0xD0 => {
                self.reg.b = self.setn(2, self.reg.b);
            }
            // SET 2,C
            0xD1 => {
                self.reg.c = self.setn(2, self.reg.c);
            }
            // SET 2,D
            0xD2 => {
                self.reg.d = self.setn(2, self.reg.d);
            }
            // SET 2,E
            0xD3 => {
                self.reg.e = self.setn(2, self.reg.e);
            }
            // SET 2,H
            0xD4 => {
                self.reg.h = self.setn(2, self.reg.h);
            }
            // SET 2,L
            0xD5 => {
                self.reg.l = self.setn(2, self.reg.l);
            }
            // SET 2,(HL)
            0xD6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(2, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 2,A
            0xD7 => {
                self.reg.a = self.setn(2, self.reg.a);
            }
            // SET 3,B
            0xD8 => {
                self.reg.b = self.setn(3, self.reg.b);
            }
            // SET 3,C
            0xD9 => {
                self.reg.c = self.setn(3, self.reg.c);
            }
            // SET 3,D
            0xDA => {
                self.reg.d = self.setn(3, self.reg.d);
            }
            // SET 3,E
            0xDB => {
                self.reg.e = self.setn(3, self.reg.e);
            }
            // SET 3,H
            0xDC => {
                self.reg.h = self.setn(3, self.reg.h);
            }
            // SET 3,L
            0xDD => {
                self.reg.l = self.setn(3, self.reg.l);
            }
            // SET 3,(HL)
            0xDE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(3, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 3,A
            0xDF => {
                self.reg.a = self.setn(3, self.reg.a);
            }
            // SET 4,B
            0xE0 => {
                self.reg.b = self.setn(4, self.reg.b);
            }
            // SET 4,C
            0xE1 => {
                self.reg.c = self.setn(4, self.reg.c);
            }
            // SET 4,D
            0xE2 => {
                self.reg.d = self.setn(4, self.reg.d);
            }
            // SET 4,E
            0xE3 => {
                self.reg.e = self.setn(4, self.reg.e);
            }
            // SET 4,H
            0xE4 => {
                self.reg.h = self.setn(4, self.reg.h);
            }
            // SET 4,L
            0xE5 => {
                self.reg.l = self.setn(4, self.reg.l);
            }
            // SET 4,(HL)
            0xE6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(4, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 4,A
            0xE7 => {
                self.reg.a = self.setn(4, self.reg.a);
            }
            // SET 5,B
            0xE8 => {
                self.reg.b = self.setn(5, self.reg.b);
            }
            // SET 5,C
            0xE9 => {
                self.reg.c = self.setn(5, self.reg.c);
            }
            // SET 5,D
            0xEA => {
                self.reg.d = self.setn(5, self.reg.d);
            }
            // SET 5,E
            0xEB => {
                self.reg.e = self.setn(5, self.reg.e);
            }
            // SET 5,H
            0xEC => {
                self.reg.h = self.setn(5, self.reg.h);
            }
            // SET 5,L
            0xED => {
                self.reg.l = self.setn(5, self.reg.l);
            }
            // SET 5,(HL)
            0xEE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(5, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 5,A
            0xEF => {
                self.reg.a = self.setn(5, self.reg.a);
            }
            // SET 6,B
            0xF0 => {
                self.reg.b = self.setn(6, self.reg.b);
            }
            // SET 6,C
            0xF1 => {
                self.reg.c = self.setn(6, self.reg.c);
            }
            // SET 6,D
            0xF2 => {
                self.reg.d = self.setn(6, self.reg.d);
            }
            // SET 6,E
            0xF3 => {
                self.reg.e = self.setn(6, self.reg.e);
            }
            // SET 6,H
            0xF4 => {
                self.reg.h = self.setn(6, self.reg.h);
            }
            // SET 6,L
            0xF5 => {
                self.reg.l = self.setn(6, self.reg.l);
            }
            // SET 6,(HL)
            0xF6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(6, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 6,A
            0xF7 => {
                self.reg.a = self.setn(6, self.reg.a);
            }
            // SET 7,B
            0xF8 => {
                self.reg.b = self.setn(7, self.reg.b);
            }
            // SET 7,C
            0xF9 => {
                self.reg.c = self.setn(7, self.reg.c);
            }
            // SET 7,D
            0xFA => {
                self.reg.d = self.setn(7, self.reg.d);
            }
            // SET 7,E
            0xFB => {
                self.reg.e = self.setn(7, self.reg.e);
            }
            // SET 7,H
            0xFC => {
                self.reg.h = self.setn(7, self.reg.h);
            }
            // SET 7,L
            0xFD => {
                self.reg.l = self.setn(7, self.reg.l);
            }
            // SET 7,(HL)
            0xFE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.setn(7, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SET 7,A
            0xFF => {
                self.reg.a = self.setn(7, self.reg.a);
            }
        };
    }

    fn rst(&mut self, n: u8) {
        self.push_stack(self.reg.pc);
        self.reg.pc = n as u16;
    }

    fn rlcn(&mut self, reg: u8) -> u8 {
        let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
            true => 1,
            false => 0,
        };
        let value = ((reg << 1) & 0xFF) | flag;
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, value > 0x7F);
        value
    }

    fn rrcn(&mut self, reg: u8) -> u8 {
        let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
            true => 0x80,
            false => 0,
        };
        let value = flag | (reg >> 1);
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
        value
    }

    fn rln(&mut self, reg: u8) -> u8 {
        let flag = match reg > 0x7F {
            true => 1,
            false => 0,
        };
        let value = (reg << 1) & 0xFF | flag;
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, reg > 0x7F);
        value
    }

    fn rrn(&mut self, reg: u8) -> u8 {
        let flag = match (reg & 0x01) == 0x01 {
            true => 0x80,
            false => 0,
        };
        let value = flag | (reg >> 1);
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, (reg & 0x01) == 0x01);
        value
    }

    fn slan(&mut self, reg: u8) -> u8 {
        let value = (reg << 1) & 0xFF;
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, value > 0x7F);
        value
    }

    fn sran(&mut self, reg: u8) -> u8 {
        let value = (reg & 0x80) | (reg >> 1);
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
        value
    }

    fn swapn(&mut self, reg: u8) -> u8 {
        let value = ((reg & 0xF) << 4) | (reg >> 4);
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, false);
        value
    }

    fn srln(&mut self, reg: u8) -> u8 {
        let value = reg >> 1;
        self.reg.set_f(FFlags::Z, value == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, false);
        self.reg.set_f(FFlags::C, (value & 0x01) == 0x01);
        value
    }

    fn bitn(&mut self, n: u8, reg: u8) {
        let b: u8 = match n {
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
        self.reg.set_f(FFlags::Z, (reg & b) == 0);
        self.reg.set_f(FFlags::N, false);
        self.reg.set_f(FFlags::H, true);
    }

    fn resn(&mut self, n: u8, reg: u8) -> u8 {
        let b: u8 = match n {
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
        reg & b
    }

    fn setn(&mut self, n: u8, reg: u8) -> u8 {
        let b: u8 = match n {
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
        reg | b
    }

    fn not_supported_instruction(&mut self) {
        println!("Instruction not supported, {:x}", self.opcode);
    }
}
