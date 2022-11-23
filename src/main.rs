#![allow(dead_code)]
#![allow(unused_variables)]
use crate::core::cartridge::Cartridge;
use crate::core::gb::GB;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};


mod core;

fn get_window() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}

fn init_gb(cartridge: Cartridge) -> GB {
    let is_cgb = cartridge.is_cgb_only();
    GB::new(cartridge, is_cgb)
}

fn load_cartridge(filename: String) -> Cartridge {
    Cartridge::new(&filename)
}

fn main() {
    let cartridge: Cartridge =
        load_cartridge("roms/gb-test-roms/cpu_instrs/cpu_instrs.gb".to_string());
    let mut gb: GB = init_gb(cartridge);
    gb.run();
}
