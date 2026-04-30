use super::EmuApp;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

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
        ui.menu_button("File", |ui| {
            if ui.button("📁 Open ROM...").clicked() {
                self.open_rom_from_picker_with_storage(frame.storage());
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
            let has_state = self.runtime.loaded_game.is_some();
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
            let has_state = self.runtime.loaded_game.is_some();
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
        self.runtime.loaded_game = None;
        self.runtime.texture = None;
        self.runtime.paused = false;
        self.debugger.reset_runtime();
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
            let sender = self.runtime.rom_load_channel.0.clone();
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
    }
}
