mod audio;
mod catalog;
mod config;
mod debugger;
mod options;
mod rom;

use eframe::egui;

use self::audio::AUDIO_PRODUCER;
#[cfg(not(target_arch = "wasm32"))]
use self::config::{GameEntry, SortMethod};
use self::config::{Palette, ScaleMode, SturdyConfig};
use self::debugger::DebuggerUiState;
use sturdygb_core::joypad::JoypadButton;

pub const APP_NAME: &str = concat!("SturdyGB v", env!("CARGO_PKG_VERSION"));

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

const GB_W: usize = 160;
const GB_H: usize = 144;

struct State {
    gb: sturdygb_core::gb::Gb,
    rgba: Vec<u8>,
    leftover_audio: Vec<[f32; 2]>,
    title: String,
    rom_bytes: Vec<u8>,
    save_path: Option<std::path::PathBuf>,
}

pub struct EmuApp {
    state: Option<State>,
    texture: Option<egui::TextureHandle>,
    error_msg: Option<String>,
    debugger: DebuggerUiState,
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
    paused: bool,
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
            debugger: DebuggerUiState::new(),
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
            paused: false,
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
            app.load_rom_file(&rom, cc.storage);
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            app.reload_all_directories();
        }

        app
    }
}

impl eframe::App for EmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "sturdygb_config", &self.config);

        #[cfg(target_arch = "wasm32")]
        if let Some(state) = &mut self.state {
            if let Some(ram) = state.gb.get_battery_ram() {
                eframe::set_value(
                    storage,
                    &format!("sturdygb_sram_{}", state.title),
                    &ram.to_vec(),
                );
            }
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.config.fullscreen));

        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
            self.config.fullscreen = !self.config.fullscreen;
        }

        if let Ok(result) = self.rom_load_channel.1.try_recv() {
            match result {
                Ok(bytes) => self.load_rom_bytes(bytes, None, _frame.storage()),
                Err(e) => self.error_msg = Some(format!("Failed to load ROM via async: {e}")),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if self.loading_directory {
            if let Some(rx) = &self.dir_load_receiver {
                let mut loaded_some = false;
                let disconnected = loop {
                    match rx.try_recv() {
                        Ok(entry) => {
                            self.game_list.push(entry);
                            loaded_some = true;
                        }
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => break true,
                        Err(std::sync::mpsc::TryRecvError::Empty) => break false,
                    }
                };

                if disconnected {
                    self.loading_directory = false;
                    self.dir_load_receiver = None;
                    self.game_list.sort_by(|a, b| a.filename.cmp(&b.filename));
                }

                if loaded_some || self.loading_directory {
                    ctx.request_repaint();
                }
            }
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📁 Open ROM...").clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            if let Some(path) = FileDialog::new()
                                .add_filter("GameBoy ROMs", &["gb", "gbc", "zip"])
                                .pick_file()
                            {
                                self.load_rom_file(path.to_str().unwrap(), _frame.storage());
                            }
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            let sender = self.rom_load_channel.0.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let file = AsyncFileDialog::new()
                                    .add_filter("GameBoy ROMs", &["gb", "gbc", "zip"])
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

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui.button("📁 Open Directory...").clicked() {
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.load_directory(path);
                            }
                            ui.close();
                        }
                        ui.checkbox(&mut self.recursive_search, "🔍 Recursive Search");
                        if ui.button("❎ Exit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });

                ui.menu_button("Emulation", |ui| {
                    let has_state = self.state.is_some();
                    if has_state && ui.button("🟥 Stop").clicked() {
                        self.state = None;
                        self.texture = None;
                        self.paused = false;
                        self.debugger.reset_runtime();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Title(APP_NAME.to_string()));
                        ui.close();
                    }
                    if ui
                        .add_enabled(
                            has_state,
                            egui::Button::new(if self.paused { "▶ Resume" } else { "⏸ Pause" }),
                        )
                        .clicked()
                    {
                        if self.paused {
                            self.debugger.prepare_resume(self.state.as_ref());
                        }
                        self.paused = !self.paused;
                        ui.close();
                    }
                    if ui
                        .add_enabled(has_state, egui::Button::new("🔄 Reset"))
                        .clicked()
                    {
                        if let Some(state) = &self.state {
                            let rom_bytes = state.rom_bytes.clone();
                            let save_path = state.save_path.clone();
                            self.load_rom_bytes(rom_bytes, save_path, _frame.storage());
                        }
                        ui.close();
                    }
                });

                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("View", |ui| {
                    if ui
                        .button(if self.config.fullscreen {
                            "⛶ Exit Fullscreen (F11)"
                        } else {
                            "⛶ Fullscreen (F11)"
                        })
                        .clicked()
                    {
                        self.config.fullscreen = !self.config.fullscreen;
                        ui.close();
                    }
                });

                ui.menu_button("Debug", |ui| {
                    let has_state = self.state.is_some();
                    if ui
                        .add_enabled(has_state, egui::Button::new("Debugger"))
                        .clicked()
                    {
                        self.debugger.show_debugger = true;
                        self.paused = true;
                        ui.close();
                    }
                    if ui
                        .add_enabled(has_state, egui::Button::new("Step"))
                        .clicked()
                    {
                        self.debugger.request_step(self.state.as_ref());
                        self.debugger.show_debugger = true;
                        self.paused = false;
                        ui.close();
                    }
                    if ui
                        .add_enabled(has_state, egui::Button::new("VRAM Viewer"))
                        .clicked()
                    {
                        self.debugger.show_vram_viewer = true;
                    }
                    if ui
                        .add_enabled(has_state, egui::Button::new("BG Map Viewer"))
                        .clicked()
                    {
                        self.debugger.show_bg_map_viewer = true;
                    }
                    if ui
                        .add_enabled(has_state, egui::Button::new("OAM Viewer"))
                        .clicked()
                    {
                        self.debugger.show_oam_viewer = true;
                    }
                });

                if ui.button("Options").clicked() {
                    self.show_options = true;
                }
            });
        });

        let mut error_cleared = false;
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

        self.show_options_window(ctx);
        self.show_debugger_windows(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(state) = &mut self.state {
                if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.state = None;
                    self.texture = None;
                    self.paused = false;
                    self.debugger.reset_runtime();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(APP_NAME.to_string()));
                    return;
                }

                if !self.paused {
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
                        set_btn(ctx, state, self.config.keybind(&btn), btn);
                    }

                    let mut channel_full = false;
                    let mut frames_run = 0;

                    let mut new_leftover = Vec::with_capacity(state.leftover_audio.len());
                    if let Ok(guard) = AUDIO_PRODUCER.lock() {
                        if let Some(prod) = guard.as_ref() {
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

                    while !channel_full && frames_run < 5 {
                        let hit_debug = self.debugger.run_until_debug_or_frame(state);
                        frames_run += 1;

                        let audio_data = state.gb.get_audio_buffer();
                        if let Ok(guard) = AUDIO_PRODUCER.lock() {
                            if let Some(prod) = guard.as_ref() {
                                for frame in audio_data.chunks_exact(2) {
                                    let sample = [frame[0], frame[1]];
                                    if !channel_full {
                                        if let Err(std::sync::mpsc::TrySendError::Full(val)) =
                                            prod.try_send(sample)
                                        {
                                            channel_full = true;
                                            if state.leftover_audio.len() < 8192 {
                                                state.leftover_audio.push(val);
                                            }
                                        }
                                    } else if state.leftover_audio.len() < 8192 {
                                        state.leftover_audio.push(sample);
                                    }
                                }
                            }
                        }

                        if hit_debug {
                            self.paused = true;
                            break;
                        }
                    }
                }

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
                                if ui.button("📁 Open ROM...").clicked() {
                                    if let Some(path) = FileDialog::new()
                                        .add_filter("GameBoy ROMs", &["gb", "gbc", "zip"])
                                        .pick_file()
                                    {
                                        self.load_rom_file(path.to_str().unwrap(), _frame.storage());
                                    }
                                }
                                if ui.button("📁 Add ROM directory...").clicked() {
                                    if let Some(path) = FileDialog::new().pick_folder() {
                                        self.load_directory(path);
                                    }
                                }
                            });
                        });
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Directories:");
                            let mut to_remove = None;
                            for (i, dir) in self.config.rom_directories.iter().enumerate() {
                                let dir_name = dir.file_name().unwrap_or_default().to_string_lossy();
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
                                    ui.heading(format!("Loading Games... ({})", self.game_list.len()));
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
                                .column(
                                    Column::auto_with_initial_suggestion(300.0)
                                        .clip(true)
                                        .resizable(true),
                                )
                                .column(
                                    Column::auto_with_initial_suggestion(150.0)
                                        .clip(true)
                                        .resizable(true),
                                )
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
                                self.load_rom_file(path.to_str().unwrap(), _frame.storage());
                            }
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                {
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(ui.available_height() / 2.0 - 30.0);
                            ui.heading(format!("{}", APP_NAME));
                            ui.heading("Select a ROM file");
                            ui.add_space(8.0);
                            if ui.button("📁 Open ROM...").clicked() {
                                let sender = self.rom_load_channel.0.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    let file = AsyncFileDialog::new()
                                        .add_filter("GameBoy ROMs", &["gb", "gbc", "zip"])
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
