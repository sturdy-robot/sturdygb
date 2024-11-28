// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::cartridge::{CartridgeHeader, Mbc};

pub struct Mbc3 {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
}
