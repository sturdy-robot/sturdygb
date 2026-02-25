// SPDX-FileCopyrightText: 2024 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

// MBC7 EEPROM Commands
const EEPROM_EWDS: u16 = 0x00; // Disable writes
const EEPROM_WRAL: u16 = 0x10; // Write all
const EEPROM_ERAL: u16 = 0x20; // Erase all
const EEPROM_EWEN: u16 = 0x30; // Enable writes
const EEPROM_WRITE: u16 = 0x40; // Write
const EEPROM_READ: u16 = 0x80; // Read
const EEPROM_ERASE: u16 = 0xC0; // Erase

pub struct Mbc7 {
    rom_data: Vec<u8>,
    header: CartridgeHeader,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_enabled: bool,
    // EEPROM state
    eeprom_write_enabled: bool,
    eeprom_cs: bool,
    eeprom_sk: bool,
    eeprom_di: bool,
    eeprom_do: bool,
    eeprom_state: EepromState,
    eeprom_command: u16,
    eeprom_address: u16,
    eeprom_data: u16,
    eeprom_bit_counter: u8,
    // Accelerometer state
    accel_x: i16,
    accel_y: i16,
    accel_enabled: bool,
    save_path: std::path::PathBuf,
}

#[derive(PartialEq)]
enum EepromState {
    Ready,
    Command,
    Address,
    Data,
    Write,
}

impl Mbc7 {
    pub fn new(rom_data: Vec<u8>, header: CartridgeHeader, save_path: std::path::PathBuf) -> Self {
        let mut ram = vec![0; 256]; // 256 bytes of EEPROM
        if save_path.exists() {
            if let Ok(data) = std::fs::read(&save_path) {
                if data.len() == 256 {
                    ram = data;
                }
            }
        }
        Self {
            rom_data,
            header,
            ram,
            rom_bank: 1,
            ram_enabled: false,
            eeprom_write_enabled: false,
            eeprom_cs: false,
            eeprom_sk: false,
            eeprom_di: false,
            eeprom_do: false,
            eeprom_state: EepromState::Ready,
            eeprom_command: 0,
            eeprom_address: 0,
            eeprom_data: 0,
            eeprom_bit_counter: 0,
            accel_x: 0x8000u16 as i16,
            accel_y: 0x8000u16 as i16,
            accel_enabled: false,
            save_path,
        }
    }

    fn handle_eeprom(&mut self, value: u8) {
        let old_sk = self.eeprom_sk;
        self.eeprom_cs = (value & 0b1000) != 0;
        self.eeprom_sk = (value & 0b0100) != 0;
        self.eeprom_di = (value & 0b0010) != 0;

        // Only process on rising edge of SK
        if !old_sk && self.eeprom_sk && self.eeprom_cs {
            match self.eeprom_state {
                EepromState::Ready => {
                    self.eeprom_command = 0;
                    self.eeprom_bit_counter = 0;
                    self.eeprom_state = EepromState::Command;
                }
                EepromState::Command => {
                    self.eeprom_command = (self.eeprom_command << 1) | (self.eeprom_di as u16);
                    self.eeprom_bit_counter += 1;
                    if self.eeprom_bit_counter == 8 {
                        self.process_eeprom_command();
                    }
                }
                EepromState::Address => {
                    self.eeprom_address = (self.eeprom_address << 1) | (self.eeprom_di as u16);
                    self.eeprom_bit_counter += 1;
                    if self.eeprom_bit_counter == 8 {
                        if (self.eeprom_command & 0xF0) == EEPROM_READ {
                            self.eeprom_data = self.ram[self.eeprom_address as usize] as u16;
                            self.eeprom_state = EepromState::Data;
                            self.eeprom_bit_counter = 0;
                        } else {
                            self.eeprom_state = EepromState::Write;
                            self.eeprom_bit_counter = 0;
                        }
                    }
                }
                EepromState::Data => {
                    self.eeprom_data = (self.eeprom_data << 1) | (self.eeprom_di as u16);
                    self.eeprom_bit_counter += 1;
                    if self.eeprom_bit_counter == 8 {
                        if self.eeprom_write_enabled {
                            self.ram[self.eeprom_address as usize] = self.eeprom_data as u8;
                        }
                        self.eeprom_state = EepromState::Ready;
                    }
                }
                EepromState::Write => {
                    // Handle write operations
                    if self.eeprom_write_enabled {
                        match self.eeprom_command & 0xF0 {
                            EEPROM_WRITE => {
                                self.ram[self.eeprom_address as usize] = self.eeprom_data as u8;
                            }
                            EEPROM_WRAL => {
                                for addr in 0..self.ram.len() {
                                    self.ram[addr] = self.eeprom_data as u8;
                                }
                            }
                            EEPROM_ERAL => {
                                for addr in 0..self.ram.len() {
                                    self.ram[addr] = 0xFF;
                                }
                            }
                            _ => {}
                        }
                    }
                    self.eeprom_state = EepromState::Ready;
                }
            }
        }
    }

    fn process_eeprom_command(&mut self) {
        match self.eeprom_command & 0xF0 {
            EEPROM_EWDS => {
                self.eeprom_write_enabled = false;
                self.eeprom_state = EepromState::Ready;
            }
            EEPROM_EWEN => {
                self.eeprom_write_enabled = true;
                self.eeprom_state = EepromState::Ready;
            }
            EEPROM_READ | EEPROM_WRITE | EEPROM_ERASE => {
                self.eeprom_address = 0;
                self.eeprom_state = EepromState::Address;
                self.eeprom_bit_counter = 0;
            }
            EEPROM_WRAL | EEPROM_ERAL => {
                self.eeprom_state = EepromState::Data;
                self.eeprom_bit_counter = 0;
            }
            _ => {
                self.eeprom_state = EepromState::Ready;
            }
        }
    }
}

impl Mbc for Mbc7 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_data[address as usize],
            0x4000..=0x7FFF => {
                let bank = self.rom_bank;
                let addr = bank * 0x4000 + (address as usize - 0x4000);
                if addr < self.rom_data.len() {
                    self.rom_data[addr]
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x2000..=0x3FFF => {
                self.rom_bank = value as usize;
                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
            }
            0x4000..=0x5FFF => {
                let was_enabled = self.ram_enabled;
                self.ram_enabled = (value & 0x0F) == 0x0A;
                if was_enabled && !self.ram_enabled {
                    let _ = std::fs::write(&self.save_path, &self.ram);
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        match address {
            0xA000..=0xA1FF => {
                // Accelerometer X value (high byte)
                if address == 0xA000 {
                    ((self.accel_x >> 8) & 0xFF) as u8
                }
                // Accelerometer X value (low byte)
                else if address == 0xA001 {
                    (self.accel_x & 0xFF) as u8
                }
                // Accelerometer Y value (high byte)
                else if address == 0xA002 {
                    ((self.accel_y >> 8) & 0xFF) as u8
                }
                // Accelerometer Y value (low byte)
                else if address == 0xA003 {
                    (self.accel_y & 0xFF) as u8
                }
                // EEPROM output
                else if address == 0xA080 {
                    let mut value = 0;
                    if self.eeprom_do {
                        value |= 1;
                    }
                    value
                } else {
                    0xFF
                }
            }
            _ => 0xFF,
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        match address {
            0xA080 => {
                self.handle_eeprom(value);
            }
            _ => {}
        }
    }
}

impl Drop for Mbc7 {
    fn drop(&mut self) {
        let _ = std::fs::write(&self.save_path, &self.ram);
    }
}
