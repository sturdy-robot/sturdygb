// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::core::mbc::{CartridgeHeader, Mbc};

pub struct Mbc6 {
    header: CartridgeHeader,
    rom_data: Vec<u8>,
}