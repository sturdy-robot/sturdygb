use super::audio::AUDIO_PRODUCER;
#[cfg(not(target_arch = "wasm32"))]
use super::config::SortMethod;
use super::state::LoadedGameState;
use super::{EmuApp, GB_H, GB_W};
use eframe::egui;
use sturdygb_core::gb::ScreenData;
use sturdygb_core::joypad::JoypadButton;

impl EmuApp {
    pub(super) fn show_main_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.runtime.loaded_game.is_some() {
                self.show_running_game_panel(ui, ctx, frame);
            } else {
                self.show_game_library_panel(ui, frame);
            }
        });
    }

    fn show_running_game_panel(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        let mut reset_requested = false;
        let keybinds = [
            JoypadButton::Up,
            JoypadButton::Down,
            JoypadButton::Left,
            JoypadButton::Right,
            JoypadButton::A,
            JoypadButton::B,
            JoypadButton::Start,
            JoypadButton::Select,
        ]
        .map(|button| (button, self.config.keybind(&button)));
        let palette = self.config.palette;

        if let Some(state) = &mut self.runtime.loaded_game {
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.stop_emulation(ctx);
                return;
            }

            if !self.runtime.paused {
                handle_emulator_input(ctx, state, &keybinds);
                if run_emulator_frame(&mut self.debugger, state) {
                    self.runtime.paused = true;
                }
            }

            update_screen_rgba(palette, state);

            let image = egui::ColorImage::from_rgba_unmultiplied([GB_W, GB_H], &state.rgba);
            let texture = self.runtime.texture.get_or_insert_with(|| {
                ctx.load_texture("gb_screen", image.clone(), egui::TextureOptions::NEAREST)
            });
            texture.set(image, egui::TextureOptions::NEAREST);
            self.runtime.frames_rendered += 1;

            if self.runtime.last_fps_update.elapsed().as_secs_f32() >= 1.0 {
                self.runtime.current_fps = self.runtime.frames_rendered;
                self.runtime.frames_rendered = 0;
                self.runtime.last_fps_update = instant::Instant::now();

                ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                    "{} - {} (FPS: {})",
                    super::APP_NAME,
                    state.title,
                    self.runtime.current_fps
                )));
            }

            ui.horizontal_centered(|ui| {
                reset_requested = Self::show_reset_game_button(ui, true);
            });
            ui.add_space(8.0);

            let available_size = ui.available_size();
            let (width, height) = match self.config.scale {
                super::config::ScaleMode::Integer(scale) => {
                    ((GB_W as f32) * scale, (GB_H as f32) * scale)
                }
                super::config::ScaleMode::Stretch => {
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
        }

        if reset_requested {
            self.reset_loaded_rom(frame.storage());
        }
    }

    fn show_game_library_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        self.show_native_library_panel(ui, frame);

        #[cfg(target_arch = "wasm32")]
        self.show_wasm_picker_panel(ui, frame);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_native_library_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        if self.config.rom_directories.is_empty()
            && self.catalog.game_list.is_empty()
            && !self.catalog.loading_directory
        {
            self.show_empty_library_panel(ui, frame);
            return;
        }

        self.show_directory_chips(ui);
        ui.separator();

        if self.catalog.loading_directory {
            self.show_loading_library_panel(ui);
            return;
        }

        self.show_game_table(ui, frame);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_empty_library_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.0 - 30.0);
                ui.heading("No games found.");
                ui.add_space(8.0);
                if ui.button("📁 Open ROM...").clicked() {
                    self.open_rom_from_picker_with_storage(frame.storage());
                }
                if ui.button("📁 Add ROM directory...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.load_directory(path);
                    }
                }
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_directory_chips(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("Directories:");
            let mut to_remove = None;
            for (index, dir) in self.config.rom_directories.iter().enumerate() {
                let dir_name = dir.file_name().unwrap_or_default().to_string_lossy();
                let response = ui.button(format!("{} ❌", dir_name));
                if response.clicked() {
                    to_remove = Some(index);
                }
            }
            if let Some(index) = to_remove {
                self.config.rom_directories.remove(index);
                self.reload_all_directories();
            }
            if ui.button("+ Add").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.load_directory(path);
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_loading_library_panel(&self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.add_space(ui.available_height() / 2.0 - 30.0);
            ui.vertical_centered(|ui| {
                ui.heading(format!(
                    "Loading Games... ({})",
                    self.catalog.game_list.len()
                ));
                ui.add(egui::Spinner::new().size(32.0));
            });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_game_table(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let mut to_load = None;

        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.catalog.search_query);

            ui.separator();

            ui.label("Sort by:");
            egui::ComboBox::from_id_salt("sort_by")
                .selected_text(format!("{:?}", self.catalog.sort_by))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.catalog.sort_by,
                        SortMethod::Filename,
                        "Filename",
                    );
                    ui.selectable_value(&mut self.catalog.sort_by, SortMethod::Title, "Title");
                    ui.selectable_value(&mut self.catalog.sort_by, SortMethod::Company, "Company");
                });

            if ui
                .button(if self.catalog.sort_ascending {
                    "⬆"
                } else {
                    "⬇"
                })
                .clicked()
            {
                self.catalog.sort_ascending = !self.catalog.sort_ascending;
            }
        });
        ui.add_space(4.0);

        let query = self.catalog.search_query.to_lowercase();
        let mut filtered_games: Vec<_> = self
            .catalog
            .game_list
            .iter()
            .filter(|game| {
                query.is_empty()
                    || game.filename.to_lowercase().contains(&query)
                    || game.title.to_lowercase().contains(&query)
                    || game.company.to_lowercase().contains(&query)
            })
            .collect();

        filtered_games.sort_by(|left, right| {
            let cmp = match self.catalog.sort_by {
                SortMethod::Filename => left.filename.cmp(&right.filename),
                SortMethod::Title => left.title.cmp(&right.title),
                SortMethod::Company => left.company.cmp(&right.company),
            };
            if self.catalog.sort_ascending {
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
                        if ui.selectable_label(false, &entry.filename).double_clicked() {
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
            self.load_rom_file(path.to_str().unwrap(), frame.storage());
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn show_wasm_picker_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.0 - 30.0);
                ui.heading(super::APP_NAME);
                ui.heading("Select a ROM file");
                ui.add_space(8.0);
                if ui.button("📁 Open ROM...").clicked() {
                    self.open_rom_from_picker_with_storage(frame.storage());
                }
            });
        });
    }
}

fn handle_emulator_input(
    ctx: &egui::Context,
    state: &mut LoadedGameState,
    keybinds: &[(JoypadButton, egui::Key); 8],
) {
    for &(button, key) in keybinds {
        set_btn(ctx, state, key, button);
    }
}

fn run_emulator_frame(
    debugger: &mut super::debugger::DebuggerUiState,
    state: &mut LoadedGameState,
) -> bool {
    let mut channel_full = false;
    let mut frames_run = 0;

    let mut new_leftover = Vec::with_capacity(state.leftover_audio.len());
    if let Ok(guard) = AUDIO_PRODUCER.lock() {
        if let Some(prod) = guard.as_ref() {
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

    while !channel_full && frames_run < 5 {
        let hit_debug = debugger.run_until_debug_or_frame(state);
        frames_run += 1;

        let audio_data = state.gb.get_audio_buffer();
        if let Ok(guard) = AUDIO_PRODUCER.lock() {
            if let Some(prod) = guard.as_ref() {
                for frame in audio_data.chunks_exact(2) {
                    let sample = [frame[0], frame[1]];
                    if !channel_full {
                        if let Err(std::sync::mpsc::TrySendError::Full(val)) = prod.try_send(sample)
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
            return true;
        }
    }

    false
}

fn update_screen_rgba(palette: super::config::Palette, state: &mut LoadedGameState) {
    match state.gb.get_screen_data() {
        ScreenData::Dmg(frame_data) => {
            let palette_colors = match palette {
                super::config::Palette::Greyscale => {
                    [(255, 255, 255), (192, 192, 192), (96, 96, 96), (0, 0, 0)]
                }
                super::config::Palette::ClassicGreen => {
                    [(224, 248, 208), (136, 192, 112), (52, 104, 86), (8, 24, 32)]
                }
                super::config::Palette::Pocket => {
                    [(232, 232, 232), (160, 160, 160), (88, 88, 88), (16, 16, 16)]
                }
            };

            for y in 0..GB_H {
                for x in 0..GB_W {
                    let shade = frame_data[y][x] as usize;
                    let (r, g, b) = palette_colors[shade];
                    let i = (y * GB_W + x) * 4;
                    state.rgba[i] = r;
                    state.rgba[i + 1] = g;
                    state.rgba[i + 2] = b;
                    state.rgba[i + 3] = 255;
                }
            }
        }
        ScreenData::Cgb(frame_data) => {
            for y in 0..GB_H {
                for x in 0..GB_W {
                    let [r, g, b] = frame_data[y][x];
                    let i = (y * GB_W + x) * 4;
                    state.rgba[i] = r;
                    state.rgba[i + 1] = g;
                    state.rgba[i + 2] = b;
                    state.rgba[i + 3] = 255;
                }
            }
        }
    }
}

fn set_btn(ctx: &egui::Context, state: &mut LoadedGameState, key: egui::Key, btn: JoypadButton) {
    if ctx.input(|i| i.key_down(key)) {
        state.gb.press_button(btn);
    } else {
        state.gb.release_button(btn);
    }
}
