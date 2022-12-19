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
            
            // LD nn, SP
            0x08 => self.mmu.write_word(self.reg.pc, self.reg.sp),
            
            // LD A, (C)
            0xF2 => self.reg.a = self.mmu.read_byte(0xFF00_u16.wrapping_add(self.reg.c as u16)),
            // LD (C), A
            0xE2 => self.mmu.write_byte(0xFF00_u16.wrapping_add(self.reg.c as u16), self.reg.a),
            // LD HL, SP + r8
            0xF8 => {
                let value = self.mmu.read_byte(self.reg.pc) as i8;
                let (v, did_overflow) = self.reg.sp.overflowing_add(value as i16 as u16);
                let c: u8; if did_overflow { c = 1; } else { c = 0; };
                let h: u8; if ((self.reg.sp & 0xFF) + (value as i16 as u16 & 0xFF)) > 0xFF { h = 1; } else { h = 0; };
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
}
