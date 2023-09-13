// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use super::Renderer;
use crate::core::gb::Gb;
use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureAccess};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowSurfaceRef};
use sdl2::{EventPump, GameControllerSubsystem, Sdl, VideoSubsystem};

pub struct SdlRenderer {
    context: Sdl,
    video_system: VideoSubsystem,
    canvas: Canvas<Window>,
    controller_system: GameControllerSubsystem,
    controllers: Vec<GameController>,
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const SCALE: u32 = 4;


const COLORS: [Color; 4] = [Color::RGB(255, 255, 255), Color::RGB(192,192,192), Color::RGB(96, 96, 96), Color::RGB(0, 0, 0)];

impl SdlRenderer {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_system = context.video().unwrap();
        let canvas = video_system
            .window("SturdyGB", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = canvas.into_canvas().build().unwrap();
        let controller_system = context.game_controller().unwrap();

        Self {
            context,
            video_system,
            canvas,
            controller_system,
            controllers: Vec::new(),
        }
    }

    pub fn check_controllers(&mut self) {
        let controllers = self
            .controller_system
            .num_joysticks()
            .map_err(|e| format!("Can't enumerate joysticks: {}", e))
            .unwrap();
        self.controllers = Vec::new();

        if controllers == 0 {
            return;
        }

        for i in 0..controllers {
            if !self.controller_system.is_game_controller(i) {
                println!("Controller {} is not a game controller", i);
                continue;
            }
            match self.controller_system.open(i) {
                Ok(controller) => {
                    self.controllers.push(controller);
                }
                Err(e) => {
                    // TODO: add to log
                    println!("Failed to open controller {}: {}", i, e);
                }
            }
        }
    }

    pub fn display_tile(&mut self, surface: &mut Surface, start_addr: u16, tile_num: u16, x: u32, y: u32, gb: &mut Gb) {
        for i in (0..16).step_by(2) {
            let b1 = gb.read_byte(start_addr.wrapping_add(tile_num * 16).wrapping_add(i));
            let b2 = gb.read_byte(start_addr.wrapping_add(tile_num * 16).wrapping_add(i + 1));

            for j in (0..=7).rev() {
                let hi = !!(b1 & (1 << j)) << 1;
                let lo = !!(b2 & (1 << j));

                let color = (hi | lo) as usize;
                let rc = Rect::new(x as i32 + ((7 - j) * SCALE) as i32, y as i32 + ((i as i32) / 2 * SCALE as i32), SCALE, SCALE);

                surface.fill_rect(rc, COLORS[color]).unwrap();
            }
        }
    }

    pub fn update_debug_canvas(&mut self, dbg_canvas: &mut Canvas<Window>, dbg_surface: &mut Surface, dbg_texture: &mut Texture, gb: &mut Gb) {
        let mut num_tile = 0;
        let mut x_draw = 0;
        let mut y_draw = 0;
        let (w, h) = dbg_canvas.window().size();
        let mut rect: Rect = Rect::new(0, 0, w, h);
        dbg_surface.fill_rect(rect, Color::RGB(0, 0, 0)).unwrap();

        let addr: u16 = 0x8000;
        for i in 0..24 {
            for j in 0..16 {
                self.display_tile(dbg_surface, addr, num_tile, x_draw + (j * SCALE), y_draw + (i * SCALE), gb);
                x_draw += 8 * SCALE;
                num_tile += 1;
            }

            y_draw += 8 * SCALE;
            x_draw = 0;
        }

        dbg_texture.update(None, dbg_surface.without_lock().unwrap(), dbg_surface.pitch() as usize).unwrap();
        dbg_canvas.clear();
        dbg_canvas.copy(&dbg_texture, None, None).unwrap();
    }
}

impl Renderer for SdlRenderer {
    fn run(&mut self, gb: &mut Gb) {
        let (x, y) = self.canvas.window().position();
        let debug_window = self
            .video_system
            .window("SturdyGB Debug Window", WIDTH, HEIGHT)
            .position(x + WIDTH as i32, y)
            .build()
            .unwrap();
        let mut debug_canvas: Canvas<Window> = debug_window.into_canvas().build().unwrap();
        let binding = debug_canvas.texture_creator();
        let mut debug_texture: Texture = binding
            .create_texture(
                PixelFormatEnum::ARGB8888,
                TextureAccess::Streaming,
                (16 * 8 * SCALE) + (16 * SCALE),
                (32 * 8 * SCALE) + (64 * SCALE),
            )
            .unwrap();
        let mut event_pump: EventPump = self.context.event_pump().unwrap();
        let mut debug_surface = debug_canvas.window().surface(&event_pump).unwrap().convert_format(PixelFormatEnum::ARGB8888).unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::ControllerDeviceAdded { .. } | Event::ControllerDeviceRemoved { .. } => {
                        self.check_controllers();
                    }
                    Event::KeyDown { keycode, .. } => {
                    }
                    _ => {}
                }
            }
            gb.run();
            self.update_debug_canvas(&mut debug_canvas, &mut debug_surface, &mut debug_texture, gb);
            debug_canvas.present();
            self.canvas.present();
        }
    }
}
