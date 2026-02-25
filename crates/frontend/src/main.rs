use notan::draw::*;
use notan::math::{Mat3, Vec2};
use notan::prelude::*;

use clap::Parser;

use rfd::MessageDialog;
use std::sync::OnceLock;

use sturdygb_core::joypad::JoypadButton;
use sturdygb_core::prelude::GbInstance;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{sync_channel, SyncSender};

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
    leftover_audio: Vec<[f32; 2]>,
}

static mut AUDIO_PRODUCER: Option<SyncSender<[f32; 2]>> = None;
static mut AUDIO_STREAM: Option<cpal::Stream> = None;

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
    let mut gb = match GbInstance::build(rom) {
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

    setup_audio(&mut gb);

    State {
        gb,
        rgba: vec![0; GB_W * GB_H * 4],
        leftover_audio: Vec::new(),
    }
}

fn setup_audio(gb: &mut sturdygb_core::gb::Gb) {
    let host = cpal::default_host();
    let device = host.default_output_device();
    if let Some(device) = device {
        let config = device.default_output_config().unwrap().config();
        
        let sample_rate: u32 = config.sample_rate.into();
        gb.set_sample_rate(sample_rate);
        
        // Keep buffer balanced (4096 samples ~ 85ms at 48kHz) to prevent audio latency
        let (prod, cons) = sync_channel::<[f32; 2]>(4096);
        
        let channels = config.channels as usize;
        let mut last_sample = [0.0, 0.0];
        
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let sample = match cons.try_recv() {
                        Ok(v) => v,
                        Err(_) => [last_sample[0] * 0.90, last_sample[1] * 0.90],
                    };
                    last_sample = sample;
                    
                    if channels >= 1 && frame.len() >= 1 { frame[0] = sample[0]; }
                    if channels >= 2 && frame.len() >= 2 { frame[1] = sample[1]; }
                }
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            None
        );
        
        if let Ok(stream) = stream {
            stream.play().unwrap();
            unsafe {
                AUDIO_PRODUCER = Some(prod);
                AUDIO_STREAM = Some(stream);
            }
        }
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

    let mut channel_full = false;
    let mut frames_run = 0;

    // First try to drain leftover audio
    let mut new_leftover = Vec::with_capacity(state.leftover_audio.len());
    unsafe {
        #[allow(static_mut_refs)]
        if let Some(prod) = &mut AUDIO_PRODUCER {
            for sample in state.leftover_audio.drain(..) {
                if !channel_full {
                    if let Err(std::sync::mpsc::TrySendError::Full(val)) = prod.try_send(sample) {
                        channel_full = true;
                        new_leftover.push(val);
                    }
                } else {
                    new_leftover.push(sample);
                }
            }
        }
    }
    state.leftover_audio = new_leftover;

    // Run frames until audio channel fills up, capped to prevent infinite loops (if audio fails)
    while !channel_full && frames_run < 5 {
        state.gb.run_one_frame();
        frames_run += 1;
        
        let audio_data = state.gb.get_audio_buffer();
        unsafe {
            #[allow(static_mut_refs)]
            if let Some(prod) = &mut AUDIO_PRODUCER {
                for frame in audio_data.chunks_exact(2) {
                    let sample = [frame[0], frame[1]];
                    if !channel_full {
                        if let Err(std::sync::mpsc::TrySendError::Full(val)) = prod.try_send(sample) {
                            channel_full = true;
                            state.leftover_audio.push(val);
                        }
                    } else {
                        state.leftover_audio.push(sample);
                    }
                }
            }
        }
    }

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
