use eframe::egui;

use sturdygb_core::joypad::JoypadButton;
use sturdygb_core::prelude::GbInstance;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{sync_channel, SyncSender};

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

const GB_W: usize = 160;
const GB_H: usize = 144;
const SCALE: f32 = 4.0;

static mut AUDIO_PRODUCER: Option<SyncSender<[f32; 2]>> = None;
static mut AUDIO_STREAM: Option<cpal::Stream> = None;

struct State {
    gb: sturdygb_core::gb::Gb,
    rgba: Vec<u8>,
    leftover_audio: Vec<[f32; 2]>,
}

pub struct EmuApp {
    state: Option<State>,
    texture: Option<egui::TextureHandle>,
    error_msg: Option<String>,
    rom_load_channel: (
        std::sync::mpsc::Sender<Result<Vec<u8>, String>>,
        std::sync::mpsc::Receiver<Result<Vec<u8>, String>>,
    ),
    #[cfg(not(target_arch = "wasm32"))]
    game_list: Vec<std::path::PathBuf>,
    #[cfg(not(target_arch = "wasm32"))]
    recursive_search: bool,
}

impl EmuApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, initial_rom: Option<String>) -> Self {
        let mut app = Self {
            state: None,
            texture: None,
            error_msg: None,
            rom_load_channel: std::sync::mpsc::channel(),
            #[cfg(not(target_arch = "wasm32"))]
            game_list: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            recursive_search: false,
        };

        if let Some(rom) = initial_rom {
            app.load_rom_file(&rom);
        }

        app
    }

    fn load_rom_file(&mut self, path: &str) {
        if let Ok(bytes) = std::fs::read(path) {
            if let Some(extracted) = extract_rom_from_bytes(&bytes) {
                self.load_rom_bytes(extracted);
                return;
            }
        }

        match GbInstance::build(path) {
            Ok(mut gb) => {
                setup_audio(&mut gb);
                self.state = Some(State {
                    gb,
                    rgba: vec![0; GB_W * GB_H * 4],
                    leftover_audio: Vec::new(),
                });
                self.error_msg = None;
            }
            Err(e) => {
                self.error_msg = Some(format!("Failed to load ROM:\n{e}"));
            }
        }
    }

    fn load_rom_bytes(&mut self, mut bytes: Vec<u8>) {
        if let Some(extracted) = extract_rom_from_bytes(&bytes) {
            bytes = extracted;
        }
        match GbInstance::build_from_bytes(bytes) {
            Ok(mut gb) => {
                setup_audio(&mut gb);
                self.state = Some(State {
                    gb,
                    rgba: vec![0; GB_W * GB_H * 4],
                    leftover_audio: Vec::new(),
                });
                self.error_msg = None;
            }
            Err(e) => {
                self.error_msg = Some(format!("Failed to load ROM:\n{e}"));
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_directory(&mut self, path: std::path::PathBuf) {
        self.game_list.clear();
        let walker = walkdir::WalkDir::new(path);
        let walker = if self.recursive_search {
            walker
        } else {
            walker.max_depth(1)
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext = ext.to_lowercase();
                    if ext == "gb" || ext == "gbc" || ext == "zip" {
                        self.game_list.push(path.to_path_buf());
                    }
                }
            }
        }
        self.game_list.sort();
    }
}

impl eframe::App for EmuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for async loaded roms
        if let Ok(result) = self.rom_load_channel.1.try_recv() {
            match result {
                Ok(bytes) => self.load_rom_bytes(bytes),
                Err(e) => self.error_msg = Some(format!("Failed to load ROM via async: {e}")),
            }
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open ROM...").clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            if let Some(path) = FileDialog::new()
                                .add_filter("GameBoy ROMs", &["gb"])
                                .pick_file()
                            {
                                self.load_rom_file(path.to_str().unwrap());
                            }
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            let sender = self.rom_load_channel.0.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let file = AsyncFileDialog::new()
                                    .add_filter("GameBoy ROMs", &["gb", "gbc"])
                                    .pick_file()
                                    .await;

                                if let Some(file) = file {
                                    let bytes = file.read().await;
                                    let _ = sender.send(Ok(bytes));
                                }
                            });
                        }
                        ui.close();
                    }
                    if ui.button("Stop").clicked() {
                        self.state = None;
                        self.texture = None;
                        ui.close();
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui.button("Open Directory...").clicked() {
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.load_directory(path);
                            }
                            ui.close();
                        }
                        ui.checkbox(&mut self.recursive_search, "Recursive Search");
                        if ui.button("Exit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
            });
        });

        let mut error_cleared = false;
        // Handle error display
        if let Some(err) = &self.error_msg {
            let mut open = true;
            egui::Window::new("Error")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(err);
                    if ui.button("OK").clicked() {
                        error_cleared = true;
                    }
                });
            if !open {
                error_cleared = true;
            }
        }

        if error_cleared {
            self.error_msg = None;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(state) = &mut self.state {
                // Input handling
                set_btn(ctx, state, egui::Key::Z, JoypadButton::A);
                set_btn(ctx, state, egui::Key::X, JoypadButton::B);
                set_btn(ctx, state, egui::Key::Enter, JoypadButton::Start);
                set_btn(ctx, state, egui::Key::Space, JoypadButton::Select);

                set_btn(ctx, state, egui::Key::ArrowUp, JoypadButton::Up);
                set_btn(ctx, state, egui::Key::ArrowDown, JoypadButton::Down);
                set_btn(ctx, state, egui::Key::ArrowLeft, JoypadButton::Left);
                set_btn(ctx, state, egui::Key::ArrowRight, JoypadButton::Right);

                // Emulation Loop
                let mut channel_full = false;
                let mut frames_run = 0;

                // First try to drain leftover audio
                let mut new_leftover = Vec::with_capacity(state.leftover_audio.len());
                unsafe {
                    #[allow(static_mut_refs)]
                    if let Some(prod) = &mut AUDIO_PRODUCER {
                        for sample in state.leftover_audio.drain(..) {
                            if !channel_full {
                                if let Err(std::sync::mpsc::TrySendError::Full(val)) =
                                    prod.try_send(sample)
                                {
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
                // For wasm we might need to be very defensive about infinite loops
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
                                    if let Err(std::sync::mpsc::TrySendError::Full(val)) =
                                        prod.try_send(sample)
                                    {
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

                // Render video
                let frame_data = state.gb.get_screen_data();
                for y in 0..GB_H {
                    for x in 0..GB_W {
                        let shade = frame_data[y][x];
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

                let image = egui::ColorImage::from_rgba_unmultiplied([GB_W, GB_H], &state.rgba);
                let texture = self.texture.get_or_insert_with(|| {
                    ctx.load_texture("gb_screen", image.clone(), egui::TextureOptions::NEAREST)
                });
                texture.set(image, egui::TextureOptions::NEAREST);

                // Show the texture centered and scaled
                let available_size = ui.available_size();
                let width = (GB_W as f32) * SCALE;
                let height = (GB_H as f32) * SCALE;
                let x_offset = (available_size.x - width) / 2.0;
                let y_offset = (available_size.y - height) / 2.0;

                let rect = egui::Rect::from_min_size(
                    ui.min_rect().min + egui::vec2(x_offset.max(0.0), y_offset.max(0.0)),
                    egui::vec2(width, height),
                );

                ui.put(
                    rect,
                    egui::Image::new(&*texture).fit_to_exact_size(egui::vec2(width, height)),
                );
            } else {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if self.game_list.is_empty() {
                        ui.centered_and_justified(|ui| {
                            ui.heading("No games found. Use File -> Open Directory...");
                        });
                    } else {
                        let mut to_load = None;
                        ui.heading("Games");
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for path in &self.game_list {
                                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                    if ui.selectable_label(false, file_name).double_clicked() {
                                        to_load = Some(path.clone());
                                    }
                                }
                            }
                        });

                        if let Some(path) = to_load {
                            self.load_rom_file(path.to_str().unwrap());
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                ui.centered_and_justified(|ui| {
                    ui.heading("No game loaded.");
                });
            }
        });

        // Request repaint to keep emulator running
        ctx.request_repaint();
    }
}

fn set_btn(ctx: &egui::Context, state: &mut State, key: egui::Key, btn: JoypadButton) {
    if ctx.input(|i| i.key_down(key)) {
        state.gb.press_button(btn);
    } else {
        state.gb.release_button(btn);
    }
}

fn setup_audio(gb: &mut sturdygb_core::gb::Gb) {
    let host = cpal::default_host();
    let device = host.default_output_device();
    if let Some(device) = device {
        let config = device.default_output_config().unwrap().config();

        let sample_rate: u32 = config.sample_rate.0;
        gb.set_sample_rate(sample_rate);

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

                    if channels >= 1 && frame.len() >= 1 {
                        frame[0] = sample[0];
                    }
                    if channels >= 2 && frame.len() >= 2 {
                        frame[1] = sample[1];
                    }
                }
            },
            |err| eprintln!("an error occurred on stream: {}", err),
            None,
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

fn extract_rom_from_bytes(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() >= 4 && bytes[0..4] == [0x50, 0x4b, 0x03, 0x04] {
        let cursor = std::io::Cursor::new(bytes);
        if let Ok(mut archive) = zip::ZipArchive::new(cursor) {
            for i in 0..archive.len() {
                if let Ok(mut file) = archive.by_index(i) {
                    let name = file.name().to_lowercase();
                    if name.ends_with(".gb") || name.ends_with(".gbc") {
                        use std::io::Read;
                        let mut extracted = Vec::new();
                        if file.read_to_end(&mut extracted).is_ok() {
                            return Some(extracted);
                        }
                    }
                }
            }
        }
    }
    None
}
