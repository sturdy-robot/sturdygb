use super::Memory;

pub struct Joypad {}

impl Joypad {
    pub fn new() -> Self {
        Self {}
    }
}

impl Memory for Joypad {}
