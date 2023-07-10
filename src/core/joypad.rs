// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::Memory;

pub struct Joypad {}

impl Joypad {
    pub fn new() -> Self {
        Self {}
    }
}

impl Memory for Joypad {}
