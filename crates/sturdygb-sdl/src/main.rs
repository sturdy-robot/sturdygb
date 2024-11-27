// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};
use sturdygb_core::prelude::GbInstance;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut gb = GbInstance::build("roms/cpu_instrs.gb").expect("Could not load file!");
    let mut start = Instant::now();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let emulation_time = Instant::now();
        gb.run_one_frame();
        let duration = emulation_time.elapsed();
        println!("Emulation time: {} ms", duration.as_millis());
        let frame = gb.get_screen_data();

        canvas.present();
        let frame_time = Duration::new(0, 1_000_000_000 / 60);
        let fps = frame_time - start.elapsed();
        println!("Frame time: {} ms", fps.as_millis());
        std::thread::sleep(fps);
        start = Instant::now();
    }
}
