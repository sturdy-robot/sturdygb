use super::EmuApp;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

#[cfg(target_arch = "wasm32")]
use super::persistence;
#[cfg(target_arch = "wasm32")]
use super::state::{PendingRomLoad, StatusUpdate, WasmUiEvent};
#[cfg(target_arch = "wasm32")]
use rfd::AsyncFileDialog;

impl EmuApp {
    pub(super) fn show_top_menu(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                self.show_file_menu(ui, ctx, frame);
                self.show_emulation_menu(ui, ctx, frame);

                #[cfg(not(target_arch = "wasm32"))]
                self.show_view_menu(ui);

                self.show_debug_menu(ui);

                if ui.button("Options").clicked() {
                    self.ui.show_options = true;
                }

                self.show_help_menu(ui);
            });
        });
    }

    fn show_file_menu(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        #[cfg(target_arch = "wasm32")]
        let _ = ctx;

        ui.menu_button("File", |ui| {
            if ui.button("📁 Open ROM...").clicked() {
                self.open_rom_from_picker_with_storage(frame.storage());
                ui.close();
            }

            #[cfg(target_arch = "wasm32")]
            {
                let has_state = self.has_loaded_game();
                if ui
                    .add_enabled(has_state, egui::Button::new("📥 Import Save..."))
                    .clicked()
                {
                    self.import_save_from_picker();
                    ui.close();
                }
                if ui
                    .add_enabled(has_state, egui::Button::new("📤 Export Save..."))
                    .clicked()
                {
                    self.export_save_file();
                    ui.close();
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                if ui.button("📁 Open Directory...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.load_directory(path);
                    }
                    ui.close();
                }
                ui.checkbox(&mut self.catalog.recursive_search, "🔍 Recursive Search");
                if ui.button("❎ Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
    }

    fn show_emulation_menu(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        ui.menu_button("Emulation", |ui| {
            let has_state = self.has_loaded_game();
            if has_state && ui.button("🟥 Stop").clicked() {
                self.stop_emulation(ctx);
                ui.close();
            }
            if ui
                .add_enabled(
                    has_state,
                    egui::Button::new(self.debugger_pause_resume_label()),
                )
                .clicked()
            {
                self.toggle_debugger_pause();
                ui.close();
            }
            if Self::show_reset_game_button(ui, has_state) {
                self.reset_loaded_rom(frame.storage());
                ui.close();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_view_menu(&mut self, ui: &mut egui::Ui) {
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
    }

    fn show_debug_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Debug", |ui| {
            let has_state = self.has_loaded_game();
            if ui
                .add_enabled(has_state, egui::Button::new("Debugger"))
                .clicked()
            {
                self.open_debugger();
                ui.close();
            }
            if ui
                .add_enabled(has_state, egui::Button::new("Step"))
                .clicked()
            {
                self.request_debugger_step(super::debugger::DebuggerStepKind::Over);
                ui.close();
            }
            if ui
                .add_enabled(has_state, egui::Button::new("VRAM Viewer"))
                .clicked()
            {
                self.open_debugger_tab(super::debugger::DebuggerTab::Vram);
                ui.close();
            }
            if ui
                .add_enabled(has_state, egui::Button::new("BG Map Viewer"))
                .clicked()
            {
                self.open_debugger_tab(super::debugger::DebuggerTab::BgMap);
                ui.close();
            }
            if ui
                .add_enabled(has_state, egui::Button::new("OAM Viewer"))
                .clicked()
            {
                self.open_debugger_tab(super::debugger::DebuggerTab::Oam);
                ui.close();
            }
        });
    }

    pub(super) fn stop_emulation(&mut self, ctx: &egui::Context) {
        #[cfg(target_arch = "wasm32")]
        let _ = ctx;

        #[cfg(target_arch = "wasm32")]
        if let Some(state) = self.runtime.loaded_game.as_ref() {
            super::persistence::persist_loaded_game(state);
        }

        self.runtime.loaded_game = None;
        self.runtime.texture = None;
        self.runtime.paused = false;
        self.debugger.reset_runtime();

        #[cfg(not(target_arch = "wasm32"))]
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(super::APP_NAME.to_string()));
    }

    pub(super) fn open_rom_from_picker_with_storage(
        &mut self,
        storage: Option<&dyn eframe::Storage>,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = FileDialog::new()
                .add_filter("GameBoy ROMs", &["gb", "gbc", "zip"])
                .pick_file()
            {
                self.load_rom_file(path.to_str().unwrap(), storage);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let _ = storage;
            let sender = self.runtime.async_event_channel.0.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let files = AsyncFileDialog::new()
                    .add_filter("GameBoy Files", &["gb", "gbc", "zip", "sav"])
                    .pick_files()
                    .await;

                if let Some(files) = files {
                    let event = WasmUiEvent::RomLoad(pending_rom_load_from_files(files).await);
                    let _ = sender.send(event);
                }
            });
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn import_save_from_picker(&mut self) {
        let sender = self.runtime.async_event_channel.0.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let file = AsyncFileDialog::new()
                .add_filter("GameBoy Save Files", &["sav"])
                .pick_file()
                .await;

            if let Some(file) = file {
                let bytes = file.read().await;
                let _ = sender.send(WasmUiEvent::SaveImport(Ok(bytes)));
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn export_save_file(&mut self) {
        match persistence::export_loaded_save(&self.runtime.loaded_game) {
            Ok(()) => self.set_status(StatusUpdate::success("Exported the current save file.")),
            Err(error) => self.set_status(StatusUpdate::error(error)),
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn pending_rom_load_from_files(
    files: Vec<rfd::FileHandle>,
) -> Result<PendingRomLoad, String> {
    let mut rom_file = None;
    let mut save_files = Vec::new();

    for file in files {
        let file_name = file.file_name();
        match lower_extension(&file_name).as_deref() {
            Some("gb") | Some("gbc") | Some("zip") if rom_file.is_none() => {
                rom_file = Some((file_name, file));
            }
            Some("sav") => {
                save_files.push((file_name, file));
            }
            _ => {}
        }
    }

    let Some((rom_name, rom_file)) = rom_file else {
        return Err("Select a Game Boy ROM file. You can also include a matching .sav file in the same selection.".to_string());
    };

    let rom_bytes = rom_file.read().await;
    let rom_stem = file_stem_lowercase(&rom_name);
    let had_save_selection = !save_files.is_empty();
    let matched_save = save_files
        .into_iter()
        .find(|(save_name, _)| file_stem_lowercase(save_name) == rom_stem);

    let (imported_save, status_update) = match matched_save {
        Some((_, save_file)) => (Some(save_file.read().await), None),
        None if had_save_selection => {
            return Err(
                "Selected .sav files must have the same basename as the selected ROM.".to_string(),
            );
        }
        None => {
            let load_outcome = persistence::load_battery_ram(&rom_bytes).await;
            (load_outcome.ram, load_outcome.status_update)
        }
    };

    Ok(PendingRomLoad {
        rom_bytes,
        imported_save,
        status_update,
    })
}

#[cfg(target_arch = "wasm32")]
fn lower_extension(file_name: &str) -> Option<String> {
    std::path::Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
}

#[cfg(target_arch = "wasm32")]
fn file_stem_lowercase(file_name: &str) -> String {
    std::path::Path::new(file_name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
}
