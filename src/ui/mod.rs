// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use crate::core::gb::Gb;

pub trait Renderer {
    fn run(&mut self, gb: &mut Gb);
}
