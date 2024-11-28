// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::cartridge::GbMode;
use super::gb::{Gb, GbTypes};
use super::memory::Memory;

impl Gb {
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.read_rom(address),
            0x8000..=0x9FFF => self.ppu.read_byte(address),
            0xA000..=0xBFFF => self.mbc.read_ram(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[(address & 0x0FFF) as usize],
            0xD000..=0xDFFF | 0xF000..=0xFDFF => {
                self.wram[(self.ram_bank * 0x1000) | address as usize & 0x0FFF]
            }
            0xFE00..=0xFE9F => self.ppu.read_byte(address),
            0xFEA0..=0xFEFF => 0x00, // PROHIBITED AREA
            0xFF00 => self.joypad.read_byte(address),
            0xFF01..=0xFF02 => self.serial.read_byte(address),
            0xFF04..=0xFF07 => self.timer.read_byte(address),
            0xFF0F => self.if_flag & 0x1F,
            0xFF10..=0xFF26 => self.sound.read_byte(address),
            0xFF30..=0xFF3F => self.sound.read_byte(address),
            0xFF40..=0xFF4B => self.ppu.read_byte(address),
            0xFF4D => self.gb_speed,
            0xFF4F => self.ppu.read_byte(address),
            0xFF50 => self.boot_rom_enabled,
            0xFF51..=0xFF55 => self.ppu.read_byte(address),
            0xFF56 => 0xFF, // INFRARED COMMS, NOT IMPLEMENTED HERE
            0xFF68..=0xFF6B => self.ppu.read_byte(address),
            0xFF70 => self.ram_bank as u8,
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F],
            0xFFFF => self.ie_flag & 0x1F,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_rom(address, value),
            0x8000..=0x9FFF => self.ppu.write_byte(address, value),
            0xA000..=0xBFFF => self.mbc.write_ram(address, value),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => {
                self.wram[(address & 0x0FFF) as usize] = value;
            }
            0xD000..=0xDFFF | 0xF000..=0xFDFF => {
                self.wram[(self.ram_bank * 0x1000) | address as usize & 0x0FFF] = value;
            }
            0xFE00..=0xFE9F => self.ppu.write_byte(address, value),
            0xFEA0..=0xFEFF => {} // PROHIBITED AREA
            0xFF00 => self.joypad.write_byte(address, value),
            0xFF01..=0xFF02 => self.serial.write_byte(address, value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            0xFF0F => self.if_flag = value & 0x1F,
            0xFF10..=0xFF27 => self.sound.write_byte(address, value),
            0xFF30..=0xFF3F => self.sound.write_byte(address, value),
            0xFF40..=0xFF4B => self.ppu.write_byte(address, value),
            0xFF4D => {
                self.gb_speed = value;
                if self.gb_type == GbTypes::Cgb {
                    self.prepare_speed_switch = value & 1 == 1;
                }
            }
            0xFF4F => self.ppu.write_byte(address, value),
            0xFF50 => self.boot_rom_enabled = value,
            0xFF51..=0xFF55 => self.ppu.write_byte(address, value),
            0xFF68..=0xFF69 => self.ppu.write_byte(address, value),
            0xFF70 => {
                if self.gb_mode == GbMode::CgbMode {
                    self.ram_bank = match value & 0x7 {
                        0 => 1,
                        n => n as usize,
                    };
                }
            }
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F] = value,
            0xFFFF => self.ie_flag = value & 0x1F,
            _ => {
                println!("Not implemented memory region {}", address);
            }
        };
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address.wrapping_add(1)) as u16) << 8)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }
}
