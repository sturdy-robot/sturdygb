use notan::draw::*;
use notan::math::{Mat3, Vec2};
use notan::prelude::*;

use sturdygb_core::joypad::JoypadButton;
use sturdygb_core::prelude::GbInstance;

const GB_W: usize = 160;
const GB_H: usize = 144;

const SCALE: f32 = 5.0;
const WIN_W: u32 = (GB_W as u32) * (SCALE as u32);
const WIN_H: u32 = (GB_H as u32) * (SCALE as u32);

#[derive(AppState)]
struct State {
    gb: sturdygb_core::gb::Gb,
    rgba: Vec<u8>,
}

#[notan_main]
fn main() -> Result<(), String> {
    let window = WindowConfig::new()
        .set_title("SturdyGB (notan)")
        .set_size(WIN_W, WIN_H)
        .set_vsync(true);

    notan::init_with(init)
        .add_config(window)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(_gfx: &mut Graphics) -> State {
    let args = std::env::args().collect::<Vec<String>>();
    let rom = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("roms/cpu_instrs.gb");

    let gb = GbInstance::build(rom)
        .map_err(|_| "Failed to load ROM".to_string())
        .unwrap();

    State {
        gb,
        rgba: vec![0; GB_W * GB_H * 4],
    }
}

fn update(app: &mut App, state: &mut State) {
    set_btn(app, state, KeyCode::Z, JoypadButton::A);
    set_btn(app, state, KeyCode::X, JoypadButton::B);
    set_btn(app, state, KeyCode::Return, JoypadButton::Start);
    set_btn(app, state, KeyCode::RShift, JoypadButton::Select);

    set_btn(app, state, KeyCode::Up, JoypadButton::Up);
    set_btn(app, state, KeyCode::Down, JoypadButton::Down);
    set_btn(app, state, KeyCode::Left, JoypadButton::Left);
    set_btn(app, state, KeyCode::Right, JoypadButton::Right);

    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }

    state.gb.run_one_frame();
    let frame = state.gb.get_screen_data();
    for y in 0..GB_H {
        for x in 0..GB_W {
            let shade = frame[y][x];
            let (r, g, b) = match shade {
                0 => (255, 255, 255),
                1 => (192, 192, 192),
                2 => (96, 96, 96),
                _ => (0, 0, 0),
            };
            let i = (y * GB_W + x) * 4;
            state.rgba[i + 0] = r;
            state.rgba[i + 1] = g;
            state.rgba[i + 2] = b;
            state.rgba[i + 3] = 255;
        }
    }
}

fn set_btn(app: &App, state: &mut State, key: KeyCode, btn: JoypadButton) {
    if app.keyboard.is_down(key) {
        state.gb.press_button(btn);
    } else {
        state.gb.release_button(btn);
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let tex = gfx
        .create_texture()
        .from_bytes(&state.rgba, GB_W as u32, GB_H as u32)
        .build()
        .unwrap();

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.transform()
        .push(Mat3::from_scale(Vec2::new(SCALE, SCALE)));

    draw.image(&tex).position(0.0, 0.0);

    draw.transform().clear();
    gfx.render(&draw);
}
