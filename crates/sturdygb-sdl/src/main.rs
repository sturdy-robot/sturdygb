// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::time::{Duration, Instant};
use sturdygb_core::joypad::JoypadButton;
use sturdygb_core::prelude::GbInstance;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 720;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SturdyGB", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
        .expect("Failed to create texture");

    canvas
        .set_scale(SCREEN_WIDTH as f32 / 160.0, SCREEN_HEIGHT as f32 / 144.0)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let args = std::env::args().collect::<Vec<String>>();
    let mut gb;
    if args.len() < 2 {
        gb = GbInstance::build("roms/cpu_instrs.gb").expect("Could not load file!");
    } else {
        gb = GbInstance::build(&args[1]).expect("Could not load file! Is it a GameBoy ROM?");
    }

    let mut start = Instant::now();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Z => gb.press_button(JoypadButton::A),
                    Keycode::X => gb.press_button(JoypadButton::B),
                    Keycode::Return => gb.press_button(JoypadButton::Start),
                    Keycode::RShift => gb.press_button(JoypadButton::Select),
                    Keycode::Up => gb.press_button(JoypadButton::Up),
                    Keycode::Down => gb.press_button(JoypadButton::Down),
                    Keycode::Left => gb.press_button(JoypadButton::Left),
                    Keycode::Right => gb.press_button(JoypadButton::Right),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Z => gb.release_button(JoypadButton::A),
                    Keycode::X => gb.release_button(JoypadButton::B),
                    Keycode::Return => gb.release_button(JoypadButton::Start),
                    Keycode::RShift => gb.release_button(JoypadButton::Select),
                    Keycode::Up => gb.release_button(JoypadButton::Up),
                    Keycode::Down => gb.release_button(JoypadButton::Down),
                    Keycode::Left => gb.release_button(JoypadButton::Left),
                    Keycode::Right => gb.release_button(JoypadButton::Right),
                    _ => {}
                },
                _ => {}
            }
        }
        gb.run_one_frame();

        let frame = gb.get_screen_data();

        // Update texture with frame data
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..144 {
                    for x in 0..160 {
                        let pixel = frame[y][x];
                        let offset = y * pitch + x * 3;
                        // Convert grayscale to RGB
                        let color = match pixel {
                            0 => [255, 255, 255], // White
                            1 => [192, 192, 192], // Light gray
                            2 => [96, 96, 96],    // Dark gray
                            3 => [0, 0, 0],       // Black
                            _ => [0, 0, 0],       // Default to black
                        };
                        buffer[offset] = color[0];
                        buffer[offset + 1] = color[1];
                        buffer[offset + 2] = color[2];
                    }
                }
            })
            .expect("Failed to update texture");

        canvas.clear();
        canvas
            .copy(&texture, None, None)
            .expect("Failed to copy texture");
        canvas.present();
        let frame_time = Duration::new(0, 1_000_000_000 / 60);
        if start.elapsed() < frame_time {
            std::thread::sleep(frame_time - start.elapsed());
        }
        start = Instant::now();
    }
}
