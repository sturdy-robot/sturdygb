use crate::core::mmu::MMU;
use crate::core::registers::{CPUFlags, Registers};
use crate::Cartridge;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    pub is_halted: bool,
    pub cycles: u8,
    pub is_cgb: bool,
    pub ime: bool,
}

impl CPU {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            reg: Registers::new(&is_cgb),
            mmu: MMU::new(cartridge),
            is_halted: false,
            cycles: 0,
            is_cgb,
            ime: true,
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
            0x04 => {
                self.reg.b = self.reg.b.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.b == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, self.reg.b == 0);
            }
            // DEC B
            0x05 => {
                self.reg.b = self.reg.b.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.b == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, self.reg.b == 0);
            }
            // LD B, U8
            0x06 => {
                self.reg.b = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RLCA
            0x07 => {
                self.reg.a = ((self.reg.a << 1) & 0xFF) | (self.reg.a >> 7);
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                self.reg.set_f(CPUFlags::C, self.reg.a > 0x7F);
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
                self.reg.set_f(CPUFlags::N, false);
                self.reg
                    .set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(CPUFlags::C, did_overflow);
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
            0x0C => {
                self.reg.c = self.reg.c.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.c == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.c & 0xF) == 0);
            }
            // DEC C
            0x0D => {
                self.reg.c = self.reg.c.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.c == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.c & 0xF) == 0);
            }
            // LD C, u8
            0x0E => {
                self.reg.c = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RRCA
            0x0F => {
                self.reg.a = (self.reg.a >> 1) | ((self.reg.a & 1) << 7);
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                self.reg.set_f(CPUFlags::C, self.reg.a > 0x7F);
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
            0x14 => {
                self.reg.d = self.reg.d.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.d == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.d & 0xF) == 0);
            }
            // DEC D
            0x15 => {
                self.reg.d = self.reg.d.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.d == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.d & 0xF) == 0);
            }
            // LD D, u8
            0x16 => {
                self.reg.d = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RLA
            0x17 => {
                let c = self.reg.f & CPUFlags::C as u8;
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                self.reg.set_f(CPUFlags::C, self.reg.a > 0x7F);
                self.reg.a = ((self.reg.a << 1) & 0xFF) | c;
            }
            // JR u8
            0x18 => {
                self.jr();
            }
            // ADD HL, DE
            0x19 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.de());
                self.reg.set_f(CPUFlags::N, false);
                self.reg
                    .set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(CPUFlags::C, did_overflow);
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
            0x1C => {
                self.reg.e = self.reg.e.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.e == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.e & 0xF) == 0);
            }
            // DEC E
            0x1D => {
                self.reg.e = self.reg.e.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.e == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.e & 0xF) == 0);
            }
            // LD E, u8
            0x1E => {
                self.reg.e = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // RRA
            0x1F => {
                let c = self.reg.f & CPUFlags::C as u8;
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                self.reg.set_f(CPUFlags::C, (self.reg.a & 1) == 1);
                self.reg.a = (self.reg.a >> 1) | c;
            }
            // JR NZ, r8
            0x20 => {
                self.jr_nf(CPUFlags::Z as u8);
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
            0x24 => {
                self.reg.h = self.reg.h.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.h == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.h & 0xF) == 0);
            }
            // DEC H
            0x25 => {
                self.reg.h = self.reg.h.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.h == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.h & 0xF) == 0);
            }
            // LD H, u8
            0x26 => {
                self.reg.h = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // DAA
            0x27 => {
                self.daa();
            }
            // JR Z, r8
            0x28 => {
                self.jr_if(CPUFlags::Z as u8);
            }
            // ADD HL, HL
            0x29 => {
                // Do nothing
            }
            // LDI A, HL
            0x2A => {
                self.reg.set_hl(self.reg.hl().wrapping_add(1));
                self.reg.a = self.mmu.read_byte(self.reg.hl());
            }
            // DEC HL
            0x2B => {
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
            }
            // INC L
            0x2C => {
                self.reg.l = self.reg.l.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, self.reg.l == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.l & 0xF) == 0);
            }
            // DEC L
            0x2D => {
                self.reg.l = self.reg.l.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.l == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.l & 0xF) == 0);
            }
            // LD L, u8
            0x2E => {
                self.reg.l = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // CPL
            0x2F => {
                self.reg.a = !self.reg.a;
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, true);
            }
            // JR NC, r8
            0x30 => {
                self.jr_nf(CPUFlags::C as u8);
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
                self.reg.set_f(CPUFlags::Z, self.reg.hl() == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, self.reg.hl() == 0);
            }
            // DEC (HL)
            0x35 => {
                self.reg.set_hl(self.reg.hl().wrapping_sub(1));
                self.reg.set_f(CPUFlags::Z, self.reg.hl() == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, self.reg.hl() == 0);
            }
            // LDD HL, u8
            0x36 => {
                self.mmu.write_word(self.reg.hl(), self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // SCF
            0x37 => {
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                self.reg.set_f(CPUFlags::C, true);
            }
            // JR C, r8
            0x38 => {
                self.jr_if(CPUFlags::C as u8);
            }
            // ADD HL, SP
            0x39 => {
                let (value, did_overflow) = self.reg.hl().overflowing_add(self.reg.sp);
                self.reg.set_f(CPUFlags::N, false);
                self.reg
                    .set_f(CPUFlags::H, (self.reg.hl() & 0xFFF) > (value & 0xFFF));
                self.reg.set_f(CPUFlags::C, did_overflow);
                self.reg.set_hl(value);
            }
            // LDD A, HL
            0x3A => {
                self.mmu.write_byte(self.reg.hl(), self.reg.a);
            }
            // DEC SP
            0x3B => {
                self.reg.sp = self.reg.sp.wrapping_sub(1);
            }
            // INC A
            0x3C => {
                self.reg.a = self.reg.a.wrapping_sub(1);
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) == 0);
            }
            // DEC A
            0x3D => {
                self.reg.a = self.reg.a.wrapping_sub(1) & 0xFF;
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
                self.reg.set_f(CPUFlags::N, true);
                self.reg.set_f(CPUFlags::H, (self.reg.a & 0xF) == 0);
            }
            // LD A, u8
            0x3E => {
                self.reg.a = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(1);
            }
            // CCF
            0x3F => {
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(CPUFlags::H, false);
                if self.reg.f & CPUFlags::C as u8 == 0x10 {
                    self.reg.set_f(CPUFlags::C, false);
                } else {
                    self.reg.set_f(CPUFlags::C, true);
                }
            }
            // LD B, B
            0x40 => {
                // Do nothing
            }
            // LD B, C
            0x41 => {
                self.reg.b = self.reg.c;
            }
            // LD B, D
            0x42 => {
                self.reg.b = self.reg.d;
            }
            // LD B, E
            0x43 => {
                self.reg.b = self.reg.e;
            }
            // LD B, H
            0x44 => {
                self.reg.b = self.reg.h;
            }
            // LD B, L
            0x45 => {
                self.reg.b = self.reg.l;
            }
            // LD B, (HL)
            0x46 => {
                self.reg.b = self.mmu.read_byte(self.reg.hl());
            }
            // LD B, A
            0x47 => {
                self.reg.b = self.reg.a;
            }
            // LD C, B
            0x48 => {
                self.reg.c = self.reg.b;
            }
            // LD C, C
            0x49 => {
                // Do nothing
            }
            // LD C, D
            0x4A => {
                self.reg.c = self.reg.d;
            }
            // LD C, E
            0x4B => {
                self.reg.c = self.reg.e;
            }
            // LD C, H
            0x4C => {
                self.reg.c = self.reg.h;
            }
            // LD C, L
            0x4D => {
                self.reg.c = self.reg.l;
            }
            // LD C, (HL)
            0x4E => {
                self.reg.c = self.mmu.read_byte(self.reg.hl());
            }
            // LD C, A
            0x4F => {
                self.reg.c = self.reg.a;
            }
            // LD D, B
            0x50 => {
                self.reg.d = self.reg.b;
            }
            // LD D, C
            0x51 => {
                self.reg.d = self.reg.c;
            }
            // LD D, D
            0x52 => {
                // Do nothing
            }
            // LD D, E
            0x53 => {
                self.reg.d = self.reg.e;
            }
            // LD D, H
            0x54 => {
                self.reg.d = self.reg.h;
            }
            // LD D, L
            0x55 => {
                self.reg.d = self.reg.l;
            }
            // LD D, (HL)
            0x56 => {
                self.reg.d = self.mmu.read_byte(self.reg.hl());
            }
            // LD D, A
            0x57 => {
                self.reg.d = self.reg.a;
            }
            // LD E, B
            0x58 => {
                self.reg.e = self.reg.b;
            }
            // LD E, C
            0x59 => {
                self.reg.e = self.reg.c;
            }
            // LD E, D
            0x5A => {
                self.reg.e = self.reg.d;
            }
            // LD E, E
            0x5B => {
                // Do nothing
            }
            // LD E, H
            0x5C => {
                self.reg.e = self.reg.h;
            }
            // LD E, L
            0x5D => {
                self.reg.e = self.reg.l;
            }
            // LD E, (HL)
            0x5E => {
                self.reg.e = self.mmu.read_byte(self.reg.hl());
            }
            // LD E, A
            0x5F => {
                self.reg.e = self.reg.a;
            }
            // LD H, B
            0x60 => {
                self.reg.h = self.reg.b;
            }
            // LD H, C
            0x61 => {
                self.reg.h = self.reg.c;
            }
            // LD H, D
            0x62 => {
                self.reg.h = self.reg.d;
            }
            // LD H, E
            0x63 => {
                self.reg.h = self.reg.e;
            }
            // LD H, H
            0x64 => {
                // Do nothing
            }
            // LD H, L
            0x65 => {
                self.reg.h = self.reg.l;
            }
            // LD H, HL
            0x66 => {
                self.reg.h = self.mmu.read_byte(self.reg.hl());
            }
            // LD H, A
            0x67 => {
                self.reg.h = self.reg.a;
            }
            // LD L, B
            0x68 => {
                self.reg.l = self.reg.b;
            }
            // LD L, C
            0x69 => {
                self.reg.l = self.reg.c;
            }
            // LD L, D
            0x6A => {
                self.reg.l = self.reg.d;
            }
            // LD L, E
            0x6B => {
                self.reg.l = self.reg.e;
            }
            // LD L, H
            0x6C => {
                self.reg.l = self.reg.h;
            }
            // LD L, L
            0x6D => {
                // Do nothing
            }
            // LD L, (HL)
            0x6E => {
                self.reg.l = self.mmu.read_byte(self.reg.hl());
            }
            // LD L, A
            0x6F => {
                self.reg.l = self.reg.a;
            }
            // LD (HL), B
            0x70 => {
                self.mmu.write_byte(self.reg.hl(), self.reg.b);
            }
            // LD (HL), C
            0x71 => {
                self.mmu.write_byte(self.reg.hl(), self.reg.c);
            }
            // LD (HL), D
            0x72 => {
                self.mmu.write_byte(self.reg.hl(), self.reg.d);
            }
            // LD (HL), E
            0x73 => {
                self.mmu.write_byte(self.reg.hl(), self.reg.e);
            }
            // LD (HL), H
            0x74 => {
                self.mmu.write_byte(self.reg.hl(), self.reg.h);
            }
            // LD (HL), L
            0x75 => {
                self.mmu.write_byte(self.reg.hl(), self.reg.l);
            }
            // HALT
            0x76 => self.halt(),
            // LD (HL), A
            0x77 => {
                self.mmu.write_byte(self.reg.hl(), self.reg.a);
            }
            // LD A, B
            0x78 => {
                self.reg.a = self.reg.b;
            }
            // LD A, C
            0x79 => {
                self.reg.a = self.reg.c;
            }
            // LD A, D
            0x7A => {
                self.reg.a = self.reg.d;
            }
            // LD A, E
            0x7B => {
                self.reg.a = self.reg.e;
            }
            // LD A, H
            0x7C => {
                self.reg.a = self.reg.h;
            }
            // LD A, L
            0x7D => {
                self.reg.a = self.reg.l;
            }
            // LD A, (HL)
            0x7E => {
                self.reg.a = self.mmu.read_byte(self.reg.hl());
            }
            // LD A, A
            0x7F => {
                // Do nothing
            }
            // ADD A, B
            0x80 => {
                self.alu_add(self.reg.b);
            }
            // ADD A, C
            0x81 => {
                self.alu_add(self.reg.c);
            }
            // ADD A, D
            0x82 => {
                self.alu_add(self.reg.d);
            }
            // ADD A, E
            0x83 => {
                self.alu_add(self.reg.e);
            }
            // ADD A, H
            0x84 => {
                self.alu_add(self.reg.h);
            }
            // ADD A, L
            0x85 => {
                self.alu_add(self.reg.l);
            }
            // ADD A, (HL)
            0x86 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_add(value);
            }
            // ADD A, A
            0x87 => {
                self.alu_add(self.reg.a);
            }
            // ADC A, B
            0x88 => {
                self.alu_adc(self.reg.b);
            }
            // ADC A, C
            0x89 => {
                self.alu_adc(self.reg.c);
            }
            // ADC A, D
            0x8A => {
                self.alu_adc(self.reg.d);
            }
            // ADC A, E
            0x8B => {
                self.alu_adc(self.reg.e);
            }
            // ADC A, H
            0x8C => {
                self.alu_adc(self.reg.h);
            }
            // ADC A, L
            0x8D => {
                self.alu_adc(self.reg.l);
            }
            // ADC A, (HL)
            0x8E => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_adc(value);
            }
            // ADC A, A
            0x8F => {
                self.alu_adc(self.reg.a);
            }
            // SUB A, B
            0x90 => {
                self.alu_sub(self.reg.b);
            }
            // SUB A, C
            0x91 => {
                self.alu_sub(self.reg.c);
            }
            // SUB A, D
            0x92 => {
                self.alu_sub(self.reg.d);
            }
            // SUB A, B
            0x93 => {
                self.alu_sub(self.reg.e);
            }
            // SUB A, H
            0x94 => {
                self.alu_sub(self.reg.h);
            }
            // SUB A, L
            0x95 => {
                self.alu_sub(self.reg.l);
            }
            // SUB A, (HL)
            0x96 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_sub(value);
            }
            // SUB A, A
            0x97 => {
                self.alu_sub(self.reg.a);
            }
            // SBC A, B
            0x98 => {
                self.alu_sbc(self.reg.b);
            }
            // SBC A, C
            0x99 => {
                self.alu_sbc(self.reg.c);
            }
            // SBC A, D
            0x9A => {
                self.alu_sbc(self.reg.d);
            }
            // SBC A, E
            0x9B => {
                self.alu_sbc(self.reg.e);
            }
            // SBC A, H
            0x9C => {
                self.alu_sbc(self.reg.h);
            }
            // SBC A, L
            0x9D => {
                self.alu_sbc(self.reg.l);
            }
            // SBC A, (HL)
            0x9E => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_sbc(value);
            }
            // SBC A, A
            0x9F => {
                self.alu_sbc(self.reg.a);
            }
            // OR B
            0xA0 => {
                self.alu_and(self.reg.b);
            }
            // OR C
            0xA1 => {
                self.alu_and(self.reg.c);
            }
            // OR D
            0xA2 => {
                self.alu_and(self.reg.d);
            }
            // OR E
            0xA3 => {
                self.alu_and(self.reg.e);
            }
            // OR H
            0xA4 => {
                self.alu_and(self.reg.h);
            }
            // OR L
            0xA5 => {
                self.alu_and(self.reg.l);
            }
            // OR (HL)
            0xA6 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_and(value);
            }
            // OR A
            0xA7 => {
                self.alu_and(self.reg.a);
            }
            // XOR B
            0xA8 => {
                self.alu_xor(self.reg.b);
            }
            // XOR C
            0xA9 => {
                self.alu_xor(self.reg.c);
            }
            // XOR D
            0xAA => {
                self.alu_xor(self.reg.d);
            }
            // XOR E
            0xAB => {
                self.alu_xor(self.reg.e);
            }
            // XOR H
            0xAC => {
                self.alu_xor(self.reg.h);
            }
            // XOR L
            0xAD => {
                self.alu_xor(self.reg.l);
            }
            // XOR (HL)
            0xAE => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_xor(value);
            }
            // XOR A
            0xAF => {
                self.alu_xor(self.reg.a);
            }
            // OR B
            0xB0 => {
                self.alu_or(self.reg.b);
            }
            // OR C
            0xB1 => {
                self.alu_or(self.reg.c);
            }
            // OR D
            0xB2 => {
                self.alu_or(self.reg.b);
            }
            // OR E
            0xB3 => {
                self.alu_or(self.reg.e);
            }
            // OR H
            0xB4 => {
                self.alu_or(self.reg.h);
            }
            // OR L
            0xB5 => {
                self.alu_or(self.reg.l);
            }
            // OR (HL)
            0xB6 => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_or(self.reg.b);
            }
            // OR A
            0xB7 => {
                self.alu_or(self.reg.a);
            }
            // CP B
            0xB8 => {
                self.alu_cp(self.reg.b);
            }
            // CP C
            0xB9 => {
                self.alu_cp(self.reg.c);
            }
            // CP D
            0xBA => {
                self.alu_cp(self.reg.d);
            }
            // CP E
            0xBB => {
                self.alu_cp(self.reg.e);
            }
            // CP H
            0xBC => {
                self.alu_cp(self.reg.h);
            }
            // CP L
            0xBD => {
                self.alu_cp(self.reg.l);
            }
            // CP (HL)
            0xBE => {
                let value = self.mmu.read_byte(self.reg.hl());
                self.alu_cp(value);
            }
            // CP A
            0xBF => {
                self.alu_cp(self.reg.a);
            }
            // RET NZ
            0xC0 => {
                if (self.reg.f & CPUFlags::Z as u8) != 0x80 {
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
                if (self.reg.f & CPUFlags::Z as u8) != 0x80 {
                    self.reg.pc = self.jp();
                }
            }
            // JP u16
            0xC3 => {
                self.reg.pc = self.jp();
            }
            // CALL NZ, a16
            0xC4 => {
                if (self.reg.f & CPUFlags::Z as u8) != 0x80 {
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
            0xC7 => {
                self.rst(0x00);
            }
            // RET Z
            0xC8 => {
                if (self.reg.f & CPUFlags::Z as u8) == 0x80 {
                    self.ret();
                }
            }
            // RET
            0xC9 => {
                self.ret();
            }
            // JP Z, u16
            0xCA => {
                if (self.reg.f & CPUFlags::Z as u8) == 0x80 {
                    self.reg.pc = self.jp();
                }
            }
            0xCB => self.decode_cb(),
            // CALL Z, u16
            0xCC => {
                if (self.reg.f & CPUFlags::Z as u8) == 0x80 {
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
            0xCF => {
                self.rst(0x08);
            }
            // RET NC
            0xD0 => {
                if (self.reg.f & CPUFlags::C as u8) != 0x10 {
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
                if (self.reg.f & CPUFlags::C as u8) != 0x10 {
                    self.reg.pc = self.jp();
                }
            }
            0xD3 => self.not_supported_instruction(instruction),
            // CALL NC, u16
            0xD4 => {
                if self.reg.f & CPUFlags::C as u8 != 0x10 {
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
            0xD7 => {
                self.rst(0x10);
            }
            // RET C
            0xD8 => {
                if (self.reg.f & CPUFlags::C as u8) == 0x10 {
                    self.ret();
                }
            }
            // RETI
            0xD9 => {
                self.ret();
                self.ime = true;
            }
            // JP C, u16
            0xDA => {
                if (self.reg.f & CPUFlags::C as u8) == 0x10 {
                    self.reg.pc = self.jp();
                }
            }
            0xDB => self.not_supported_instruction(instruction),
            // CALL C, u16
            0xDC => {
                if self.reg.f & CPUFlags::C as u8 == 0x10 {
                    self.push_stack(self.reg.pc.wrapping_add(2));
                    self.reg.pc = self.mmu.read_word(self.reg.pc).wrapping_add(2);
                } else {
                    self.reg.pc = self.reg.pc.wrapping_add(2);
                }
            }
            0xDD => self.not_supported_instruction(instruction),
            // SBC A, u16
            0xDE => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.reg.pc = self.reg.pc.wrapping_add(2);
                self.alu_sbc(value);
            }
            // RST 18h
            0xDF => {
                self.rst(0x18);
            }
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
            0xE3 => self.not_supported_instruction(instruction),
            0xE4 => self.not_supported_instruction(instruction),
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
            0xE7 => {
                self.rst(0x20);
            }
            // ADD SP, u8
            0xE8 => {
                let value = self.mmu.read_byte(self.reg.pc) as u16;
                self.reg.pc = self.reg.pc.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(
                    CPUFlags::H,
                    (self.reg.pc & 0x000F).wrapping_add(value & 0x000F) > 0x000F,
                );
                self.reg.set_f(
                    CPUFlags::C,
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
            0xEB => self.not_supported_instruction(instruction),
            0xEC => self.not_supported_instruction(instruction),
            0xED => self.not_supported_instruction(instruction),
            // XOR u8
            0xEE => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_xor(value);
            }
            // RST 28h
            0xEF => {
                self.rst(0x28);
            }
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
                self.reg.set_f(CPUFlags::Z, sp > 0x7F);
                self.reg.set_f(CPUFlags::N, (sp & 0x40) == 0x40);
                self.reg.set_f(CPUFlags::H, (sp & 0x20) == 0x20);
                self.reg.set_f(CPUFlags::C, (sp & 0x10) == 0x10);
            }
            // LD A, (C)
            0xF2 => {
                self.reg.a = self.mmu.read_byte(0xFF00 | self.reg.c as u16);
            }
            // DI
            0xF3 => {
                self.ime = false;
            }
            0xF4 => self.not_supported_instruction(instruction),
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
            0xF7 => {
                self.rst(0x30);
            }
            // LD HL, SP + u8
            0xF8 => {
                let value = self.mmu.read_byte(self.reg.pc) as u16;
                self.reg.pc = self.reg.pc.wrapping_add(1);
                self.reg.set_f(CPUFlags::Z, false);
                self.reg.set_f(CPUFlags::N, false);
                self.reg.set_f(
                    CPUFlags::H,
                    (self.reg.pc & 0x000F).wrapping_add(value & 0x000F) > 0x000F,
                );
                self.reg.set_f(
                    CPUFlags::C,
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
                self.ime = true;
            }
            // NOT SUPPORTED
            0xFC => self.not_supported_instruction(instruction),
            // NOT SUPPORTED
            0xFD => self.not_supported_instruction(instruction),
            // CP u8
            0xFE => {
                let value = self.mmu.read_byte(self.reg.pc);
                self.alu_cp(value);
            }
            // RST 38h
            0xFF => {
                self.rst(0x38);
            }
        };
        self.reg.pc = self.reg.pc.wrapping_add(1);
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
        self.reg.pc.wrapping_add(1); // stop instruction skips a byte
    }

    fn halt(&mut self) {
        self.is_halted = true;
    }

    fn alu_add(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_add(reg);
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg
            .set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_adc(&mut self, reg: u8) {
        let (value, did_overflow) = self
            .reg
            .a
            .overflowing_add(reg.wrapping_add(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg
            .set_f(CPUFlags::H, (value & 0xF) < (self.reg.a & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_sub(&mut self, reg: u8) {
        let (value, did_overflow) = self.reg.a.overflowing_sub(reg);
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg
            .set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
        self.reg.set_f(CPUFlags::C, did_overflow);
        self.reg.a = value;
    }

    fn alu_sbc(&mut self, reg: u8) {
        let (value, did_overflow) = self
            .reg
            .a
            .overflowing_sub(reg.wrapping_sub(CPUFlags::C as u8));
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, true);
        self.reg
            .set_f(CPUFlags::H, (self.reg.a & 0xF) > (value & 0xF));
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

    fn daa(&mut self) {
        if self.reg.f & CPUFlags::N as u8 != 0x40 {
            if (self.reg.f & CPUFlags::C as u8 == 0x10) || self.reg.a > 0x99 {
                self.reg.a = self.reg.a.wrapping_add(0x60);
                self.reg.set_f(CPUFlags::C, true);
            }
            if (self.reg.f & CPUFlags::H as u8 == 0x20) || (self.reg.a & 0xF) > 0x9 {
                self.reg.a = self.reg.a.wrapping_add(0x06);
                self.reg.set_f(CPUFlags::C, false);
            }
        } else if (self.reg.f & CPUFlags::C as u8 == 0x10)
            && (self.reg.f & CPUFlags::H as u8 == 0x20)
        {
            self.reg.a = self.reg.a.wrapping_add(0x9A);
            self.reg.set_f(CPUFlags::H, false);
        } else if self.reg.f & CPUFlags::C as u8 == 0x10 {
            self.reg.a = self.reg.a.wrapping_add(0xFA);
            self.reg.set_f(CPUFlags::H, false);
        }
        self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
    }

    fn jr(&mut self) {
        let value = self.fetch_instruction();
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
        let instruction = self.fetch_instruction();
        match instruction {
            0x00 => {}
            0x01 => {}
            0x02 => {}
            0x03 => {}
            0x04 => {}
            0x05 => {}
            0x06 => {}
            0x07 => {}
            0x08 => {}
            0x09 => {}
            0x0A => {}
            0x0B => {}
            0x0C => {}
            0x0D => {}
            0x0E => {}
            0x0F => {}
            0x10 => {}
            0x11 => {}
            0x12 => {}
            0x13 => {}
            0x14 => {}
            0x15 => {}
            0x16 => {}
            0x17 => {}
            0x18 => {}
            0x19 => {}
            0x1A => {}
            0x1B => {}
            0x1C => {}
            0x1D => {}
            0x1E => {}
            0x1F => {}
            0x20 => {}
            0x21 => {}
            0x22 => {}
            0x23 => {}
            0x24 => {}
            0x25 => {}
            0x26 => {}
            0x27 => {}
            0x28 => {}
            0x29 => {}
            0x2A => {}
            0x2B => {}
            0x2C => {}
            0x2D => {}
            0x2E => {}
            0x2F => {}
            0x30 => {}
            0x31 => {}
            0x32 => {}
            0x33 => {}
            0x34 => {}
            0x35 => {}
            0x36 => {}
            0x37 => {}
            0x38 => {}
            0x39 => {}
            0x3A => {}
            0x3B => {}
            0x3C => {}
            0x3D => {}
            0x3E => {}
            0x3F => {}
            0x40 => {}
            0x41 => {}
            0x42 => {}
            0x43 => {}
            0x44 => {}
            0x45 => {}
            0x46 => {}
            0x47 => {}
            0x48 => {}
            0x49 => {}
            0x4A => {}
            0x4B => {}
            0x4C => {}
            0x4D => {}
            0x4E => {}
            0x4F => {}
            0x50 => {}
            0x51 => {}
            0x52 => {}
            0x53 => {}
            0x54 => {}
            0x55 => {}
            0x56 => {}
            0x57 => {}
            0x58 => {}
            0x59 => {}
            0x5A => {}
            0x5B => {}
            0x5C => {}
            0x5D => {}
            0x5E => {}
            0x5F => {}
            0x60 => {}
            0x61 => {}
            0x62 => {}
            0x63 => {}
            0x64 => {}
            0x65 => {}
            0x66 => {}
            0x67 => {}
            0x68 => {}
            0x69 => {}
            0x6A => {}
            0x6B => {}
            0x6C => {}
            0x6D => {}
            0x6E => {}
            0x6F => {}
            0x70 => {}
            0x71 => {}
            0x72 => {}
            0x73 => {}
            0x74 => {}
            0x75 => {}
            0x76 => {}
            0x77 => {}
            0x78 => {}
            0x79 => {}
            0x7A => {}
            0x7B => {}
            0x7C => {}
            0x7D => {}
            0x7E => {}
            0x7F => {}
            0x80 => {}
            0x81 => {}
            0x82 => {}
            0x83 => {}
            0x84 => {}
            0x85 => {}
            0x86 => {}
            0x87 => {}
            0x88 => {}
            0x89 => {}
            0x8A => {}
            0x8B => {}
            0x8C => {}
            0x8D => {}
            0x8E => {}
            0x8F => {}
            0x90 => {}
            0x91 => {}
            0x92 => {}
            0x93 => {}
            0x94 => {}
            0x95 => {}
            0x96 => {}
            0x97 => {}
            0x98 => {}
            0x99 => {}
            0x9A => {}
            0x9B => {}
            0x9C => {}
            0x9D => {}
            0x9E => {}
            0x9F => {}
            0xA0 => {}
            0xA1 => {}
            0xA2 => {}
            0xA3 => {}
            0xA4 => {}
            0xA5 => {}
            0xA6 => {}
            0xA7 => {}
            0xA8 => {}
            0xA9 => {}
            0xAA => {}
            0xAB => {}
            0xAC => {}
            0xAD => {}
            0xAE => {}
            0xAF => {}
            0xB0 => {}
            0xB1 => {}
            0xB2 => {}
            0xB3 => {}
            0xB4 => {}
            0xB5 => {}
            0xB6 => {}
            0xB7 => {}
            0xB8 => {}
            0xB9 => {}
            0xBA => {}
            0xBB => {}
            0xBC => {}
            0xBD => {}
            0xBE => {}
            0xBF => {}
            0xC0 => {}
            0xC1 => {}
            0xC2 => {}
            0xC3 => {}
            0xC4 => {}
            0xC5 => {}
            0xC6 => {}
            0xC7 => {}
            0xC8 => {}
            0xC9 => {}
            0xCA => {}
            0xCB => {}
            0xCC => {}
            0xCD => {}
            0xCE => {}
            0xCF => {}
            0xD0 => {}
            0xD1 => {}
            0xD2 => {}
            0xD3 => {}
            0xD4 => {}
            0xD5 => {}
            0xD6 => {}
            0xD7 => {}
            0xD8 => {}
            0xD9 => {}
            0xDA => {}
            0xDB => {}
            0xDC => {}
            0xDD => {}
            0xDE => {}
            0xDF => {}
            0xE0 => {}
            0xE1 => {}
            0xE2 => {}
            0xE3 => {}
            0xE4 => {}
            0xE5 => {}
            0xE6 => {}
            0xE7 => {}
            0xE8 => {}
            0xE9 => {}
            0xEA => {}
            0xEB => {}
            0xEC => {}
            0xED => {}
            0xEE => {}
            0xEF => {}
            0xF0 => {}
            0xF1 => {}
            0xF2 => {}
            0xF3 => {}
            0xF4 => {}
            0xF5 => {}
            0xF6 => {}
            0xF7 => {}
            0xF8 => {}
            0xF9 => {}
            0xFA => {}
            0xFB => {}
            0xFC => {}
            0xFD => {}
            0xFE => {}
            0xFF => {}
        };
    }

    fn rst(&mut self, n: u8) {
        self.push_stack(self.reg.pc);
        self.reg.pc = n as u16;
    }

    fn not_supported_instruction(&mut self, instruction: u8) {
        println!("Instruction not supported, {:x}", instruction);
    }
}

#[cfg(test)]
mod test {
    use super::CPU;
    use crate::core::cartridge::Cartridge;
}
