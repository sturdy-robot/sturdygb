use crate::core::mmu::MMU;
use crate::core::registers::{CPUFlags, Registers, ByteRegister, WordRegister};
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

    pub fn execute(&mut self) {
        while !self.is_halted && self.reg.pc < 0x8000 {
            let instruction = self.fetch_instruction();
            self.decode(instruction);
            self.reg.pc = self.reg.pc.wrapping_add(1);
        }
    }

    fn fetch_instruction(&mut self) -> u8 {
        let instruction = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        instruction
    }

    pub fn decode(&mut self, instruction: u8) {
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
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
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
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
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
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
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
                self.reg.set_f(CPUFlags::Z, self.reg.a == 0);
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
            // RLC B
            0x00 => {
                self.reg.b = self.rlcn(self.reg.b);
            }
            // RLC C
            0x01 => {
                self.reg.c = self.rlcn(self.reg.c);
            }
            // RLC D
            0x02 => {
                self.reg.d = self.rlcn(self.reg.d);
            }
            // RLC E
            0x03 => {
                self.reg.e = self.rlcn(self.reg.e);
            }
            // RLC H
            0x04 => {
                self.reg.h = self.rlcn(self.reg.h);
            }
            // RLC L
            0x05 => {
                self.reg.l = self.rlcn(self.reg.l);
            }
            // RLC (HL)
            0x06 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.rlcn(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RLC A
            0x07 => {
                self.reg.a = self.rlcn(self.reg.a);
            }
            // RRC B
            0x08 => {
                self.reg.b = self.rrcn(self.reg.b);
            }
            // RRC C
            0x09 => {
                self.reg.c = self.rrcn(self.reg.c);
            }
            // RRC D
            0x0A => {
                self.reg.d = self.rrcn(self.reg.d);
            }
            // RRC E
            0x0B => {
                self.reg.e = self.rrcn(self.reg.e);
            }
            // RRC H
            0x0C => {
                self.reg.h = self.rrcn(self.reg.h);
            }
            // RRC L
            0x0D => {
                self.reg.l = self.rrcn(self.reg.l);
            }
            // RRC (HL)
            0x0E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.rrcn(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RRC A
            0x0F => {
                self.reg.a = self.rrcn(self.reg.a);
            }
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
            0x30 => {
                self.reg.b = self.swapn(self.reg.b);
            }
            // SWAP C
            0x31 => {
                self.reg.c = self.swapn(self.reg.c);
            }
            // SWAP D
            0x32 => {
                self.reg.d = self.swapn(self.reg.d);
            }
            // SWAP E
            0x33 => {
                self.reg.e = self.swapn(self.reg.e);
            }
            // SWAP H
            0x34 => {
                self.reg.h = self.swapn(self.reg.h);
            }
            // SWAP L
            0x35 => {
                self.reg.l = self.swapn(self.reg.l);
            }
            // SWAP (HL)
            0x36 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.swapn(byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // SWAP A
            0x37 => {
                self.reg.a = self.swapn(self.reg.a);
            }
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
            0x80 => {
                self.reg.b = self.resn(0, self.reg.b);
            }
            // RES 0,C
            0x81 => {
                self.reg.c = self.resn(0, self.reg.c);
            }
            // RES 0,D
            0x82 => {
                self.reg.d = self.resn(0, self.reg.d);
            }
            // RES 0,E
            0x83 => {
                self.reg.e = self.resn(0, self.reg.e);
            }
            // RES 0,H
            0x84 => {
                self.reg.h = self.resn(0, self.reg.h);
            }
            // RES 0,L
            0x85 => {
                self.reg.l = self.resn(0, self.reg.l);
            }
            // RES 0,(HL)
            0x86 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(0, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 0,A
            0x87 => {
                self.reg.a = self.resn(0, self.reg.a);
            }
            // RES 1,B
            0x88 => {
                self.reg.b = self.resn(1, self.reg.b);
            }
            // RES 1,C
            0x89 => {
                self.reg.c = self.resn(1, self.reg.c);
            }
            // RES 1,D
            0x8A => {
                self.reg.d = self.resn(1, self.reg.d);
            }
            // RES 1,E
            0x8B => {
                self.reg.e = self.resn(1, self.reg.e);
            }
            // RES 1,H
            0x8C => {
                self.reg.h = self.resn(1, self.reg.h);
            }
            // RES 1,L
            0x8D => {
                self.reg.l = self.resn(1, self.reg.l);
            }
            // RES 1,(HL)
            0x8E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(1, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 1,A
            0x8F => {
                self.reg.a = self.resn(1, self.reg.a);
            }
            // RES 2,B
            0x90 => {
                self.reg.b = self.resn(2, self.reg.b);
            }
            // RES 2,C
            0x91 => {
                self.reg.c = self.resn(2, self.reg.c);
            }
            // RES 2,D
            0x92 => {
                self.reg.d = self.resn(2, self.reg.d);
            }
            // RES 2,E
            0x93 => {
                self.reg.e = self.resn(2, self.reg.e);
            }
            // RES 2,H
            0x94 => {
                self.reg.h = self.resn(2, self.reg.h);
            }
            // RES 2,L
            0x95 => {
                self.reg.l = self.resn(2, self.reg.l);
            }
            // RES 2,(HL)
            0x96 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(2, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 2,A
            0x97 => {
                self.reg.a = self.resn(2, self.reg.a);
            }
            // RES 3,B
            0x98 => {
                self.reg.b = self.resn(3, self.reg.b);
            }
            // RES 3,C
            0x99 => {
                self.reg.c = self.resn(3, self.reg.c);
            }
            // RES 3,D
            0x9A => {
                self.reg.d = self.resn(3, self.reg.d);
            }
            // RES 3,E
            0x9B => {
                self.reg.e = self.resn(3, self.reg.e);
            }
            // RES 3,H
            0x9C => {
                self.reg.h = self.resn(3, self.reg.h);
            }
            // RES 3,L
            0x9D => {
                self.reg.l = self.resn(3, self.reg.l);
            }
            // RES 3,(HL)
            0x9E => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(3, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 3,A
            0x9F => {
                self.reg.a = self.resn(3, self.reg.a);
            }
            // RES 4,B
            0xA0 => {
                self.reg.b = self.resn(4, self.reg.b);
            }
            // RES 4,C
            0xA1 => {
                self.reg.c = self.resn(4, self.reg.c);
            }
            // RES 4,D
            0xA2 => {
                self.reg.d = self.resn(4, self.reg.d);
            }
            // RES 4,E
            0xA3 => {
                self.reg.e = self.resn(4, self.reg.e);
            }
            // RES 4,H
            0xA4 => {
                self.reg.h = self.resn(4, self.reg.h);
            }
            // RES 4,L
            0xA5 => {
                self.reg.l = self.resn(4, self.reg.l);
            }
            // RES 4,(HL)
            0xA6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(4, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 4,A
            0xA7 => {
                self.reg.a = self.resn(4, self.reg.a);
            }
            // RES 5,B
            0xA8 => {
                self.reg.b = self.resn(5, self.reg.b);
            }
            // RES 5,C
            0xA9 => {
                self.reg.c = self.resn(5, self.reg.c);
            }
            // RES 5,D
            0xAA => {
                self.reg.d = self.resn(5, self.reg.d);
            }
            // RES 5,E
            0xAB => {
                self.reg.e = self.resn(5, self.reg.e);
            }
            // RES 5,H
            0xAC => {
                self.reg.h = self.resn(5, self.reg.h);
            }
            // RES 5,L
            0xAD => {
                self.reg.l = self.resn(5, self.reg.l);
            }
            // RES 5,(HL)
            0xAE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(5, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 5,A
            0xAF => {
                self.reg.a = self.resn(5, self.reg.a);
            }
            // RES 6,B
            0xB0 => {
                self.reg.b = self.resn(6, self.reg.b);
            }
            // RES 6,C
            0xB1 => {
                self.reg.c = self.resn(6, self.reg.c);
            }
            // RES 6,D
            0xB2 => {
                self.reg.d = self.resn(6, self.reg.d);
            }
            // RES 6,E
            0xB3 => {
                self.reg.e = self.resn(6, self.reg.e);
            }
            // RES 6,H
            0xB4 => {
                self.reg.h = self.resn(6, self.reg.h);
            }
            // RES 6,L
            0xB5 => {
                self.reg.l = self.resn(6, self.reg.l);
            }
            // RES 6,(HL)
            0xB6 => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(6, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 6,A
            0xB7 => {
                self.reg.a = self.resn(6, self.reg.a);
            }
            // RES 7,B
            0xB8 => {
                self.reg.b = self.resn(7, self.reg.b);
            }
            // RES 7,C
            0xB9 => {
                self.reg.c = self.resn(7, self.reg.c);
            }
            // RES 7,D
            0xBA => {
                self.reg.d = self.resn(7, self.reg.d);
            }
            // RES 7,E
            0xBB => {
                self.reg.e = self.resn(7, self.reg.e);
            }
            // RES 7,H
            0xBC => {
                self.reg.h = self.resn(7, self.reg.h);
            }
            // RES 7,L
            0xBD => {
                self.reg.l = self.resn(7, self.reg.l);
            }
            // RES 7,(HL)
            0xBE => {
                let byte_hl = self.mmu.read_byte(self.reg.hl());
                let value = self.resn(7, byte_hl);
                self.mmu.write_byte(self.reg.hl(), value);
            }
            // RES 7,A
            0xBF => {
                self.reg.a = self.resn(7, self.reg.a);
            }
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
        let flag: u8 = match self.reg.f & CPUFlags::C as u8 == CPUFlags::C as u8 {
            true => 1,
            false => 0,
        };
        let value = ((reg << 1) & 0xFF) | flag;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, value > 0x7F);
        value
    }

    fn rrcn(&mut self, reg: u8) -> u8 {
        let flag: u8 = match self.reg.f & CPUFlags::C as u8 == CPUFlags::C as u8 {
            true => 0x80,
            false => 0,
        };
        let value = flag | (reg >> 1);
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, (value & 0x01) == 0x01);
        value
    }

    fn rln(&mut self, reg: u8) -> u8 {
        let flag = match reg > 0x7F {
            true => 1,
            false => 0,
        };
        let value = (reg << 1) & 0xFF | flag;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, reg > 0x7F);
        value
    }

    fn rrn(&mut self, reg: u8) -> u8 {
        let flag = match (reg & 0x01) == 0x01 {
            true => 0x80,
            false => 0,
        };
        let value = flag | (reg >> 1);
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, (reg & 0x01) == 0x01);
        value
    }

    fn slan(&mut self, reg: u8) -> u8 {
        let value = (reg << 1) & 0xFF;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, value > 0x7F);
        value
    }

    fn sran(&mut self, reg: u8) -> u8 {
        let value = (reg & 0x80) | (reg >> 1);
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, (value & 0x01) == 0x01);
        value
    }

    fn swapn(&mut self, reg: u8) -> u8 {
        let value = ((reg & 0xF) << 4) | (reg >> 4);
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, false);
        value
    }

    fn srln(&mut self, reg: u8) -> u8 {
        let value = reg >> 1;
        self.reg.set_f(CPUFlags::Z, value == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, false);
        self.reg.set_f(CPUFlags::C, (value & 0x01) == 0x01);
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
        self.reg.set_f(CPUFlags::Z, (reg & b) == 0);
        self.reg.set_f(CPUFlags::N, false);
        self.reg.set_f(CPUFlags::H, true);
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

    fn not_supported_instruction(&mut self, instruction: u8) {
        println!("Instruction not supported, {:x}", instruction);
    }
}

#[cfg(test)]
mod test {
    use super::CPU;
    use crate::core::cartridge::{Cartridge, CartridgeHeader, MBCTypes};
    use crate::core::mmu::MMU;
    use crate::core::registers::Registers;

    fn set_up(rom_data: Vec<u8>) -> CPU {
        let cartridge_header = CartridgeHeader {
            entry: [0; 4],
            logo: [0; 0x30],
            title: [0; 16],
            cgb_flag: 0x80,
            licensee_code: "".to_string(),
            sgb_flag: 0x00,
            rom_type: MBCTypes::ROMONLY,
            rom_size: 0x00,
            ram_size: 0x00,
            dest_code: 0x00,
            checksum: 0x00,
        };
        let cartridge = Cartridge {
            header: cartridge_header,
            rom_data: rom_data,
            ram: Vec::new(),
        };

        CPU {
            cycles: 0,
            ime: true,
            is_cgb: false,
            is_halted: false,
            mmu: MMU::new(cartridge),
            reg: Registers::new(&false),
        }
    }
}
