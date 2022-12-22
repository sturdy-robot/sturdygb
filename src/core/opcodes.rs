use crate::core::mmu::Mmu;
use crate::core::registers::{FFlags, Registers};


const OPCODE_NAMES: [&str; 256] = [
    "NOP",         "LD BC, d16", "LD (BC), A",  "INC BC",     "INC B",        "DEC B",      "LD B, d8",    "RLCA",       "LD (a16), SP",   "ADD HL, BC", "LD A, (BC)",  "DEC BC",    "INC C",       "DEC C",    "LD C, d8",    "RRCA",
    "STOP",        "LD DE, d16", "LD (DE), A",  "INC DE",     "INC D",        "DEC D",      "LD D, d8",    "RLA",        "JR r8",          "ADD HL, DE", "LD A, (DE)",  "DEC DE",    "INC E",       "DEC E",    "LD E, d8",    "RRA",
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

const OPCODES_SIZE: [u16; 256] = [
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
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 2, 3, 3, 2, 1,
    1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1,
    2, 1, 1, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1,
    2, 1, 1, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1,
];

const OPCODES_CYCLES: [usize; 256] = [
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
    pub trigger_ime: bool,
}

impl<'a> Opcode<'a> {
    pub fn new(opcode: u8, reg: &'a mut Registers, mmu: &'a mut Mmu) -> Self {
        let mut cb_inst: u8 = 0x00;
        let mut is_cb: bool = false;
        if opcode == 0xCB {
            is_cb = true;
            cb_inst = mmu.read_byte(reg.pc.wrapping_add(1));
        }
        let name = OPCODE_NAMES[opcode as usize];
        let size = OPCODES_SIZE[opcode as usize];
        let cycles = OPCODES_CYCLES[opcode as usize];

        Self {
            opcode,
            name,
            size,
            cycles,
            reg,
            mmu,
            cb_inst,
            is_cb,
            is_halted: false,
            trigger_ime: false,
        }
    }

    pub fn advance_pc(&mut self) {
        self.reg.pc = self.reg.pc.wrapping_add(self.size);
    }

    pub fn decode(&mut self) {
        self.debug_registers();
        let previous_pc = self.reg.pc;
        // MACROS FOR OPCODES
        match self.opcode {
            // NOP
            0x00 => { }

            // LD INSTRUCTIONS
            // LD rr, nn
            0x01 => self.reg.set_bc(self.mmu.read_word(self.reg.pc.wrapping_add(1))),
            0x11 => self.reg.set_de(self.mmu.read_word(self.reg.pc.wrapping_add(1))),
            0x21 => self.reg.set_hl(self.mmu.read_word(self.reg.pc.wrapping_add(1))),
            0x31 => self.reg.sp = self.mmu.read_word(self.reg.pc.wrapping_add(1)),
            // LD nn, n
            0x02 => self.mmu.write_byte(self.reg.bc(), self.reg.a),
            0x12 => self.mmu.write_byte(self.reg.de(), self.reg.a),
            // LD r, n
            0x06 => self.reg.b = self.mmu.read_byte(self.reg.pc.wrapping_add(1)),
            0x0E => self.reg.c = self.mmu.read_byte(self.reg.pc.wrapping_add(1)),
            0x16 => self.reg.d = self.mmu.read_byte(self.reg.pc.wrapping_add(1)),
            0x1E => self.reg.e = self.mmu.read_byte(self.reg.pc.wrapping_add(1)),
            0x26 => self.reg.h = self.mmu.read_byte(self.reg.pc.wrapping_add(1)),
            0x2E => self.reg.l = self.mmu.read_byte(self.reg.pc.wrapping_add(1)),
            // LD nn, SP
            0x08 => self.mmu.write_word(self.reg.pc.wrapping_add(1), self.reg.sp),
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
            0x77 => self.mmu.write_byte(self.reg.hl(), self.reg.a),
            0x78 => self.reg.a = self.reg.b,
            0x79 => self.reg.a = self.reg.c,
            0x7A => self.reg.a = self.reg.d,
            0x7B => self.reg.a = self.reg.e,
            0x7C => self.reg.a = self.reg.h,
            0x7D => self.reg.a = self.reg.l,
            0x7E => self.reg.a = self.mmu.read_byte(self.reg.hl()),
            0x7F => { }, // LD A, A
            0x3E => self.reg.a = self.mmu.read_byte(self.reg.pc.wrapping_add(1)),
            0x36 => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.mmu.write_byte(self.reg.hl(), value); },
            0x0A => self.reg.a = self.mmu.read_byte(self.reg.bc()),
            0x1A => self.reg.a = self.mmu.read_byte(self.reg.de()),
            0xFA => { let value = self.mmu.read_word(self.reg.pc.wrapping_add(1)); self.reg.a = self.mmu.read_byte(value); },
            0xEA => { let value = self.mmu.read_word(self.reg.pc.wrapping_add(1)); self.mmu.write_byte(value, self.reg.a); },
            
            // LD A, (C)
            0xF2 => self.reg.a = self.mmu.read_byte(0xFF00_u16 | self.reg.c as u16),
            // LD (C), A
            0xE2 => self.mmu.write_byte(0xFF00_u16 | self.reg.c as u16, self.reg.a),
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
            0x32 => { self.mmu.write_byte(self.reg.hl(), self.reg.a); self.reg.set_hl(self.reg.hl().wrapping_sub(1)); }
            // LDD A, (HL)
            0x3A => { self.reg.a = self.mmu.read_byte(self.reg.hl()); self.reg.set_hl(self.reg.hl().wrapping_sub(1)); }
            // LDH (n), A
            0xE0 => { let value = 0xFF00 | self.mmu.read_byte(self.reg.pc.wrapping_add(1)) as u16; self.mmu.write_byte(value, self.reg.a); }
            // LDH A, (n)
            0xF0 => { let value = 0xFF00 | self.mmu.read_byte(self.reg.pc.wrapping_add(1)) as u16; self.reg.a = self.mmu.read_byte(value); }
            // LD SP, HL
            0xF9 => self.reg.sp = self.reg.hl(),

            // PUSH rr
            0xF5 => { self.reg.sp = self.reg.sp.wrapping_sub(2); self.mmu.write_word(self.reg.sp, self.reg.af()); }
            0xC5 => { self.reg.sp = self.reg.sp.wrapping_sub(2); self.mmu.write_word(self.reg.sp, self.reg.bc()); }
            0xD5 => { self.reg.sp = self.reg.sp.wrapping_sub(2); self.mmu.write_word(self.reg.sp, self.reg.de()); }
            0xE5 => { self.reg.sp = self.reg.sp.wrapping_sub(2); self.mmu.write_word(self.reg.sp, self.reg.hl()); }
            
            // POP rr
            0xF1 => { let value = self.mmu.read_word(self.reg.sp) & 0xFFF0; self.reg.set_af(value); self.reg.sp = self.reg.sp.wrapping_add(2); }
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
            0xC6 => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.add_a_n(value) }

            // ADC A, n
            0x88 => self.adc_a_n(self.reg.b),
            0x89 => self.adc_a_n(self.reg.c),
            0x8A => self.adc_a_n(self.reg.d),
            0x8B => self.adc_a_n(self.reg.e),
            0x8C => self.adc_a_n(self.reg.h),
            0x8D => self.adc_a_n(self.reg.l),
            0x8E => { let value = self.mmu.read_byte(self.reg.hl()); self.adc_a_n(value); }
            0x8F => self.adc_a_n(self.reg.a),
            0xCE => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.adc_a_n(value); }

            // SUB n
            0x90 => self.sub_a_n(self.reg.b),
            0x91 => self.sub_a_n(self.reg.c),
            0x92 => self.sub_a_n(self.reg.d),
            0x93 => self.sub_a_n(self.reg.e),
            0x94 => self.sub_a_n(self.reg.h),
            0x95 => self.sub_a_n(self.reg.l),
            0x96 => { let value = self.mmu.read_byte(self.reg.hl()); self.sub_a_n(value); }
            0x97 => self.sub_a_n(self.reg.a),
            0xD6 => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.sub_a_n(value); }

            // SBC A, n
            0x98 => self.sbc_a_n(self.reg.b),
            0x99 => self.sbc_a_n(self.reg.c),
            0x9A => self.sbc_a_n(self.reg.d),
            0x9B => self.sbc_a_n(self.reg.e),
            0x9C => self.sbc_a_n(self.reg.h),
            0x9D => self.sbc_a_n(self.reg.l),
            0x9E => { let value = self.mmu.read_byte(self.reg.hl()); self.sbc_a_n(value); }
            0x9F => self.sbc_a_n(self.reg.a),
            0xDE => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.sbc_a_n(value); }

            // AND n
            0xA0 => self.and_n(self.reg.b),
            0xA1 => self.and_n(self.reg.c),
            0xA2 => self.and_n(self.reg.d),
            0xA3 => self.and_n(self.reg.e),
            0xA4 => self.and_n(self.reg.h),
            0xA5 => self.and_n(self.reg.l),
            0xA6 => { let value = self.mmu.read_byte(self.reg.hl()); self.and_n(value); }
            0xA7 => self.and_n(self.reg.a),
            0xE6 => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.and_n(value); }
            
            // OR n
            0xB0 => self.or_n(self.reg.b),
            0xB1 => self.or_n(self.reg.c),
            0xB2 => self.or_n(self.reg.d),
            0xB3 => self.or_n(self.reg.e),
            0xB4 => self.or_n(self.reg.h),
            0xB5 => self.or_n(self.reg.l),
            0xB6 => { let value = self.mmu.read_byte(self.reg.hl()); self.or_n(value) }
            0xB7 => self.or_n(self.reg.a),
            0xF6 => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.or_n(value) }

            // XOR n
            0xA8 => self.xor_n(self.reg.b),
            0xA9 => self.xor_n(self.reg.c),
            0xAA => self.xor_n(self.reg.d),
            0xAB => self.xor_n(self.reg.e),
            0xAC => self.xor_n(self.reg.h),
            0xAD => self.xor_n(self.reg.l),
            0xAE => { let value = self.mmu.read_byte(self.reg.hl()); self.xor_n(value); }
            0xAF => self.xor_n(self.reg.a),
            0xEE => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.xor_n(value); }

            // CP n
            0xB8 => self.cp_n(self.reg.b),
            0xB9 => self.cp_n(self.reg.c),
            0xBA => self.cp_n(self.reg.d),
            0xBB => self.cp_n(self.reg.e),
            0xBC => self.cp_n(self.reg.h),
            0xBD => self.cp_n(self.reg.l),
            0xBE => { let value = self.mmu.read_byte(self.reg.hl()); self.cp_n(value); }
            0xBF => { self.reg.set_f(0, 0, 1, 1); },
            0xFE => { let value = self.mmu.read_byte(self.reg.pc.wrapping_add(1)); self.cp_n(value); }
            
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
            0x27 => self.daa(),
            // CPL
            0x2F => { self.reg.a = !self.reg.a; self.reg.set_f(2, 1, 1, 2); },
            // CCF
            0x3F => { if self.reg.get_flag(FFlags::C) == 1 { self.reg.set_f(0, 0, 0, 2); } else { self.reg.set_f(1, 0, 0, 2); } }
            // SCF
            0x37 => self.reg.set_f(1, 0, 0, 2),

            // HALT
            0x76 => self.halt(),

            // STOP
            0x10 => self.stop(),

            // ENABLE/DISABLE IME
            0xF3 => self.trigger_ime = true, // DI
            0xFB => self.trigger_ime = true, // EI

            // PREFIX CB
            0xCB => self.decode_cb_prefix(),

            // RST
            0xC7 => self.rst(0x00),
            0xD7 => self.rst(0x10),
            0xE7 => self.rst(0x20),
            0xF7 => self.rst(0x30),
            0xCF => self.rst(0x08),
            0xDF => self.rst(0x18),
            0xEF => self.rst(0x28),
            0xFF => self.rst(0x38),

            // ROTATE
            // RLCA
            0x07 => {
                let flag: u8 = self.reg.get_flag(FFlags::C);
                let value = ((self.reg.a << 1) & 0xFF) | flag;
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if value > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.a = value;
            }
            // RLA
            0x17 => {
                let flag = match self.reg.a > 0x7F {
                    true => 1,
                    false => 0,
                };
                let value = (self.reg.a << 1) | flag;
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (self.reg.a & 0x7F) > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.a = value;
            }
            // RRA
            0x1F => {
                let flag = match (self.reg.a & 1) == 1 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (self.reg.a >> 1);
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (self.reg.a & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.a = value;
            }
            // RRCA
            0x0F => {
                let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (self.reg.a >> 1);
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (value & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.a = value;
            }

            // JP nn
            0xC3 => self.jp(),
            // JP cc, nn
            0xC2 => { if self.reg.get_flag(FFlags::Z) == 0 { self.jp(); } }
            0xCA => { if self.reg.get_flag(FFlags::Z) == 1 { self.jp(); } }
            0xD2 => { if self.reg.get_flag(FFlags::C) == 0 { self.jp(); } }
            0xDA => { if self.reg.get_flag(FFlags::C) == 1 { self.jp(); } }

            // JP (HL)
            0xE9 => self.reg.pc = self.reg.hl(),

            // JR n
            0x18 => self.jr(),

            // JR cc, n
            0x20 => { if self.reg.get_flag(FFlags::Z) == 0 { self.jr(); } }
            0x28 => { if self.reg.get_flag(FFlags::Z) == 1 { self.jr(); } }
            0x30 => { if self.reg.get_flag(FFlags::C) == 0 { self.jr(); } }
            0x38 => { if self.reg.get_flag(FFlags::C) == 1 { self.jr(); } }

            // CALL nn
            0xCD => self.call(),
            
            // CALL cc nn
            0xC4 => self.call_cc(self.reg.get_flag(FFlags::Z) == 0),
            0xCC => self.call_cc(self.reg.get_flag(FFlags::Z) == 1),
            0xD4 => self.call_cc(self.reg.get_flag(FFlags::C) == 0),
            0xDC => self.call_cc(self.reg.get_flag(FFlags::C) == 1),
            
            // RET
            0xC9 => self.ret(),

            // RET c
            0xC0 => { if self.reg.get_flag(FFlags::Z) == 0 { self.ret(); } }
            0xC8 => { if self.reg.get_flag(FFlags::Z) == 1 { self.ret(); } }
            0xD0 => { if self.reg.get_flag(FFlags::C) == 0 { self.ret(); } }
            0xD8 => { if self.reg.get_flag(FFlags::C) == 1 { self.ret(); } }

            // RETI
            0xD9 => { self.ret(); self.trigger_ime = true; }

            0xD3 | 0xE3 | 0xE4 | 0xF4 | 0xDB | 0xDD | 0xEB | 0xEC | 0xED | 0xFC | 0xFD => panic!("Unsupported instruction {}", self.name),
        }

        if previous_pc == self.reg.pc {
            self.advance_pc();
        }
    } 

    fn ret(&mut self) {
        let address = self.mmu.read_word(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(2);
        self.reg.pc = address;
    }

    fn jp(&mut self) {
        self.reg.pc = self.mmu.read_word(self.reg.pc.wrapping_add(1));
    }

    fn jr(&mut self) {
        let address = self.mmu.read_byte(self.reg.pc.wrapping_add(1)) as i8;
        self.reg.pc = self.reg.pc.wrapping_add(2);
        self.reg.pc = ((self.reg.pc as u32 as i32) + (address as i32)) as u16;
    }

    fn call(&mut self) {
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.mmu.write_word(self.reg.sp, self.reg.pc.wrapping_add(3));
        let address = self.mmu.read_word(self.reg.pc.wrapping_add(1));
        self.reg.pc = address;
    }

    fn call_cc(&mut self, cc: bool) {
        if cc {
            self.call();
        }
    }

    fn decode_cb_prefix(&mut self) {
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
                let z: u8 = if (self.reg.$reg & bit) == 0 { 1 } else { 0 };
                self.reg.set_f(2, 1, 0, z);
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
                let z: u8 = if (byte_hl & bit) == 0 { 1 } else { 0 };
                self.reg.set_f(2, 1, 0, z);
            }};
        }

        // SWAP
        macro_rules! swapn {
            ($reg:ident) => {{
                let value = ((self.reg.$reg & 0xF) << 4) | (self.reg.$reg >> 4);
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                self.reg.set_f(0, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = ((byte_hl & 0xF) << 4) | (byte_hl >> 4);
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                self.reg.set_f(0, 0, 0, z);
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
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (self.reg.$reg & 0x7F) > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag = match byte_hl > 0x7F {
                    true => 1,
                    false => 0,
                };
                let value = (byte_hl << 1) | flag;
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (byte_hl & 0x7F) > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }


        // RR
        macro_rules! rrn {
            ($reg:ident) => {{
                let flag = match (self.reg.$reg & 1) == 1 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (self.reg.$reg >> 1);
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (self.reg.$reg & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag = match (byte_hl & 1) == 1 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (byte_hl >> 1);
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (byte_hl & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        // RLC
        macro_rules! rlcn {
            ($reg:ident) => {{
                let flag: u8 = self.reg.get_flag(FFlags::C);
                let value = ((self.reg.$reg << 1) & 0xFF) | flag;
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if value > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag: u8 = self.reg.get_flag(FFlags::C);
                let value = ((byte_hl << 1) & 0xFF) | flag;
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if value > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
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
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (value & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let flag: u8 = match self.reg.f & FFlags::C as u8 == FFlags::C as u8 {
                    true => 0x80,
                    false => 0,
                };
                let value = flag | (byte_hl >> 1);
                let z: u8 = if (value) == 0 { 1 } else { 0 };
                let c: u8 = if (value & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
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
                let z: u8 = if value == 0 { 1 } else { 0 };
                let c: u8 = if value > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = byte_hl << 1;
                let z: u8 = if value == 0 { 1 } else { 0 };
                let c: u8 = if value > 0x7F { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }

        // SRA
        macro_rules! sran {
            ($reg:ident) => {{
                let value = (self.reg.$reg & 0x80) | (self.reg.$reg >> 1);
                let z: u8 = if value == 0 { 1 } else { 0 };
                let c: u8 = if (value & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = (byte_hl & 0x80) | (byte_hl >> 1);
                let z: u8 = if value == 0 { 1 } else { 0 };
                let c: u8 = if (value & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }
        
        // SRL
        macro_rules! srln {
            ($reg:ident) => {{
                let value = self.reg.$reg >> 1;
                let z: u8 = if value == 0 { 1 } else { 0 };
                let c: u8 = if (value & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.reg.$reg = value;
            }};
            ($hl:expr) => {{
                let byte_hl = $hl;
                let value = byte_hl >> 1;
                let z: u8 = if value == 0 { 1 } else { 0 };
                let c: u8 = if (value & 1) == 1 { 1 } else { 0 };
                self.reg.set_f(c, 0, 0, z);
                self.mmu.write_byte(self.reg.hl(), value);
            }};
        }
        match self.cb_inst {
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
        let h: u8 = if (self.reg.a & 0xF) + (value & 0xF) + f > 0xF { 1 } else { 0 };
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
        let a = self.reg.a;
        self.sub_a_n(self.reg.a);
        self.reg.a = a;
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

    fn daa(&mut self) {
        let mut a: u8 = if (self.reg.get_flag(FFlags::C)) == 1 { 0x60 } else { 0 };
        if (self.reg.get_flag(FFlags::H)) == 1 { a |= 0x06; };
        if (self.reg.get_flag(FFlags::N)) == 1 { self.reg.a = self.reg.a.wrapping_sub(a); }
        else { if self.reg.a & 0x0F > 0x09 { a |= 0x06; }; if self.reg.a > 0x99 { a |= 0x60; }; self.reg.a = self.reg.a.wrapping_add(a); }
        let z: u8 = if self.reg.a == 0 { 1 } else { 0 };
        let h: u8 = 0;
        let n: u8 = 2;
        let c: u8 = if a >= 0x60 { 1 } else { 0 };
        self.reg.set_f(c, h, n, z);
    }

    fn halt(&mut self) {
        self.is_halted = true;
    }

    fn stop(&mut self) {
    }

}
