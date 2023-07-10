// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::core::gb::Gb;

pub enum Interrupt {
    VBLANK = 0x01,
    LCDSTAT = 0x02,
    TIMER = 0x04,
    SERIAL = 0x08,
    JOYPAD = 0x10,
    INVALID,
}

impl Gb {
    pub fn handle_interrupt(&mut self) {
        if self.check_interrupts() {
            self.cpu.interrupt_master = false;
            self.cpu.sp = self.cpu.sp.wrapping_sub(2);
            if self.cpu.is_halted {
                self.write_word(self.cpu.sp, self.cpu.pc.wrapping_add(1));
                self.cpu.pending_cycles = 5;                
            } else {
                self.write_word(self.cpu.sp, self.cpu.pc);
                self.cpu.pending_cycles = 5;
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
        if self.if_flag & (Interrupt::VBLANK as u8) != 0 {
            Interrupt::VBLANK
        } else if self.if_flag & (Interrupt::LCDSTAT as u8) != 0 {
            Interrupt::LCDSTAT
        } else if self.if_flag & (Interrupt::TIMER as u8) != 0 {
            Interrupt::TIMER
        } else if self.if_flag & (Interrupt::SERIAL as u8) != 0 {
            Interrupt::SERIAL
        } else if self.if_flag & (Interrupt::JOYPAD as u8) != 0 {
            Interrupt::JOYPAD
        } else {
            Interrupt::INVALID
        }
    }

    fn go_interrupt(&mut self, interrupt: &Interrupt) -> u16 {
        match interrupt {
            Interrupt::VBLANK => 0x40,
            Interrupt::LCDSTAT => 0x48,
            Interrupt::TIMER => 0x50,
            Interrupt::SERIAL => 0x58,
            Interrupt::JOYPAD => 0x60,
            Interrupt::INVALID => panic!("Invalid interrupt called!"),
        }
    }
}
