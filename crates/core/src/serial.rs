// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::memory::Memory;

pub struct Serial {
    sb: u8,
    sc: u8,
    serial_data: Vec<u8>,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0x7E,
            serial_data: Vec::new(),
        }
    }

    pub fn get_serial_message(&mut self) -> Option<String> {
        if !self.serial_data.is_empty() {
            let serial_string = self.serial_data.escape_ascii().to_string();
            Some(serial_string.to_owned())
        } else {
            None
        }
    }
}

impl Memory for Serial {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => {
                self.sb = value;
            }
            0xFF02 => {
                self.sc = value;
                if value == 0x81 {
                    self.serial_data.push(self.sb);
                    self.sc = 0;
                }
            }
            _ => unreachable!(),
        };
    }
}
