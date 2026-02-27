use eframe::egui;

use sturdygb_core::joypad::JoypadButton;
use sturdygb_core::prelude::GbInstance;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, SyncSender};

pub const APP_NAME: &str = concat!("SturdyGB v", env!("CARGO_PKG_VERSION"));

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

const GB_W: usize = 160;
const GB_H: usize = 144;

static mut AUDIO_PRODUCER: Option<SyncSender<[f32; 2]>> = None;
static mut AUDIO_STREAM: Option<cpal::Stream> = None;

struct State {
    gb: sturdygb_core::gb::Gb,
    rgba: Vec<u8>,
    leftover_audio: Vec<[f32; 2]>,
    title: String,
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
    game_list: Vec<GameEntry>,
    #[cfg(not(target_arch = "wasm32"))]
    recursive_search: bool,
    #[cfg(not(target_arch = "wasm32"))]
    search_query: String,
    #[cfg(not(target_arch = "wasm32"))]
    sort_by: SortMethod,
    #[cfg(not(target_arch = "wasm32"))]
    sort_ascending: bool,
    config: SturdyConfig,
    show_options: bool,
    #[cfg(not(target_arch = "wasm32"))]
    loading_directory: bool,
    #[cfg(not(target_arch = "wasm32"))]
    dir_load_receiver: Option<std::sync::mpsc::Receiver<GameEntry>>,
    start_time: instant::Instant,
    frames_rendered: usize,
    last_fps_update: instant::Instant,
    current_fps: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // automatically implement Default fallback
pub struct SturdyConfig {
    pub scale: ScaleMode,
    pub palette: Palette,
    #[cfg(not(target_arch = "wasm32"))]
    pub rom_directories: Vec<std::path::PathBuf>,
    pub keybinds: HashMap<JoypadButton, egui::Key>,
}

impl Default for SturdyConfig {
    fn default() -> Self {
        let mut keybinds = HashMap::new();
        keybinds.insert(JoypadButton::Up, egui::Key::ArrowUp);
        keybinds.insert(JoypadButton::Down, egui::Key::ArrowDown);
        keybinds.insert(JoypadButton::Left, egui::Key::ArrowLeft);
        keybinds.insert(JoypadButton::Right, egui::Key::ArrowRight);
        keybinds.insert(JoypadButton::A, egui::Key::Z);
        keybinds.insert(JoypadButton::B, egui::Key::X);
        keybinds.insert(JoypadButton::Start, egui::Key::Enter);
        keybinds.insert(JoypadButton::Select, egui::Key::Space);

        Self {
            #[cfg(not(target_arch = "wasm32"))]
            rom_directories: Vec::new(),
            scale: ScaleMode::Integer(4.0),
            palette: Palette::Greyscale,
            keybinds,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, Debug)]
pub enum ScaleMode {
    Integer(f32),
    Stretch,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, Debug)]
pub enum Palette {
    Greyscale,
    ClassicGreen,
    Pocket,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SortMethod {
    Filename,
    Title,
    Company,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
struct GameEntry {
    path: std::path::PathBuf,
    filename: String,
    title: String,
    company: String,
}

impl EmuApp {
    pub fn new(cc: &eframe::CreationContext<'_>, initial_rom: Option<String>) -> Self {
        let mut config: SturdyConfig = Default::default();
        if let Some(storage) = cc.storage {
            if let Some(saved) = eframe::get_value::<SturdyConfig>(storage, "sturdygb_config") {
                config = saved;
            }
        }

        let mut app = Self {
            state: None,
            texture: None,
            error_msg: None,
            rom_load_channel: std::sync::mpsc::channel(),
            #[cfg(not(target_arch = "wasm32"))]
            game_list: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            recursive_search: false,
            #[cfg(not(target_arch = "wasm32"))]
            search_query: String::new(),
            #[cfg(not(target_arch = "wasm32"))]
            sort_by: SortMethod::Filename,
            #[cfg(not(target_arch = "wasm32"))]
            sort_ascending: true,
            config,
            show_options: false,
            #[cfg(not(target_arch = "wasm32"))]
            loading_directory: false,
            #[cfg(not(target_arch = "wasm32"))]
            dir_load_receiver: None,
            start_time: instant::Instant::now(),
            frames_rendered: 0,
            last_fps_update: instant::Instant::now(),
            current_fps: 0,
        };

        if let Some(rom) = initial_rom {
            app.load_rom_file(&rom);
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            app.reload_all_directories();
        }

        app
    }

    fn load_rom_file(&mut self, path: &str) {
        if let Ok(bytes) = std::fs::read(path) {
            if let Some(extracted) = extract_rom_from_bytes(&bytes) {
                self.load_rom_bytes(
                    extracted,
                    Some(std::path::PathBuf::from(path).with_extension("sav")),
                );
                return;
            }

            let mut title = "Unknown Title".to_string();
            if let Ok(header) = sturdygb_core::cartridge::CartridgeHeader::new(&bytes) {
                title = header.title;
            }

            let save_path = std::path::PathBuf::from(path).with_extension("sav");
            match GbInstance::build_from_bytes(bytes, Some(save_path)) {
                Ok(mut gb) => {
                    setup_audio(&mut gb);
                    self.state = Some(State {
                        gb,
                        rgba: vec![0; GB_W * GB_H * 4],
                        leftover_audio: Vec::new(),
                        title,
                    });
                    self.error_msg = None;
                    self.frames_rendered = 0;
                    self.last_fps_update = instant::Instant::now();
                }
                Err(e) => {
                    self.error_msg = Some(format!("Failed to load ROM:\n{e}"));
                }
            }
        } else {
            self.error_msg = Some(format!("Could not read file {path}"));
        }
    }

    fn load_rom_bytes(&mut self, mut bytes: Vec<u8>, save_path: Option<std::path::PathBuf>) {
        if let Some(extracted) = extract_rom_from_bytes(&bytes) {
            bytes = extracted;
        }

        let mut title = "Unknown Title".to_string();
        if let Ok(header) = sturdygb_core::cartridge::CartridgeHeader::new(&bytes) {
            title = header.title;
        }

        match GbInstance::build_from_bytes(bytes, save_path) {
            Ok(mut gb) => {
                setup_audio(&mut gb);
                self.state = Some(State {
                    gb,
                    rgba: vec![0; GB_W * GB_H * 4],
                    leftover_audio: Vec::new(),
                    title,
                });
                self.error_msg = None;
                self.frames_rendered = 0;
                self.last_fps_update = instant::Instant::now();
            }
            Err(e) => {
                self.error_msg = Some(format!("Failed to load ROM:\n{e}"));
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_directory(&mut self, path: std::path::PathBuf) {
        if !self.config.rom_directories.contains(&path) {
            self.config.rom_directories.push(path);
        }
        self.reload_all_directories();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn reload_all_directories(&mut self) {
        self.game_list.clear();
        if self.config.rom_directories.is_empty() {
            return;
        }
        self.loading_directory = true;

        let (tx, rx) = std::sync::mpsc::channel();
        self.dir_load_receiver = Some(rx);
        let recursive = self.recursive_search;
        let dirs = self.config.rom_directories.clone();

        std::thread::spawn(move || {
            for path in dirs {
                let walker = walkdir::WalkDir::new(path);
                let walker = if recursive {
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
                                let filename = path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                let mut title = "Unknown Title".to_string();
                                let mut company = "Unknown Company".to_string();

                                if let Ok(bytes) = std::fs::read(&path) {
                                    let try_bytes = extract_rom_from_bytes(&bytes).unwrap_or(bytes);
                                    if let Ok(header) =
                                        sturdygb_core::cartridge::CartridgeHeader::new(&try_bytes)
                                    {
                                        title = header.title;
                                        company = header.company;
                                    }
                                }

                                if tx
                                    .send(GameEntry {
                                        path: path.to_path_buf(),
                                        filename,
                                        title,
                                        company,
                                    })
                                    .is_err()
                                {
                                    return; // receiver dropped
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

impl eframe::App for EmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "sturdygb_config", &self.config);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for async loaded roms
        if let Ok(result) = self.rom_load_channel.1.try_recv() {
            match result {
                Ok(bytes) => self.load_rom_bytes(bytes, None),
                Err(e) => self.error_msg = Some(format!("Failed to load ROM via async: {e}")),
            }
        }

        // Handle asynchronous directory loading updates
        #[cfg(not(target_arch = "wasm32"))]
        if self.loading_directory {
            if let Some(rx) = &self.dir_load_receiver {
                let mut loaded_some = false;
                while let Ok(entry) = rx.try_recv() {
                    self.game_list.push(entry);
                    loaded_some = true;
                }

                // If the channel is disconnected, the loading thread is done
                if let Err(std::sync::mpsc::TryRecvError::Disconnected) = rx.try_recv() {
                    self.loading_directory = false;
                    self.dir_load_receiver = None;
                    self.game_list.sort_by(|a, b| a.filename.cmp(&b.filename));
                }

                // Request a repaint so we show progress
                if loaded_some || self.loading_directory {
                    ctx.request_repaint();
                }
            }
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
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
                    if let Some(_) = self.state {
                        if ui.button("Stop").clicked() {
                            self.state = None;
                            self.texture = None;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Title(
                                APP_NAME.to_string(),
                            ));
                            ui.close();
                        }
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
                if ui.button("Options").clicked() {
                    self.show_options = true;
                }
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

        let mut is_open = self.show_options;
        if is_open {
            egui::Window::new("Emulator Options")
                .collapsible(false)
                .resizable(false)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    egui::Grid::new("options_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Scale Mode:");
                            egui::ComboBox::from_id_salt("scale_combo")
                                .selected_text(format!("{:?}", self.config.scale))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        ScaleMode::Integer(1.0),
                                        "1x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        ScaleMode::Integer(2.0),
                                        "2x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        ScaleMode::Integer(3.0),
                                        "3x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        ScaleMode::Integer(4.0),
                                        "4x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        ScaleMode::Integer(5.0),
                                        "5x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        ScaleMode::Integer(6.0),
                                        "6x",
                                    );
                                    ui.separator();
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        ScaleMode::Stretch,
                                        "Stretch (Fit window)",
                                    );
                                });
                            ui.end_row();

                            ui.label("Color Palette:");
                            egui::ComboBox::from_id_salt("palette_combo")
                                .selected_text(format!("{:?}", self.config.palette))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.config.palette,
                                        Palette::Greyscale,
                                        "Greyscale",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.palette,
                                        Palette::ClassicGreen,
                                        "Classic Green",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.palette,
                                        Palette::Pocket,
                                        "Pocket (Grey/Green)",
                                    );
                                });
                            ui.end_row();
                        });

                    ui.separator();
                    ui.label("Keybindings:");

                    egui::Grid::new("keybinds_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .show(ui, |ui| {
                            let buttons = [
                                JoypadButton::Up,
                                JoypadButton::Down,
                                JoypadButton::Left,
                                JoypadButton::Right,
                                JoypadButton::A,
                                JoypadButton::B,
                                JoypadButton::Start,
                                JoypadButton::Select,
                            ];

                            for btn in buttons {
                                ui.label(format!("{:?}", btn));

                                let current_key = self
                                    .config
                                    .keybinds
                                    .get(&btn)
                                    .copied()
                                    .unwrap_or(egui::Key::Escape);

                                let btn_text = if ctx.memory(|mem| {
                                    mem.data
                                        .get_temp::<JoypadButton>(egui::Id::new("listening_bind"))
                                }) == Some(btn)
                                {
                                    "Press any key...".to_string()
                                } else {
                                    format!("{:?}", current_key)
                                };

                                let response = ui.button(btn_text);

                                if response.clicked() {
                                    ctx.memory_mut(|mem| {
                                        mem.data.insert_temp(egui::Id::new("listening_bind"), btn)
                                    });
                                }

                                ui.end_row();
                            }
                        });

                    if let Some(btn) = ctx.memory(|mem| {
                        mem.data
                            .get_temp::<JoypadButton>(egui::Id::new("listening_bind"))
                    }) {
                        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                            ctx.memory_mut(|mem| {
                                mem.data
                                    .remove::<JoypadButton>(egui::Id::new("listening_bind"))
                            });
                        } else if let Some(key) = ctx.input(|i| {
                            i.events.iter().find_map(|e| {
                                if let egui::Event::Key {
                                    key, pressed: true, ..
                                } = e
                                {
                                    Some(*key)
                                } else {
                                    None
                                }
                            })
                        }) {
                            self.config.keybinds.insert(btn, key);
                            ctx.memory_mut(|mem| {
                                mem.data
                                    .remove::<JoypadButton>(egui::Id::new("listening_bind"))
                            });
                        }
                    }
                });
        }
        self.show_options = is_open;

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(state) = &mut self.state {
                if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.state = None;
                    self.texture = None;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(APP_NAME.to_string()));
                    return;
                }

                // Input handling
                let k = &self.config.keybinds;
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::Up).unwrap(),
                    JoypadButton::Up,
                );
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::Down).unwrap(),
                    JoypadButton::Down,
                );
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::Left).unwrap(),
                    JoypadButton::Left,
                );
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::Right).unwrap(),
                    JoypadButton::Right,
                );
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::A).unwrap(),
                    JoypadButton::A,
                );
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::B).unwrap(),
                    JoypadButton::B,
                );
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::Start).unwrap(),
                    JoypadButton::Start,
                );
                set_btn(
                    ctx,
                    state,
                    *k.get(&JoypadButton::Select).unwrap(),
                    JoypadButton::Select,
                );

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

                let palette_colors = match self.config.palette {
                    Palette::Greyscale => {
                        [(255, 255, 255), (192, 192, 192), (96, 96, 96), (0, 0, 0)]
                    }
                    Palette::ClassicGreen => {
                        [(224, 248, 208), (136, 192, 112), (52, 104, 86), (8, 24, 32)]
                    }
                    Palette::Pocket => {
                        [(232, 232, 232), (160, 160, 160), (88, 88, 88), (16, 16, 16)]
                    }
                };

                for y in 0..GB_H {
                    for x in 0..GB_W {
                        let shade = frame_data[y][x] as usize;
                        let (r, g, b) = palette_colors[shade];
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
                self.frames_rendered += 1;

                if self.last_fps_update.elapsed().as_secs_f32() >= 1.0 {
                    self.current_fps = self.frames_rendered;
                    self.frames_rendered = 0;
                    self.last_fps_update = instant::Instant::now();

                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                        "{} - {} (FPS: {})",
                        APP_NAME, state.title, self.current_fps
                    )));
                }

                // Show the texture centered and scaled
                let available_size = ui.available_size();

                let (width, height) = match self.config.scale {
                    ScaleMode::Integer(s) => ((GB_W as f32) * s, (GB_H as f32) * s),
                    ScaleMode::Stretch => {
                        let w_ratio = available_size.x / (GB_W as f32);
                        let h_ratio = available_size.y / (GB_H as f32);
                        let min_ratio = w_ratio.min(h_ratio);
                        ((GB_W as f32) * min_ratio, (GB_H as f32) * min_ratio)
                    }
                };

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

                // Request repaint if we are running the emulator
                ctx.request_repaint();
            } else {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if self.config.rom_directories.is_empty()
                        && self.game_list.is_empty()
                        && !self.loading_directory
                    {
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(ui.available_height() / 2.0 - 30.0);
                                ui.heading("No games found.");
                                ui.add_space(8.0);
                                if ui.button("Open ROM...").clicked() {
                                    if let Some(path) = FileDialog::new()
                                        .add_filter("GameBoy ROMs", &["gb"])
                                        .pick_file()
                                    {
                                        self.load_rom_file(path.to_str().unwrap());
                                    }
                                }
                                if ui.button("Add ROM directory...").clicked() {
                                    if let Some(path) = FileDialog::new().pick_folder() {
                                        self.load_directory(path);
                                    }
                                }
                            });
                        });
                    } else {
                        // Show directory chips
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Directories:");
                            let mut to_remove = None;
                            for (i, dir) in self.config.rom_directories.iter().enumerate() {
                                let dir_name =
                                    dir.file_name().unwrap_or_default().to_string_lossy();
                                let response = ui.button(format!("{} ❌", dir_name));
                                if response.clicked() {
                                    to_remove = Some(i);
                                }
                            }
                            if let Some(i) = to_remove {
                                self.config.rom_directories.remove(i);
                                self.reload_all_directories();
                            }
                            if ui.button("+ Add").clicked() {
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    self.load_directory(path);
                                }
                            }
                        });
                        ui.separator();

                        if self.loading_directory {
                            ui.centered_and_justified(|ui| {
                                ui.add_space(ui.available_height() / 2.0 - 30.0);
                                ui.vertical_centered(|ui| {
                                    ui.heading(format!(
                                        "Loading Games... ({})",
                                        self.game_list.len()
                                    ));
                                    ui.add(egui::Spinner::new().size(32.0));
                                });
                            });
                        } else {
                            let mut to_load = None;

                            ui.horizontal(|ui| {
                                ui.label("Search:");
                                ui.text_edit_singleline(&mut self.search_query);

                                ui.separator();

                                ui.label("Sort by:");
                                egui::ComboBox::from_id_salt("sort_by")
                                    .selected_text(format!("{:?}", self.sort_by))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.sort_by,
                                            SortMethod::Filename,
                                            "Filename",
                                        );
                                        ui.selectable_value(
                                            &mut self.sort_by,
                                            SortMethod::Title,
                                            "Title",
                                        );
                                        ui.selectable_value(
                                            &mut self.sort_by,
                                            SortMethod::Company,
                                            "Company",
                                        );
                                    });

                                if ui
                                    .button(if self.sort_ascending { "⬆" } else { "⬇" })
                                    .clicked()
                                {
                                    self.sort_ascending = !self.sort_ascending;
                                }
                            });
                            ui.add_space(4.0);

                            let query = self.search_query.to_lowercase();
                            let mut filtered_games: Vec<_> = self
                                .game_list
                                .iter()
                                .filter(|g| {
                                    query.is_empty()
                                        || g.filename.to_lowercase().contains(&query)
                                        || g.title.to_lowercase().contains(&query)
                                        || g.company.to_lowercase().contains(&query)
                                })
                                .collect();

                            filtered_games.sort_by(|a, b| {
                                let cmp = match self.sort_by {
                                    SortMethod::Filename => a.filename.cmp(&b.filename),
                                    SortMethod::Title => a.title.cmp(&b.title),
                                    SortMethod::Company => a.company.cmp(&b.company),
                                };
                                if self.sort_ascending {
                                    cmp
                                } else {
                                    cmp.reverse()
                                }
                            });

                            let row_height = 20.0;

                            use egui_extras::{Column, TableBuilder};
                            let table = TableBuilder::new(ui)
                                .striped(true)
                                .resizable(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::initial(150.0).clip(true).resizable(true))
                                .column(Column::initial(150.0).clip(true).resizable(true))
                                .column(Column::remainder())
                                .min_scrolled_height(0.0);

                            table
                                .header(row_height, |mut header| {
                                    header.col(|ui| {
                                        ui.strong("Filename");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Title");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Company");
                                    });
                                })
                                .body(|body| {
                                    body.rows(row_height, filtered_games.len(), |mut row| {
                                        let entry = filtered_games[row.index()];
                                        row.col(|ui| {
                                            if ui
                                                .selectable_label(false, &entry.filename)
                                                .double_clicked()
                                            {
                                                to_load = Some(entry.path.clone());
                                            }
                                        });
                                        row.col(|ui| {
                                            ui.label(&entry.title);
                                        });
                                        row.col(|ui| {
                                            ui.label(&entry.company);
                                        });
                                    });
                                });

                            if let Some(path) = to_load {
                                self.load_rom_file(path.to_str().unwrap());
                            }
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                {
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() / 2.0 - 30.0);
                            ui.heading("SturdyGB Web");
                            ui.add_space(8.0);
                            if ui.button("Open ROM...").clicked() {
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
                        });
                    });
                }
            }
        });
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

        let sample_rate: u32 = config.sample_rate.into();
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
