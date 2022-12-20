use crate::core::mmu::Mmu;
use crate::core::registers::{FFlags, Registers};


const opcode_names: [&str; 256] = [
    "NOP",         "LD BC, d16", "LD (BC), A",  "INC BC",     "INC B",        "DEC B",      "LD B, d8",    "RLCA",       "LD (a16), SP",   "ADD HL, BC", "LD A, (BC)",  "DEC BC",    "INC C",       "DEC C",    "LD C, d8",    "RRCA",
    "STOP 0",      "LD DE, d16", "LD (DE), A",  "INC DE",     "INC D",        "DEC D",      "LD D, d8",    "RLA",        "JR r8",          "ADD HL, DE", "LD A, (DE)",  "DEC DE",    "INC E",       "DEC E",    "LD E, d8",    "RRA",
    "JR NZ, r8",   "LD HL, d16", "LD (HL+), A", "INC HL",     "INC H",        "DEC H",      "LD H, d8",    "DAA",        "JR Z, r8",       "ADD HL, HL", "LD A, (HL+)", "DEC HL",    "INC L",       "DEC L",    "LD L, d8",    "CPL",
    "JR NC, r8",   "LD SP, d16", "LD (HL-), A", "INC SP",     "INC (HL)",     "DEC (HL)",   "LD (HL), d8", "SCF",        "JR C, r8",       "ADD HL, SP", "LD A, (HL-)", "DEC SP",    "INC A",       "DEC A",    "LD A, d8",    "CCF",
    "LD B, B",     "LD B, C",    "LD B, D",     "LD B, E",    "LD B, H",      "LD B, L",    "LD B, (HL)",  "LD B, A",    "LD C, B",        "LD C, C",    "LD C, D",     "LD C, E",   "LD C, H",     "LD C, L",  "LD C, (HL)",  "LD C, A",
    "LD D, B",     "LD D, C",    "LD D, D",     "LD D, E",    "LD D, H",      "LD D, L",    "LD D, (HL)",  "LD D, A",    "LD E, B",        "LD E, C",    "LD E, D",     "LD E, E",   "LD E, H",     "LD E, L",  "LD E, (HL)",  "LD E, A",
    "LD H, B",     "LD H, C",    "LD H, D",     "LD H, E",    "LD H, H",      "LD H, L",    "LD H, (HL)",  "LD H, A",    "LD L, B",        "LD L, C",    "LD L, D",     "LD L, E",   "LD L, H",     "LD L, L",  "LD L, (HL)",  "LD L, A",
    "LD (HL), B",  "LD (HL), C", "LD (HL), D",  "LD (HL), E", "LD (HL), H",   "LD (HL), L", "HALT",        "LD (HL), A", "LD A, B",        "LD A, C",    "LD A, D",     "LD A, E",   "LD A, H",     "LD A, L",  "LD A, (HL)",  "LD A, A",
    "ADD A, B",    "ADD A, C",   "ADD A, D",    "ADD A, E",   "ADD A, H",     "ADD A, L",   "ADD A, (HL)", "ADD A, A",   "ADC A, B",       "ADC A, C",   "ADC A, D",    "ADC A, E",  "ADC A, H",    "ADC A, L", "ADC A, (HL)", "ADC A, A",
    "SUB B",       "SUB C",      "SUB D",       "SUB E",      "SUB H",        "SUB L",      "SUB (HL)",    "SUB A",      "SBC A, B",       "SBC A, C",   "SBC A, D",    "SBC A, E",  "SBC A, H",    "SBC A, L", "SBC A, (HL)", "SBC A, A",
    "AND B",       "AND C",      "AND D",       "AND E",      "AND H",        "AND L",      "AND (HL)",    "AND A",      "XOR B",          "XOR C",      "XOR D",       "XOR E",     "XOR H",       "XOR L",    "XOR (HL)",    "XOR A",
    "OR B",        "OR C",       "OR D",        "OR E",       "OR H",         "OR L",       "OR (HL)",     "OR A",       "CP B",           "CP C",       "CP D",        "CP E",      "CP H",        "CP L",     "CP (HL)",     "CP A",
    "RET NZ",      "POP BC",     "JP NZ, a16",  "JP a16",     "CALL NZ, a16", "PUSH BC",    "ADD A, d8",   "RST 00H",    "RET Z",          "RET",        "JP Z, a16",   "PREFIX CB", "CALL Z, a16", "CALL a16", "ADC A, d8",   "RST 08H",
    "RET NC",      "POP DE",     "JP NC, a16",  "NULL",       "CALL NC, a16", "PUSH DE",    "SUB d8",      "RST 10H",    "RET C",          "RETI",       "JP C, a16",   "NULL",      "CALL C, a16", "NULL",     "SBC A, d8",   "RST 18H",
    "LDH (a8), A", "POP HL",     "LD (C), A",   "NULL",       "NULL",         "PUSH HL",    "AND d8",      "RST 20H",    "ADD SP, r8",     "JP (HL)",    "JP PE, a16",  "NULL",      "NULL",        "NULL",     "XOR d8",      "RST 28H",
    "LDH A, (a8)", "POP AF",     "LD A, (C)",   "DI",         "NULL",         "PUSH AF",    "OR d8",       "RST 30H",    "LD HL, SP + r8", "LD SP, HL",  "JP M, a16",   "EI",        "NULL",        "NULL",     "CP d8",       "RST 38H"
];

const opcodes_size: [u16; 256] = [
//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1,
    1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1,
    2, 1, 1, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1,
    2, 1, 1, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1,
];

const opcodes_cycles: [usize; 256] = [
//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1,
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1,
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1,
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    2, 2, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 1, 3, 6, 2, 4,
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4,
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4,
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4,
];

pub struct Opcode<'a> {
    pub opcode: u8,
    pub name: &'a str,
    pub size: u16,
    pub cycles: usize,
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
        let name = opcode_names[opcode as usize];
        let size = opcodes_size[opcode as usize];
        let cycles = opcodes_cycles[opcode as usize];

        Self {
            opcode,
            name,
            size,
            cycles,
            reg,
            mmu,
            cb_inst: 0,
            is_cb,
            is_halted: false,
        }
    }


    pub fn decode(&mut self) {
        self.debug_registers();
        // MACROS FOR OPCODES
        match self.opcode {
            // NOP
            0x00 => { }

            // LD nn, SP
            0x08 => self.mmu.write_word(self.reg.pc, self.reg.sp),

            // LD INSTRUCTIONS
            // LD r, n
            0x06 => self.reg.b = self.mmu.read_byte(self.reg.pc),
            0x0E => self.reg.c = self.mmu.read_byte(self.reg.pc),
            0x16 => self.reg.d = self.mmu.read_byte(self.reg.pc),
            0x1E => self.reg.e = self.mmu.read_byte(self.reg.pc),
            0x26 => self.reg.h = self.mmu.read_byte(self.reg.pc),
            0x2E => self.reg.l = self.mmu.read_byte(self.reg.pc),
            
            // LD r, r
            0x40 => { }, // LD B, B
            0x41 => self.reg.b = self.reg.c,
            0x42 => self.reg.b = self.reg.d,
            0x43 => self.reg.b = self.reg.e,
            0x44 => self.reg.b = self.reg.h,
            0x45 => self.reg.b = self.reg.l,
            0x46 => self.reg.b = self.mmu.read_byte(self.reg.hl()),
            0x47 => self.reg.b = self.reg.a,
            0x48 => self.reg.c = self.reg.b,
            0x49 => { }, // LD C, C
            0x4A => self.reg.c = self.reg.d,
            0x4B => self.reg.c = self.reg.e,
            0x4C => self.reg.c = self.reg.h,
            0x4D => self.reg.c = self.reg.l,
            0x4E => self.reg.c = self.mmu.read_byte(self.reg.hl()),
            0x4F => self.reg.c = self.reg.a,
            0x50 => self.reg.d = self.reg.b,
            0x51 => self.reg.d = self.reg.c,
            0x52 => { }, // LD D, D
            0x53 => self.reg.d = self.reg.e,
            0x54 => self.reg.d = self.reg.h,
            0x55 => self.reg.d = self.reg.l,
            0x56 => self.reg.d = self.mmu.read_byte(self.reg.hl()),
            0x57 => self.reg.d = self.reg.a,
            0x58 => self.reg.e = self.reg.b,
            0x59 => self.reg.e = self.reg.c,
            0x5A => self.reg.e = self.reg.d,
            0x5B => { }, // LD E, E
            0x5C => self.reg.e = self.reg.h,
            0x5D => self.reg.e = self.reg.l,
            0x5E => self.reg.e = self.mmu.read_byte(self.reg.hl()),
            0x5F => self.reg.e = self.reg.a,
            0x60 => self.reg.h = self.reg.b,
            0x61 => self.reg.h = self.reg.c,
            0x62 => self.reg.h = self.reg.d,
            0x63 => self.reg.h = self.reg.e,
            0x64 => { }, // LD H, H
            0x65 => self.reg.h = self.reg.l,
            0x66 => self.reg.h = self.mmu.read_byte(self.reg.hl()),
            0x67 => self.reg.h = self.reg.a,
            0x68 => self.reg.l = self.reg.b,
            0x69 => self.reg.l = self.reg.c,
            0x6A => self.reg.l = self.reg.d,
            0x6B => self.reg.l = self.reg.e,
            0x6C => self.reg.l = self.reg.h,
            0x6D => { }, // LD L, L
            0x6E => self.reg.l = self.mmu.read_byte(self.reg.hl()),
            0x6F => self.reg.l = self.reg.a,
            0x70 => self.mmu.write_byte(self.reg.hl(), self.reg.b),
            0x71 => self.mmu.write_byte(self.reg.hl(), self.reg.c),
            0x72 => self.mmu.write_byte(self.reg.hl(), self.reg.d),
            0x73 => self.mmu.write_byte(self.reg.hl(), self.reg.e),
            0x74 => self.mmu.write_byte(self.reg.hl(), self.reg.h),
            0x75 => self.mmu.write_byte(self.reg.hl(), self.reg.l),
            0x76 => self.mmu.write_byte(self.reg.hl(), self.reg.a),
            0x7F => { }, // LD A, A
            0x78 => self.reg.a = self.reg.b,
            0x79 => self.reg.a = self.reg.c,
            0x7A => self.reg.a = self.reg.d,
            0x7B => self.reg.a = self.reg.e,
            0x7C => self.reg.a = self.reg.h,
            0x7D => self.reg.a = self.reg.l,
            0x36 => { let value = self.mmu.read_byte(self.reg.pc); self.mmu.write_byte(self.reg.hl(), value); },
            0x0A => self.reg.a = self.mmu.read_byte(self.reg.bc()),
            0x1A => self.reg.a = self.mmu.read_byte(self.reg.de()),
            0x7E => self.reg.a = self.mmu.read_byte(self.reg.hl()),
            0xFA => { let value = self.mmu.read_word(self.reg.pc); self.reg.a = self.mmu.read_byte(value); },
            0x3E => self.reg.a = self.mmu.read_byte(self.reg.pc),
            0xEA => { let value = self.mmu.read_word(self.reg.pc); self.mmu.write_byte(value, self.reg.a); },
            
            // LD rr, nn
            0x01 => self.reg.set_bc(self.mmu.read_word(self.reg.pc)),
            0x11 => self.reg.set_de(self.mmu.read_word(self.reg.pc)),
            0x21 => self.reg.set_hl(self.mmu.read_word(self.reg.pc)),
            0x31 => self.reg.sp = self.mmu.read_word(self.reg.pc),
            
            // LD A, (C)
            0xF2 => self.reg.a = self.mmu.read_byte(0xFF00_u16.wrapping_add(self.reg.c as u16)),
            // LD (C), A
            0xE2 => self.mmu.write_byte(0xFF00_u16.wrapping_add(self.reg.c as u16), self.reg.a),
            // LD HL, SP + r8
            0xF8 => {
                let value = self.mmu.read_byte(self.reg.pc) as i8;
                let (v, did_overflow) = self.reg.sp.overflowing_add(value as i16 as u16);
                let c: u8 = if did_overflow { 1 } else { 0 };
                let h: u8 = if ((self.reg.sp & 0xFF) + (value as i16 as u16 & 0xFF)) > 0xFF { 1 } else { 0 };
                self.reg.set_f(c, h,0,0);
                self.reg.set_hl(v);
            }

            // LDI A, (HL)
            0x2A => { self.reg.a = self.mmu.read_byte(self.reg.hl()); self.reg.set_hl(self.reg.hl().wrapping_add(1)); }
            // LDI (HL), A
            0x22 => { self.mmu.write_byte(self.reg.hl(), self.reg.a); self.reg.set_hl(self.reg.hl().wrapping_add(1)); }
            // LDD (HL), A
            0x32 => { let value = self.mmu.write_byte(self.reg.hl(), self.reg.a); self.reg.set_hl(self.reg.hl().wrapping_sub(1)); }
            // LDD A, (HL)
            0x3A => { self.reg.a = self.mmu.read_byte(self.reg.hl()); self.reg.set_hl(self.reg.hl().wrapping_sub(1)); }
            // LDH (n), A
            0xE0 => { let value = self.mmu.read_byte(self.reg.pc); self.mmu.write_byte(0xFF00_u16.wrapping_add(value as u16), self.reg.a); }
            // LDH A, (n)
            0xF0 => { let value = self.mmu.read_byte(self.reg.pc); self.reg.a = self.mmu.read_byte(0xFF00_u16.wrapping_add(value as u16)); }
            // LD SP, HL
            0xF9 => self.reg.sp = self.reg.hl(),

            // PUSH rr
            0xF5 => { self.mmu.write_word(self.reg.sp, self.reg.af()); self.reg.sp = self.reg.sp.wrapping_sub(2); }
            0xC5 => { self.mmu.write_word(self.reg.sp, self.reg.bc()); self.reg.sp = self.reg.sp.wrapping_sub(2); }
            0xD5 => { self.mmu.write_word(self.reg.sp, self.reg.de()); self.reg.sp = self.reg.sp.wrapping_sub(2); }
            0xE5 => { self.mmu.write_word(self.reg.sp, self.reg.hl()); self.reg.sp = self.reg.sp.wrapping_sub(2); }
            
            // POP rr
            0xF1 => { let value = self.mmu.read_word(self.reg.sp); self.reg.set_af(value); self.reg.sp = self.reg.sp.wrapping_add(2); }
            0xC1 => { let value = self.mmu.read_word(self.reg.sp); self.reg.set_bc(value); self.reg.sp = self.reg.sp.wrapping_add(2); }
            0xD1 => { let value = self.mmu.read_word(self.reg.sp); self.reg.set_de(value); self.reg.sp = self.reg.sp.wrapping_add(2); }
            0xE1 => { let value = self.mmu.read_word(self.reg.sp); self.reg.set_hl(value); self.reg.sp = self.reg.sp.wrapping_add(2); }
            
            // ADD A, n
            0x87 => self.add_a_n(self.reg.a),
            0x80 => self.add_a_n(self.reg.b),
            0x81 => self.add_a_n(self.reg.c),
            0x82 => self.add_a_n(self.reg.d), 
            0x83 => self.add_a_n(self.reg.e),
            0x84 => self.add_a_n(self.reg.h),
            0x85 => self.add_a_n(self.reg.l),
            0x86 => { let value = self.mmu.read_byte(self.reg.hl()); self.add_a_n(value) }
            0xC6 => { let value = self.mmu.read_byte(self.reg.pc); self.add_a_n(value) }

            // ADC A, n
            0x88 => self.adc_a_n(self.reg.b),
            0x89 => self.adc_a_n(self.reg.c),
            0x8A => self.adc_a_n(self.reg.d),
            0x8B => self.adc_a_n(self.reg.e),
            0x8C => self.adc_a_n(self.reg.h),
            0x8D => self.adc_a_n(self.reg.l),
            0x8E => { let value = self.mmu.read_byte(self.reg.hl()); self.adc_a_n(value); }
            0x8F => self.adc_a_n(self.reg.a),
            0xCE => { let value = self.mmu.read_byte(self.reg.pc); self.adc_a_n(value); }

            // SUB n
            0x90 => self.sub_a_n(self.reg.b),
            0x91 => self.sub_a_n(self.reg.c),
            0x92 => self.sub_a_n(self.reg.d),
            0x93 => self.sub_a_n(self.reg.e),
            0x94 => self.sub_a_n(self.reg.h),
            0x95 => self.sub_a_n(self.reg.l),
            0x96 => { let value = self.mmu.read_byte(self.reg.hl()); self.sub_a_n(value); }
            0x97 => self.sub_a_n(self.reg.a),

            // SBC A, n
            0x98 => self.sbc_a_n(self.reg.b),
            0x99 => self.sbc_a_n(self.reg.c),
            0x9A => self.sbc_a_n(self.reg.d),
            0x9B => self.sbc_a_n(self.reg.e),
            0x9C => self.sbc_a_n(self.reg.h),
            0x9D => self.sbc_a_n(self.reg.l),
            0x9E => { let value = self.mmu.read_byte(self.reg.hl()); self.sbc_a_n(value); }
            0x9F => self.sbc_a_n(self.reg.a),

            // AND n
            0xA0 => self.and_n(self.reg.b),
            0xA1 => self.and_n(self.reg.c),
            0xA2 => self.and_n(self.reg.d),
            0xA3 => self.and_n(self.reg.e),
            0xA4 => self.and_n(self.reg.h),
            0xA5 => self.and_n(self.reg.l),
            0xA6 => { let value = self.mmu.read_byte(self.reg.hl()); self.and_n(value); }
            0xA7 => self.and_n(self.reg.a),
            0xE6 => { let value = self.mmu.read_byte(self.reg.pc); self.and_n(value); }
            
            // OR n
            0xB0 => self.or_n(self.reg.b),
            0xB1 => self.or_n(self.reg.c),
            0xB2 => self.or_n(self.reg.d),
            0xB3 => self.or_n(self.reg.e),
            0xB4 => self.or_n(self.reg.h),
            0xB5 => self.or_n(self.reg.l),
            0xB6 => { let value = self.mmu.read_byte(self.reg.hl()); self.or_n(value) }
            0xB7 => self.or_n(self.reg.a),
            0xF6 => { let value = self.mmu.read_byte(self.reg.pc); self.or_n(value) }

            // XOR n
            0xA8 => self.xor_n(self.reg.b),
            0xA9 => self.xor_n(self.reg.c),
            0xAA => self.xor_n(self.reg.d),
            0xAB => self.xor_n(self.reg.e),
            0xAC => self.xor_n(self.reg.h),
            0xAD => self.xor_n(self.reg.l),
            0xAE => { let value = self.mmu.read_byte(self.reg.hl()); self.xor_n(value); }
            0xAF => self.xor_n(self.reg.a),
            0xEE => { let value = self.mmu.read_byte(self.reg.pc); self.xor_n(value); }

            // CP n
            0xB8 => self.cp_n(self.reg.b),
            0xB9 => self.cp_n(self.reg.c),
            0xBA => self.cp_n(self.reg.d),
            0xBB => self.cp_n(self.reg.e),
            0xBC => self.cp_n(self.reg.h),
            0xBD => self.cp_n(self.reg.l),
            0xBE => { let value = self.mmu.read_byte(self.reg.hl()); self.cp_n(value); }
            0xBF => self.cp_n(self.reg.a),
            0xFE => { let value = self.mmu.read_byte(self.reg.pc); self.cp_n(value); }
            
            // INC n
            0x3C => self.reg.a = self.inc_n(self.reg.a),
            0x04 => self.reg.b = self.inc_n(self.reg.b),
            0x0C => self.reg.c = self.inc_n(self.reg.c),
            0x14 => self.reg.d = self.inc_n(self.reg.d),
            0x1C => self.reg.e = self.inc_n(self.reg.e),
            0x24 => self.reg.h = self.inc_n(self.reg.h),
            0x2C => self.reg.l = self.inc_n(self.reg.l),
            0x34 => { let value = self.mmu.read_byte(self.reg.hl()); let inc = self.inc_n(value); self.mmu.write_byte(self.reg.hl(), inc); }
            
            // DEC n
            0x3D => self.reg.a = self.dec_n(self.reg.a),
            0x05 => self.reg.b = self.dec_n(self.reg.b),
            0x0D => self.reg.c = self.dec_n(self.reg.c),
            0x15 => self.reg.d = self.dec_n(self.reg.d),
            0x1D => self.reg.e = self.dec_n(self.reg.e),
            0x25 => self.reg.h = self.dec_n(self.reg.h),
            0x2D => self.reg.l = self.dec_n(self.reg.l),
            0x35 => { let value = self.mmu.read_byte(self.reg.hl()); let dec = self.dec_n(value); self.mmu.write_byte(self.reg.hl(), dec); }
            
            // ADD HL, n
            0x09 => self.add_hl_n(self.reg.bc()),
            0x19 => self.add_hl_n(self.reg.de()),
            0x29 => self.add_hl_n(self.reg.hl()),
            0x39 => self.add_hl_n(self.reg.sp),

            // ADD SP, n
            0xE8 => self.add_sp_n(),

            // INC nn
            0x03 => self.reg.set_bc(self.reg.bc().wrapping_add(1)),
            0x13 => self.reg.set_de(self.reg.de().wrapping_add(1)),
            0x23 => self.reg.set_hl(self.reg.hl().wrapping_add(1)),
            0x33 => self.reg.sp = self.reg.sp.wrapping_add(1),

            // DEC nn
            0x0B => self.reg.set_bc(self.reg.bc().wrapping_sub(1)),
            0x1B => self.reg.set_de(self.reg.de().wrapping_sub(1)),
            0x2B => self.reg.set_hl(self.reg.hl().wrapping_sub(1)),
            0x3B => self.reg.sp = self.reg.sp.wrapping_sub(1),

            // DAA

            // CPL
            0x2F => self.cpl(),
            _ => println!("Not implemented!"),
        }
    }

    fn decode_cb(&mut self) {

    }

    fn ld_hl_r(&mut self, r: u8) {
        self.mmu.write_byte(self.reg.hl(), r);
    }

    fn rst(&mut self, v: u16) {
        self.reg.pc = v;
    }

    fn add_a_n(&mut self, value: u8) {
        let (a, did_overflow) = self.reg.a.overflowing_add(value);
        let z: u8 = if a == 0 { 1 } else { 0 };
        let h: u8 = if (a & 0xF) < (self.reg.a & 0xF) { 1 } else { 0 };
        let c: u8 = if did_overflow { 1 } else { 0 };
        self.reg.set_f(c, h, 0, z);
        self.reg.a = a;
    }

    fn adc_a_n(&mut self, value: u8) {
        let f = self.reg.get_flag(FFlags::C);
        let a = self.reg.a.wrapping_add(value).wrapping_add(f);
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 0;
        let h: u8 = if (self.reg.a & 0xF) + (value) + f > 0xF { 1 } else { 0 };
        let c: u8 = if (self.reg.a as u16) + (value as u16) + (f as u16) > 0xFF { 1 } else { 0 };
        self.reg.set_f(c, h, n, z);
        self.reg.a = a;
    }

    fn sub_a_n(&mut self, value: u8) {
        let (a, did_overflow) = self.reg.a.overflowing_sub(value);
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 1;
        let h: u8 = if (self.reg.a & 0xF) < (value & 0xF) { 1 } else { 0 };
        let c: u8 = if did_overflow { 1 } else { 0 };
        self.reg.set_f(c, h, n, z);
        self.reg.a = a;
    }

    fn sbc_a_n(&mut self, value: u8) {
        let f = self.reg.get_flag(FFlags::C);
        let a = self.reg.a.wrapping_sub(value).wrapping_sub(f);
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 1;
        let h: u8 = if (self.reg.a & 0xF) < (value & 0xF) + f { 1 } else { 0 };
        let c: u8 = if (self.reg.a as u16) < (value as u16) + (f as u16) { 1 } else { 0 };
        self.reg.set_f(c, h, n, z);
        self.reg.a = a;
    }

    fn and_n(&mut self, value: u8) {
        let a = self.reg.a & value;
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 0;
        let h: u8 = 1;
        let c: u8 = 0;
        self.reg.set_f(c, h, n, z);
        self.reg.a = a;
    }

    fn or_n(&mut self, value: u8) {
        let a = self.reg.a | value;
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 0;
        let h: u8 = 0;
        let c: u8 = 0;
        self.reg.set_f(c, h, n, z);
        self.reg.a = a;
    }

    fn xor_n(&mut self, value: u8) {
        let a = self.reg.a ^ value;
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 0;
        let h: u8 = 0;
        let c: u8 = 0;
        self.reg.set_f(c, h, n, z);
        self.reg.a = a;
    }

    fn cp_n(&mut self, value: u8) {
        let a = self.reg.a.wrapping_sub(value);
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 1;
        let h: u8 = if (self.reg.a & 0xF) < (value & 0xF) { 1 } else { 0 };
        let c: u8 = if (self.reg.a) < (value) { 1 } else { 0 };
        self.reg.set_f(c, h, n, z);
    }

    fn inc_n(&mut self, value: u8) -> u8 {
        let a = value.wrapping_add(1);
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 0;
        let h: u8 = if (value & 0xF) + 1 > 0xF { 1 } else { 0 };
        let c: u8 = 2;
        self.reg.set_f(c, h, n, z);
        a
    }

    fn dec_n(&mut self, value: u8) -> u8 {
        let a = value.wrapping_sub(1);
        let z: u8 = if a == 0 { 1 } else { 0 };
        let n: u8 = 1;
        let h: u8 = if (value & 0xF) == 0 { 1 } else { 0 };
        let c: u8 = 2;
        self.reg.set_f(c, h, n, z);
        a
    }

    fn add_hl_n(&mut self, value: u16) {
        let (a, did_overflow) = self.reg.hl().overflowing_add(value);
        let z: u8 = 2;
        let n: u8 = 0;
        let h: u8 = if (self.reg.hl() & 0x07FF) + (value & 0x07FF) > 0x07FF { 1 } else { 0 };
        let c: u8 = if self.reg.hl() > 0xFFFF - value { 1 } else { 0 };
        self.reg.set_f(c, h, n, z);
        self.reg.set_hl(a);
    }

    fn add_sp_n(&mut self) {
        let value: u16 = self.mmu.read_byte(self.reg.pc) as i8 as i16 as u16;
        let (a, did_overflow) = self.reg.sp.overflowing_add(value);
        let z: u8 = 0;
        let n: u8 = 0;
        let h: u8 = if (self.reg.sp & 0x000F) + (value & 0x000F) > 0x000F { 1 } else { 0 };
        let c: u8 = if did_overflow { 1 } else { 0 };
        self.reg.set_f(c, h, n, z);
    }

    fn cpl(&mut self) {
        self.reg.a = !self.reg.a;
        self.reg.set_f(2, 1, 1, 2);
    }

    fn daa(&mut self) {
    //     let c: u8;
    //     let h: u8 = 0;
    //     let n: u8 = 2;
    //     if self.reg.f & FFlags::N as u8 != 0x40 {
    //         if (self.reg.f & FFlags::C as u8 == 0x10) || self.reg.a > 0x99 {
    //             self.reg.a = self.reg.a.wrapping_add(0x60);
    //             c = 1;
    //         }
    //         if (self.reg.f & FFlags::H as u8 == 0x20) || (self.reg.a & 0xF) > 0x9 {
    //             self.reg.a = self.reg.a.wrapping_add(0x06);
    //             c = 0;
    //         }
    //     } else if (self.reg.f & FFlags::C as u8 == 0x10) && (self.reg.f & FFlags::H as u8 == 0x20) {
    //         self.reg.a = self.reg.a.wrapping_add(0x9A);
    //     } else if self.reg.f & FFlags::C as u8 == 0x10 {
    //         self.reg.a = self.reg.a.wrapping_add(0xFA);
    //     }
    //     let z: u8 = if self.reg.a == 0 { 1 } else { 0 };
    //     self.reg.set_f(c, 0, 2, z);
    }

}
