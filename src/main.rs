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
use std::env;

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
    let args: Vec<String> = env::args().collect();
    let filename: &str;
    if args.len() == 1 {
        filename = "roms/cpu_instrs.gb"
    } else {
        filename = &args[1];
    }

    match load_cartridge(filename) {
        Ok(mbc) => {
            gb = GB::new(mbc, GbType::Dmg);
            gb.run()
        },
        Err(e) => println!("{}", e),
    }
    
}
