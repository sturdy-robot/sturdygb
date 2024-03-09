// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::gb::Gb;
use super::interrupts::Interrupt;
use super::memory::Memory;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    frequency: u32,
    enabled: bool,
    ticks: u32,
    cycles: u32,
}

fn get_frequency_values(tac: u8) -> u32 {
    match tac & 3 {
        1 => 16,
        2 => 64,
        3 => 256,
        _ => 1024,
    }
}

impl Timer {
    pub fn new(div: u8) -> Self {
        let tac: u8 = 0xF8;
        let frequency = get_frequency_values(tac);
        Self {
            div,
            tima: 0,
            tma: 0,
            tac,
            frequency,
            enabled: false,
            ticks: 0,
            cycles: 0,
        }
    }
}

impl Memory for Timer {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div as u8,
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
                self.enabled = value & 0x04 == 0x04;
                self.tac = value;
                self.frequency = get_frequency_values(self.tac)
            }
            _ => unreachable!(),
        };
    }
}


impl Gb {
    pub fn timer_tick(&mut self, cycles: u32) {
        self.timer.cycles += cycles;
        while self.timer.cycles >= 256 {
            self.timer.cycles -= 256;
            self.timer.div = self.timer.div.wrapping_add(1);
        }

        if self.timer.enabled {
            self.timer.ticks += cycles;

            while self.timer.ticks >= self.timer.frequency {
                self.timer.ticks -= cycles;
                self.timer.tima = self.timer.tima.wrapping_add(1);
                if self.timer.tima == 0 {
                    self.timer.tima = self.timer.tma;
                    self.request_interrupt(Interrupt::Timer);
                }
            }
        }
    }
}