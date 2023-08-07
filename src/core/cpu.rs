// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

pub struct Cpu {
    pub registers: [u8; 8],
    pub sp: u16,
    pub pc: u16,
    pub carry: bool,
    pub half_carry: bool,
    pub negative: bool,
    pub zero: bool,
    pub current_instruction: u8,
    pub instruction_cycles: usize,
    pub pending_cycles: usize,
    pub interrupt_master: bool,
    pub is_halted: bool,
    pub ime_toggle: bool,
    pub is_stopped: bool,
    pub ticks: u32,
}

//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
#[rustfmt::skip]
const OPCODES_SIZE: [u16; 256] = [
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
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 1, 2, 1,
    2, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1,
    2, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 1,
];

fn get_initial_flag_states(value: u8) -> (bool, bool, bool, bool) {
    let carry = value & 0x10 == 0x10;
    let half_carry = value & 0x20 == 0x20;
    let negative = value & 0x40 == 0x40;
    let zero = value & 0x80 == 0x80;
    (carry, half_carry, negative, zero)
}

impl Cpu {
    pub fn new(registers: [u8; 8]) -> Self {
        let (f_carry, f_half_carry, f_negative, f_zero) = get_initial_flag_states(registers[1]);
        Self {
            registers,
            carry: f_carry,
            half_carry: f_half_carry,
            negative: f_negative,
            zero: f_zero,
            sp: 0xFFFE,
            pc: 0x0100,
            current_instruction: 0,
            instruction_cycles: 0,
            pending_cycles: 0,
            interrupt_master: true,
            is_halted: false,
            ime_toggle: false,
            is_stopped: false,
            ticks: 0,
        }
    }

    pub fn advance_pc(&mut self) {
        let adv = OPCODES_SIZE[self.current_instruction as usize];
        self.pc = self.pc.wrapping_add(adv);
    }

    pub fn a(&self) -> u8 {
        self.registers[0]
    }

    pub fn f(&self) -> u8 {
        self.registers[1] & 0xF0
    }

    pub fn b(&self) -> u8 {
        self.registers[2]
    }

    pub fn c(&self) -> u8 {
        self.registers[3]
    }

    pub fn d(&self) -> u8 {
        self.registers[4]
    }

    pub fn e(&self) -> u8 {
        self.registers[5]
    }

    pub fn h(&self) -> u8 {
        self.registers[6]
    }

    pub fn l(&self) -> u8 {
        self.registers[7]
    }

    pub fn af(&self) -> u16 {
        ((self.a() as u16) << 8) | (self.f() as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b() as u16) << 8) | (self.c() as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d() as u16) << 8) | (self.e() as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h() as u16) << 8) | (self.l() as u16)
    }

    pub fn set_a(&mut self, value: u8) {
        self.registers[0] = value;
    }

    pub fn set_f(&mut self, value: u8) {
        self.registers[1] = value & 0xF0;
        self.carry = (value & 0x10) == 0x10;
        self.half_carry = (value & 0x20) == 0x20;
        self.negative = (value & 0x40) == 0x40;
        self.zero = (value & 0x80) == 0x80;
    }

    pub fn set_b(&mut self, value: u8) {
        self.registers[2] = value;
    }

    pub fn set_c(&mut self, value: u8) {
        self.registers[3] = value;
    }

    pub fn set_d(&mut self, value: u8) {
        self.registers[4] = value;
    }

    pub fn set_e(&mut self, value: u8) {
        self.registers[5] = value;
    }

    pub fn set_h(&mut self, value: u8) {
        self.registers[6] = value;
    }

    pub fn set_l(&mut self, value: u8) {
        self.registers[7] = value;
    }

    pub fn set_af(&mut self, value: u16) {
        self.set_a((value >> 8) as u8);
        self.set_f((value & 0xF0) as u8);
    }

    pub fn set_bc(&mut self, value: u16) {
        self.set_b((value >> 8) as u8);
        self.set_c((value & 0xFF) as u8);
    }

    pub fn set_de(&mut self, value: u16) {
        self.set_d((value >> 8) as u8);
        self.set_e((value & 0xFF) as u8);
    }

    pub fn set_hl(&mut self, value: u16) {
        self.set_h((value >> 8) as u8);
        self.set_l((value & 0xFF) as u8);
    }

    pub fn get_carry(&self) -> u8 {
        if self.carry {
            1
        } else {
            0
        }
    }

    pub fn set_carry(&mut self, value: bool) {
        self.set_f(if value {
            self.f() | 0x10
        } else {
            self.f() & 0xE0
        });
    }

    pub fn set_half_carry(&mut self, value: bool) {
        self.set_f(if value {
            self.f() | 0x20
        } else {
            self.f() & 0xD0
        });
    }

    pub fn set_negative(&mut self, value: bool) {
        self.set_f(if value {
            self.f() | 0x40
        } else {
            self.f() & 0xB0
        });
    }

    pub fn set_zero(&mut self, value: bool) {
        self.set_f(if value {
            self.f() | 0x80
        } else {
            self.f() & 0x70
        });
    }
}
