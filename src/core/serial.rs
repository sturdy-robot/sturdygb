use super::Memory;

pub struct Serial {}

impl Serial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Memory for Serial {}
