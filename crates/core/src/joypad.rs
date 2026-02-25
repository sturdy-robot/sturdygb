// SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::memory::Memory;

pub struct Joypad {
    // P1/JOYP register (0xFF00)
    // Bit 7-6: Not used
    // Bit 5: Select Action buttons (0=Select)
    // Bit 4: Select Direction buttons (0=Select)
    // Bit 3: Down or Start (0=Pressed)
    // Bit 2: Up or Select (0=Pressed)
    // Bit 1: Left or B (0=Pressed)
    // Bit 0: Right or A (0=Pressed)
    data: u8,
    button_states: u8,
    dpad_states: u8,
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
            data: 0xCF,         // Initial state: no buttons selected or pressed
            button_states: 0xF, // All buttons unpressed
            dpad_states: 0xF,   // All d-pad unpressed
        }
    }

    pub fn press(&mut self, button: JoypadButton) {
        match button {
            // Action buttons
            JoypadButton::A => self.button_states &= !0x01,
            JoypadButton::B => self.button_states &= !0x02,
            JoypadButton::Select => self.button_states &= !0x04,
            JoypadButton::Start => self.button_states &= !0x08,
            // Direction buttons
            JoypadButton::Right => self.dpad_states &= !0x01,
            JoypadButton::Left => self.dpad_states &= !0x02,
            JoypadButton::Up => self.dpad_states &= !0x04,
            JoypadButton::Down => self.dpad_states &= !0x08,
        }
        self.update_joyp();
    }

    pub fn release(&mut self, button: JoypadButton) {
        match button {
            // Action buttons
            JoypadButton::A => self.button_states |= 0x01,
            JoypadButton::B => self.button_states |= 0x02,
            JoypadButton::Select => self.button_states |= 0x04,
            JoypadButton::Start => self.button_states |= 0x08,
            // Direction buttons
            JoypadButton::Right => self.dpad_states |= 0x01,
            JoypadButton::Left => self.dpad_states |= 0x02,
            JoypadButton::Up => self.dpad_states |= 0x04,
            JoypadButton::Down => self.dpad_states |= 0x08,
        }
        self.update_joyp();
    }

    fn update_joyp(&mut self) {
        // Keep the upper bits (4-5) which select button type
        let selection = self.data & 0x30;

        // When neither is selected (both bits set), all inputs read as high
        if selection == 0x30 {
            self.data = 0xFF;
            return;
        }

        // Start with unused bits set (6-7) and keep selection bits
        let mut result = 0xC0 | selection;

        // Add appropriate button states based on selection
        if selection & 0x20 == 0 {
            // Action buttons selected (P15)
            result |= self.button_states;
        }
        if selection & 0x10 == 0 {
            // Direction buttons selected (P14)
            result |= self.dpad_states;
        }

        // If neither is selected, all inputs read as high
        if selection == 0x30 {
            result |= 0x0F;
        }

        self.data = result;
    }
}

impl Memory for Joypad {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.data,
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => {
                // Only bits 4-5 are writable (button selection)
                self.data = (value & 0x30) | (self.data & 0xCF);
                self.update_joyp();
            }
            _ => unreachable!(),
        }
    }
}
