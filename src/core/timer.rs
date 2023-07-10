// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::Memory;
use super::gb::Gb;
use super::interrupts::Interrupt;

// The PanDocs deals with DIV as a u8 value
// The Cyle-Accurate Game Boy documentation says it's a u16 value
pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    frequency: u32
}

fn get_frequency_values(tac: u8) -> u32 {
    match tac & 3 {
        0 => 4096,
        1 => 262114,
        2 => 65536,
        3 => 16386,
        _ => 4096,
    }
}

impl Timer {
    pub fn new(div: u16) -> Self {
        let tac: u8 = 0xF8;
        let frequency = get_frequency_values(tac);
        Self {
            div,
            tima: 0,
            tma: 0,
            tac,
            frequency,
        }
    }
}

impl Memory for Timer {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => {
                if value & 0x04 == 0x04 { // TIMER ENABLED
                    self.tac = value;
                    self.frequency = get_frequency_values(self.tac)
                }
            }
            _ => unreachable!(),
        };
    }
}

impl Timer {
    pub fn get_timer_update(&mut self, prev_div: u16) -> bool {
        let timer_update: bool;
        match self.tac & 0x03 {
            0 => timer_update = ((prev_div & (1 << 9)) != 0) && (!(prev_div & (1 << 9)) != 0),
            1 => timer_update = ((prev_div & (1 << 3)) != 0) && (!(prev_div & (1 << 3)) != 0),
            2 => timer_update = ((prev_div & (1 << 5)) != 0) && (!(prev_div & (1 << 5)) != 0),
            3 => timer_update = ((prev_div & (1 << 7)) != 0) && (!(prev_div & (1 << 7)) != 0),
            _ => unreachable!(),
        };
        timer_update
    }
}

impl Gb {
    pub fn run_timer(&mut self) {
        let old_div = self.timer.div;
        
        self.timer.div = self.timer.div.wrapping_add(1);
        let timer_update = self.timer.get_timer_update(old_div);

        if timer_update && ((self.timer.tac & 0x04) > 0) {
            self.timer.tima = self.timer.tima.wrapping_add(1);

            if self.timer.tima == 0xFF {
                self.timer.tima = self.timer.tma;
                self.if_flag = self.if_flag | (Interrupt::TIMER as u8);
            }
        }
    }
}