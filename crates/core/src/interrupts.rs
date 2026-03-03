// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::gb::Gb;

pub enum Interrupt {
    Vblank = 0x01,
    LcdStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
    Invalid,
}

impl Gb {
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.if_flag |= interrupt as u8;
    }

    pub fn handle_interrupt(&mut self) {
        if self.cpu.ime_toggle {
            self.cpu.ime_toggle = false;
            self.cpu.interrupt_master = true;
            return;
        }

        if self.cpu.d_ime_toggle {
            self.cpu.d_ime_toggle = false;
            self.cpu.interrupt_master = false;
            return;
        }

        let pending = self.ie_flag & self.if_flag & 0x1F;

        if pending != 0 && self.cpu.is_halted {
            self.cpu.is_halted = false;
        }

        if !self.cpu.interrupt_master || pending == 0 {
            return;
        }

        self.cpu.interrupt_master = false;
        self.cpu.sp = self.cpu.sp.wrapping_sub(2);
        self.write_word(self.cpu.sp, self.cpu.pc);
        self.cpu.pending_cycles += 5;
        let interrupt_source = self.get_interrupt_source();
        let address = self.go_interrupt(&interrupt_source);
        self.cpu.pc = address;
        self.if_flag &= !(interrupt_source as u8);
    }

    fn get_interrupt_source(&mut self) -> Interrupt {
        let pending = self.ie_flag & self.if_flag & 0x1F;
        if pending & (Interrupt::Vblank as u8) != 0 {
            Interrupt::Vblank
        } else if pending & (Interrupt::LcdStat as u8) != 0 {
            Interrupt::LcdStat
        } else if pending & (Interrupt::Timer as u8) != 0 {
            Interrupt::Timer
        } else if pending & (Interrupt::Serial as u8) != 0 {
            Interrupt::Serial
        } else if pending & (Interrupt::Joypad as u8) != 0 {
            Interrupt::Joypad
        } else {
            Interrupt::Invalid
        }
    }

    fn go_interrupt(&mut self, interrupt: &Interrupt) -> u16 {
        match interrupt {
            Interrupt::Vblank => 0x40,
            Interrupt::LcdStat => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
            Interrupt::Invalid => panic!("Invalid interrupt called!"),
        }
    }
}
