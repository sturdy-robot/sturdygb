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
            // NOP
            0x00 => { },
            // LD BC, u16
            0x01 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.reg.set_bc(value);
            },
            // LD BC, A
            0x02 => { self.mmu.write_byte(self.reg.bc(), self.reg.a); },
            // INC BC
            0x03 => { self.reg.set_bc(self.reg.bc().wrapping_add(1)); },
            // INC B
            0x04 => { 
                self.reg.b = self.reg.b.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.b == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, self.reg.b == 0);
            },
            // DEC B
            0x05 => {
                self.reg.b = self.reg.b.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.b == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, self.reg.b == 0);
            },
            // LD B, U8
            0x06 => {
                self.reg.b = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // RLCA
            0x07 => {
                self.reg.a = ((self.reg.a << 1) & 0xFF) | (self.reg.a >> 7);
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                self.reg.set_f(CPUFlags::C, self.reg.a > 0x7F);
            },
            // LD u16, SP
            0x08 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.mmu.write_word(value, self.reg.sp);
            },
            // ADD HL, BC
            0x09 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.bc());
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(CPUFlags::C, did_overflow);
                self.reg.set_hl(value);
            },
            // LD A, BC
            0x0A => {
                self.reg.a = self.mmu.read_byte(self.reg.bc());
            },
            // DEC BC
            0x0B => { self.reg.set_bc(self.reg.bc().wrapping_sub(1)); },
            // INC C
            0x0C => {
                self.reg.c = self.reg.c.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.c == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.c & 0xF) == 0);
            },
            // DEC C
            0x0D => {
                self.reg.c = self.reg.c.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.c == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.c & 0xF) == 0);
            },
            // LD C, u8
            0x0E => {
                self.reg.c = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // RRCA
            0x0F => {
                self.reg.a = (self.reg.a >> 1) | ((self.reg.a & 1) << 7);
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                self.reg.set_f(CPUFlags::C, self.reg.a > 0x7F);
            },
            // STOP
            0x10 => self.stop(),
            // LD DE, u16
            0x11 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.reg.set_de(value);
            },
            // LD DE, A
            0x12 => { self.mmu.write_byte(self.reg.de(), self.reg.a); },
            // INC DE
            0x13 => { self.reg.set_de(self.reg.de().wrapping_add(1)); },
            // INC D
            0x14 => {
                self.reg.d = self.reg.d.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.d == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.d & 0xF) == 0);
            },
            // DEC D
            0x15 => {
                self.reg.d = self.reg.d.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.d == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.d & 0xF) == 0);
            },
            // LD D, u8
            0x16 => {
                self.reg.d = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // RLA
            0x17 => {
                todo!();
            },
            // JR u8
            0x18 => {
                todo!();
            },
            // ADD HL, DE
            0x19 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.de());
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(CPUFlags::C, did_overflow);
                self.reg.set_hl(value);
            },
            // LD A, DE
            0x1A => { self.reg.a = self.mmu.read_byte(self.reg.de()); },
            // DEC DE
            0x1B => { self.reg.set_de(self.reg.de().wrapping_sub(1)); },
            // INC E
            0x1C => { 
                self.reg.e = self.reg.e.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.e == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.e & 0xF) == 0);
            },
            // DEC E
            0x1D => {
                self.reg.e = self.reg.e.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.e == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.e & 0xF) == 0);
            },
            // LD E, u8
            0x1E => {
                self.reg.e = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // RRA
            0x1F => {
                todo!();
            },
            // JR NZ, r8
            0x20 => {
                todo!();
            },
            // LD HL, u16
            0x21 => {
                let value = self.mmu.read_word(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.reg.set_hl(value);
            },
            // LDI HL, A
            0x22 => {
                todo!();
            },
            // INC HL
            0x23 => { self.reg.set_hl(self.reg.hl().wrapping_sub(1)); },
            // INC H
            0x24 => {
                self.reg.h = self.reg.h.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.h == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.h & 0xF) == 0);
            },
            // DEC H
            0x25 => {
                self.reg.h = self.reg.h.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.h == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.h & 0xF) == 0);
            },
            // LD H, u8
            0x26 => {
                self.reg.h = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // DAA
            0x27 => {
                todo!();
            },
            // JR Z, r8
            0x28 => {
                todo!()
            },
            // ADD HL, HL
            0x29 => {
                // Do nothing
            },
            // LDI A, HL
            0x2A => {
                todo!()
            },
            // DEC HL
            0x2B => {
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
            },
            // INC L
            0x2C => {
                self.reg.l = self.reg.l.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.l == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.l & 0xF) == 0);
            },
            // DEC L
            0x2D => {
                self.reg.l = self.reg.l.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.l == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.l & 0xF) == 0);
            },
            // LD L, u8
            0x2E => {
                self.reg.l = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // CPL
            0x2F => {
                todo!();
            },
            // JR NC, r8
            0x30 => {
                todo!();
            },
            // LD SP, u16
            0x31 => {

            },
            // LDD HL, A
            0x32 => {
                todo!()
            },
            // INC SP
            0x33 => { self.reg.sp = self.reg.sp.wrapping_add(1); },
            // INC (HL)
            0x34 => {
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                self.reg.set_f(CPUFlags::Z, self.reg.hl() == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, self.reg.hl() == 0);
            },
            // DEC (HL)
            0x35 => {
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
                self.reg.set_f(CPUFlags::Z, self.reg.hl() == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, self.reg.hl() == 0);
            },
            // LDD HL, u8
            0x36 => {
                self.mmu.write_word(self.reg.hl(), self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // SCF
            0x37 => {
                todo!();
            },
            // JR C, r8
            0x38 => {
                todo!()
            },
            // ADD HL, SP
            0x39 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.sp);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(CPUFlags::C, did_overflow);
                self.reg.set_hl(value);
            },
            // LDD A, HL
            0x3A => {
                self.mmu.write_byte(self.reg.hl(), self.reg.a);
            },
            // DEC SP
            0x3B => {
                self.reg.sp = self.reg.sp.wrapping_sub(1);
            },
            // INC A
            0x3C => {
                self.reg.a = self.reg.a.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) == 0);
            },
            // DEC A
            0x3D => {
                self.reg.a = self.reg.a.wrapping_sub(1) & 0xFF;
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) == 0);
            },
            // LD A, u8
            0x3E => {
                self.reg.a = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            // CCF
            0x3F => {
                todo!()
            },
            // LD B, B
            0x40 => {
                // Do nothing
            },
            // LD B, C
            0x41 => {
                self.reg.b = self.reg.c;
            },
            // LD B, D
            0x42 => { self.reg.b = self.reg.d; },
            // LD B, E
            0x43 => { self.reg.b = self.reg.e; },
            // LD B, H
            0x44 => { self.reg.b = self.reg.h; },
            // LD B, L
            0x45 => { self.reg.b = self.reg.l; },
            // LD B, (HL)
            0x46 => { self.reg.b = self.mmu.read_byte(self.reg.hl()); },
            // LD B, A
            0x47 => { self.reg.b = self.reg.a; },
            // LD C, B
            0x48 => { self.reg.c = self.reg.b; },
            // LD C, C
            0x49 => {
                // Do nothing
            },
            // LD C, D
            0x4A => { self.reg.c = self.reg.d; },
            // LD C, E
            0x4B => { self.reg.c = self.reg.e; },
            // LD C, H
            0x4C => { self.reg.c = self.reg.h; },
            // LD C, L
            0x4D => { self.reg.c = self.reg.l; },
            // LD C, (HL)
            0x4E => { self.reg.c = self.mmu.read_byte(self.reg.hl()); },
            // LD C, A
            0x4F => { self.reg.c = self.reg.a; },
            // LD D, B
            0x50 => { self.reg.d = self.reg.b; },
            // LD D, C
            0x51 => { self.reg.d = self.reg.c; },
            // LD D, D
            0x52 => {
                // Do nothing
            },
            // LD D, E
            0x53 => { self.reg.d = self.reg.e; },
            // LD D, H
            0x54 => { self.reg.d = self.reg.h; },
            // LD D, L
            0x55 => { self.reg.d = self.reg.l; },
            // LD D, (HL)
            0x56 => { self.reg.d = self.mmu.read_byte(self.reg.hl()); },
            // LD D, A
            0x57 => { self.reg.d = self.reg.a; },
            // LD E, B
            0x58 => { self.reg.e = self.reg.b; },
            // LD E, C
            0x59 => { self.reg.e = self.reg.c; },
            // LD E, D
            0x5A => { self.reg.e = self.reg.d; },
            // LD E, E
            0x5B => {
                // Do nothing
            },
            // LD E, H
            0x5C => { self.reg.e = self.reg.h; },
            // LD E, L
            0x5D => { self.reg.e = self.reg.l; },
            // LD E, (HL)
            0x5E => { self.reg.e = self.mmu.read_byte(self.reg.hl()); },
            // LD E, A
            0x5F => { self.reg.e = self.reg.a; },
            // LD H, B
            0x60 => { self.reg.h = self.reg.b; },
            // LD H, C
            0x61 => { self.reg.h = self.reg.c; },
            // LD H, D
            0x62 => { self.reg.h = self.reg.d; },
            // LD H, E
            0x63 => { self.reg.h = self.reg.e; },
            // LD H, H
            0x64 => {
                // Do nothing
            },
            // LD H, L
            0x65 => { self.reg.h = self.reg.l; },
            // LD H, HL
            0x66 => { self.reg.h = self.mmu.read_byte(self.reg.hl()); },
            // LD H, A
            0x67 => { self.reg.h = self.reg.a; },
            // LD L, B
            0x68 => { self.reg.l = self.reg.b; },
            // LD L, C
            0x69 => { self.reg.l = self.reg.c; },
            // LD L, D
            0x6A => { self.reg.l = self.reg.d; },
            // LD L, E
            0x6B => { self.reg.l = self.reg.e; },
            // LD L, H
            0x6C => { self.reg.l = self.reg.h; },
            // LD L, L
            0x6D => {
                // Do nothing
            },
            // LD L, (HL)
            0x6E => { self.reg.l = self.mmu.read_byte(self.reg.hl()); },
            // LD L, A
            0x6F => { self.reg.l = self.reg.a; },
            // LD (HL), B
            0x70 => { self.mmu.write_byte(self.reg.hl(), self.reg.b); },
            // LD (HL), C
            0x71 => { self.mmu.write_byte(self.reg.hl(), self.reg.c); },
            // LD (HL), D
            0x72 => { self.mmu.write_byte(self.reg.hl(), self.reg.d); },
            // LD (HL), E
            0x73 => { self.mmu.write_byte(self.reg.hl(), self.reg.e); },
            // LD (HL), H
            0x74 => { self.mmu.write_byte(self.reg.hl(), self.reg.h); },
            // LD (HL), L
            0x75 => { self.mmu.write_byte(self.reg.hl(), self.reg.l); },
            // HALT
            0x76 => self.halt(),
            // LD (HL), A
            0x77 => { self.mmu.write_byte(self.reg.hl(), self.reg.a); },
            // LD A, B
            0x78 => { self.reg.a = self.reg.b; },
            // LD A, C
            0x79 => { self.reg.a = self.reg.c; },
            // LD A, D
            0x7A => { self.reg.a = self.reg.d; },
            // LD A, E
            0x7B => { self.reg.a = self.reg.e; },
            // LD A, H
            0x7C => { self.reg.a = self.reg.h; },
            // LD A, L
            0x7D => { self.reg.a = self.reg.l; },
            // LD A, (HL)
            0x7E => { self.reg.a = self.mmu.read_byte(self.reg.hl()); },
            // LD A, A
            0x7F => { 
                // Do nothing
            },
            // ADD A, B
            0x80 => { self.alu_add(self.reg.b); },
            // ADD A, C
            0x81 => { self.alu_add(self.reg.c); },
            // ADD A, D
            0x82 => { self.alu_add(self.reg.d); },
            // ADD A, E
            0x83 => { self.alu_add(self.reg.e); },
            // ADD A, H
            0x84 => { self.alu_add(self.reg.h); },
            // ADD A, L
            0x85 => { self.alu_add(self.reg.l); },
            // ADD A, (HL)
            0x86 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_add(value);
            },
            // ADD A, A
            0x87 => { self.alu_add(self.reg.a); },
            // ADC A, B
            0x88 => { self.alu_adc(self.reg.b); },
            // ADC A, C
            0x89 => { self.alu_adc(self.reg.c); },
            // ADC A, D
            0x8A => { self.alu_adc(self.reg.d); },
            // ADC A, E
            0x8B => { self.alu_adc(self.reg.e); },
            // ADC A, H
            0x8C => { self.alu_adc(self.reg.h); },
            // ADC A, L
            0x8D => { self.alu_adc(self.reg.l); },
            // ADC A, (HL)
            0x8E => { 
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_adc(value);
            },
            // ADC A, A
            0x8F => { self.alu_adc(self.reg.a); },
            // SUB A, B
            0x90 => { self.alu_sub(self.reg.b); },
            // SUB A, C
            0x91 => { self.alu_sub(self.reg.c); },
            // SUB A, D
            0x92 => { self.alu_sub(self.reg.d); },
            // SUB A, B
            0x93 => { self.alu_sub(self.reg.e); },
            // SUB A, H
            0x94 => { self.alu_sub(self.reg.h); },
            // SUB A, L
            0x95 => { self.alu_sub(self.reg.l); },
            // SUB A, (HL)
            0x96 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_sub(value);
            },
            // SUB A, A
            0x97 => { self.alu_sub(self.reg.a); },
            // SBC A, B
            0x98 => { self.alu_sbc(self.reg.b); },
            // SBC A, C
            0x99 => { self.alu_sbc(self.reg.c); },
            // SBC A, D
            0x9A => { self.alu_sbc(self.reg.d); },
            // SBC A, E
            0x9B => { self.alu_sbc(self.reg.e); },
            // SBC A, H
            0x9C => { self.alu_sbc(self.reg.h); },
            // SBC A, L
            0x9D => { self.alu_sbc(self.reg.l); },
            // SBC A, (HL)
            0x9E => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_sbc(value);
            },
            // SBC A, A
            0x9F => { self.alu_sbc(self.reg.a); },
            // OR B
            0xA0 => { self.alu_and(self.reg.b); },
            // OR C
            0xA1 => { self.alu_and(self.reg.c); },
            // OR D
            0xA2 => { self.alu_and(self.reg.d); },
            // OR E
            0xA3 => { self.alu_and(self.reg.e); },
            // OR H
            0xA4 => { self.alu_and(self.reg.h); },
            // OR L
            0xA5 => { self.alu_and(self.reg.l); },
            // OR (HL)
            0xA6 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_and(value);
            },
            // OR A
            0xA7 => { self.alu_and(self.reg.a); },
            // XOR B
            0xA8 => { self.alu_xor(self.reg.b); },
            // XOR C
            0xA9 => { self.alu_xor(self.reg.c); },
            // XOR D
            0xAA => { self.alu_xor(self.reg.d); },
            // XOR E
            0xAB => { self.alu_xor(self.reg.e); },
            // XOR H
            0xAC => { self.alu_xor(self.reg.h); },
            // XOR L
            0xAD => { self.alu_xor(self.reg.l); },
            // XOR (HL)
            0xAE => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_xor(value);
            },
            // XOR A
            0xAF => { self.alu_xor(self.reg.a); },
            // OR B
            0xB0 => { self.alu_or(self.reg.b); },
            // OR C
            0xB1 => { self.alu_or(self.reg.c); },
            // OR D
            0xB2 => { self.alu_or(self.reg.b); },
            // OR E
            0xB3 => { self.alu_or(self.reg.e); },
            // OR H
            0xB4 => { self.alu_or(self.reg.h); },
            // OR L
            0xB5 => { self.alu_or(self.reg.l); },
            // OR (HL)
            0xB6 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_or(self.reg.b);
            },
            // OR A
            0xB7 => { self.alu_or(self.reg.a); },
            // CP B
            0xB8 => { self.alu_cp(self.reg.b); },
            // CP C
            0xB9 => { self.alu_cp(self.reg.c); },
            // CP D
            0xBA => { self.alu_cp(self.reg.d); },
            // CP E
            0xBB => { self.alu_cp(self.reg.e); },
            // CP H
            0xBC => { self.alu_cp(self.reg.h); },
            // CP L
            0xBD => { self.alu_cp(self.reg.l); },
            // CP (HL)
            0xBE => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_cp(value);
            },
            // CP A
            0xBF => { self.alu_cp(self.reg.a); },
            // RET NZ
            0xC0 => {
                let value = self.reg.f & CPUFlags::Z as u8 > 0;
                if !value {
                    let sp = self.mmu.read_word(self.reg.sp);
                    self.reg.sp = self.reg.sp.wrapping_add(2);  
                    self.reg.pc = sp;        
                }
            },
            // POP BC
            0xC1 => {
                let sp = self.mmu.read_word(self.reg.sp);
                self.reg.sp = self.reg.sp.wrapping_add(2);
                self.reg.set_bc(sp);
            },
            // JP NZ, a16
            0xC2 => {
                let value = self.reg.f & CPUFlags::Z as u8 > 0;
                if !value {
                    self.reg.pc = self.mmu.read_word(self.reg.pc);
                }
            },
            // JP a16
            0xC3 => {
                self.reg.pc = self.mmu.read_word(self.reg.pc);
            },
            // CALL NZ, a16
            0xC4 => {
                let value = self.reg.f & CPUFlags::Z as u8 > 0;
                if !value {
                    self.reg.sp = self.reg.sp.wrapping_sub(2);
                    self.mmu.write_word(self.reg.sp, self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc).wrapping_add(2);
                }
            },
            // PUSH BC
            0xC5 => {
                self.reg.sp = self.reg.sp.wrapping_sub(2);
                self.mmu.write_word(self.reg.sp, self.reg.bc());
            },
            // ADD A, u8
            0xC6 => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_add(value);
            },
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
            // XOR u8
            0xEE => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_xor(value);
            },
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
            // LD A, u8
            0xFA => {
                self.reg.a = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            },
            0xFB => self.ei(),
            0xFC => self.not_supported_instruction(instruction),
            0xFD => self.not_supported_instruction(instruction),
            0xFE => self.cp_a_n(),
            0xFF => self.rst_38h(),
        };
        self.reg.pc = self.reg.pc.wrapping_add(1);
    }

    fn stop(&mut self) {
        if self.is_cgb {
            
        }
    }
    
    fn halt(&mut self) {
        self.is_halted = true;
    }
    
    fn alu_add(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_add(reg);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn alu_adc(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_add(reg.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }
    
    fn alu_sub(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(reg);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_sbc(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(reg.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_and(&mut self, reg: u8) {
        let value = self.reg.a & reg;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn alu_xor(&mut self, reg: u8) {
        let value = self.reg.a ^ reg;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;
    }

    fn alu_or(&mut self, reg: u8) {
        let value = self.reg.a | reg;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        self.reg.a = value;        
    }

    fn alu_cp(&mut self, reg: u8) {
        let value = self.reg.a.wrapping_sub(reg);
        self.reg.set_f(CPUFlags::Z, self.reg.a == reg);
        self.reg.set_f(CPUFlags::N, true);
        self.reg.set_f(CPUFlags::H, value > self.reg.a);
        self.reg.set_f(CPUFlags::C, self.reg.a < reg);
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