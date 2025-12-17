use notan::draw::*;
use notan::math::{Mat3, Vec2};
use notan::prelude::*;

use clap::Parser;

use rfd::MessageDialog;
use std::sync::OnceLock;

use sturdygb_core::joypad::JoypadButton;
use sturdygb_core::prelude::GbInstance;

const GB_W: usize = 160;
const GB_H: usize = 144;

const SCALE: f32 = 5.0;
const WIN_W: u32 = (GB_W as u32) * (SCALE as u32);
const WIN_H: u32 = (GB_H as u32) * (SCALE as u32);

static ROM_PATH: OnceLock<String> = OnceLock::new();
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn example_usage() -> &'static str {
    if cfg!(windows) {
        "sturdygb path\\to\\game.gb"
    } else {
        "sturdygb path/to/game.gb"
    }
}

#[derive(Parser, Debug)]
#[command(name = "sturdygb")]
struct Cli {
    #[arg(value_name = "ROM")]
    rom: Option<String>,
}

#[derive(AppState)]
struct State {
    gb: sturdygb_core::gb::Gb,
    rgba: Vec<u8>,
}

#[notan_main]
fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let Some(rom) = cli.rom else {
        let example = example_usage();
        MessageDialog::new()
            .set_title("SturdyGB")
            .set_description(&format!(
                "No ROM was provided.\n\nRun this program from the command line and pass a ROM path.\n\nExample:\n  {example}"
            ))
            .show();
        return Ok(());
    };

    let _ = ROM_PATH.set(rom);

    let window = WindowConfig::new()
        .set_title(&format!("SturdyGB v{}", VERSION))
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
    let rom = ROM_PATH.get().expect("ROM path must be set");
    let gb = match GbInstance::build(rom) {
        Ok(gb) => gb,
        Err(e) => {
            let example = example_usage();
            let msg = format!(
                "Failed to load ROM:\n  {rom}\n\n{e}\n\nRun this program from the command line and pass a valid ROM path.\n\nExample:\n  {example}"
            );
            MessageDialog::new()
                .set_title("SturdyGB")
                .set_description(&msg)
                .show();
            std::process::exit(1);
        }
    };

    State {
        gb,
        rgba: vec![0; GB_W * GB_H * 4],
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }

    set_btn(app, state, KeyCode::Z, JoypadButton::A);
    set_btn(app, state, KeyCode::X, JoypadButton::B);
    set_btn(app, state, KeyCode::Return, JoypadButton::Start);
    set_btn(app, state, KeyCode::RShift, JoypadButton::Select);

    set_btn(app, state, KeyCode::Up, JoypadButton::Up);
    set_btn(app, state, KeyCode::Down, JoypadButton::Down);
    set_btn(app, state, KeyCode::Left, JoypadButton::Left);
    set_btn(app, state, KeyCode::Right, JoypadButton::Right);

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
