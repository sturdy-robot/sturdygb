// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::memory::Memory;

pub struct Joypad {
    data: u8,
}

pub enum JoypadButton {
    A,
    B,
    Left,
    Right,
    Up,
    Down,
    Start,
    Select,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            data: 0xff
        }
    }
}

impl Memory for Joypad {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.data,
            _ => unreachable!()
        }
    }
}
