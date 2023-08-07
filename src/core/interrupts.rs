// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::core::gb::Gb;

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
        if self.check_interrupts() {
            self.cpu.interrupt_master = false;
            self.cpu.sp = self.cpu.sp.wrapping_sub(2);
            if self.cpu.is_halted {
                self.write_word(self.cpu.sp, self.cpu.pc.wrapping_add(1));
                self.cpu.pending_cycles += 5;
            } else {
                self.write_word(self.cpu.sp, self.cpu.pc);
                self.cpu.pending_cycles += 5;
            }
            let interrupt_source = self.get_interrupt_source();
            let address = self.go_interrupt(&interrupt_source);
            self.cpu.pc = address;
            self.if_flag &= !(interrupt_source as u8);
            self.cpu.is_halted = false;
        }

        if self.cpu.is_halted {
            if !self.cpu.interrupt_master {
                if self.ie_flag & self.if_flag != 0 { // Halt Bug
                    self.cpu.is_halted = false;
                    // TODO: Implement the halt bug
                }
            }
        }

        if self.cpu.ime_toggle {
            self.cpu.interrupt_master = true;
            self.cpu.ime_toggle = false;
        }
    }

    fn check_interrupts(&mut self) -> bool {
        self.cpu.interrupt_master && (self.ie_flag & self.if_flag != 0)
    }

    fn get_interrupt_source(&mut self) -> Interrupt {
        if self.if_flag & (Interrupt::Vblank as u8) != 0 {
            Interrupt::Vblank
        } else if self.if_flag & (Interrupt::LcdStat as u8) != 0 {
            Interrupt::LcdStat
        } else if self.if_flag & (Interrupt::Timer as u8) != 0 {
            Interrupt::Timer
        } else if self.if_flag & (Interrupt::Serial as u8) != 0 {
            Interrupt::Serial
        } else if self.if_flag & (Interrupt::Joypad as u8) != 0 {
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
