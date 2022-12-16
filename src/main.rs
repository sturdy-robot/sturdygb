#![allow(dead_code)]
#![allow(unused_variables)]
use crate::core::gb::{GB, GbType};
use crate::core::cartridge::*;

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

fn main() {
    let mut gb: GB;
    match load_cartridge("roms/cpu_instrs.gb") {
        Ok(mbc) => {
            gb = GB::new(mbc, GbType::Dmg0);
            gb.run()
        },
        Err(e) => println!("{}", e),
    }
}
