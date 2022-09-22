#![allow(dead_code)]
#![allow(unused_variables)]

use glfw::{Action, Context, Key};
use crate::core::gb::GB;
use crate::core::cartridge::Cartridge;

mod core;

fn init_gb(cartridge: Cartridge) -> GB {
    let is_cgb = cartridge.is_cgb_only();
    GB::new(cartridge, is_cgb)
}

fn load_cartridge(filename: String) -> Cartridge {
    Cartridge::new(&filename)
}

fn init_glfw() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "SturdyGB", glfw::WindowMode::Windowed).expect("Unable to create GLFW window!");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => ()
    }
}

fn main() {
    let cartridge: Cartridge = load_cartridge("roms/dmg-acid2.gb".to_string());
    let mut gb: GB = init_gb(cartridge);
    gb.run();
}

