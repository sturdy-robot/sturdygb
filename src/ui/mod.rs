// SPDX-FileCopyrightText: 2023 Pedrenrique G. Guimarães
//
// SPDX-License-Identifier: MIT

use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::ContextBuilder;
use glium::Display;


pub fn initialize_ui() -> Display {
    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(1024.0, 768.0)).with_title("Hello World!");
    let cb = ContextBuilder::new();
    
    Display::new(wb, cb, &event_loop).unwrap()
}