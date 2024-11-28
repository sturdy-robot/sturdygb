// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

#[allow(unused_variables)]
pub trait Memory {
    fn read_byte(&self, address: u16) -> u8 {
        0x00
    }
    fn write_byte(&mut self, address: u16, value: u8) {}
}
