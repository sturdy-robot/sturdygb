// SPDX-FileCopyrightText: 2024 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::memory::Memory;

pub struct Sound {}

impl Sound {
    pub fn new() -> Self {
        Self {}
    }
}

impl Memory for Sound {}
