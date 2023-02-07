pub mod cb_prefix;
pub mod cpu;
pub mod disasm;
pub mod gb;
pub mod instructions;
pub mod interrupts;
pub mod joypad;
pub mod mbc;
pub mod memorybus;
pub mod ppu;
pub mod serial;
pub mod sound;
pub mod timer;

#[allow(unused_variables)]
pub trait Memory {
    fn read_byte(&self, address: u16) -> u8 { 0x00 }
    fn write_byte(&mut self, address: u16, value: u8) {}
}
