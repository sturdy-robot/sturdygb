#[cfg(target_arch = "wasm32")]
use super::state::{ActiveStatusMessage, StatusLevel, StatusUpdate, WasmUiEvent};
use super::EmuApp;
use eframe::egui;

impl EmuApp {
    pub(super) fn sync_viewport_state(&mut self, ctx: &egui::Context) {
        #[cfg(target_arch = "wasm32")]
        let _ = ctx;

        #[cfg(not(target_arch = "wasm32"))]
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.config.fullscreen));

        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input(|input| input.key_pressed(egui::Key::F11)) {
            self.config.fullscreen = !self.config.fullscreen;
        }
    }

    pub(super) fn process_background_tasks(
        &mut self,
        ctx: &egui::Context,
        storage: Option<&dyn eframe::Storage>,
    ) {
        #[cfg(target_arch = "wasm32")]
        let _ = ctx;

        #[cfg(not(target_arch = "wasm32"))]
        let _ = storage;

        #[cfg(target_arch = "wasm32")]
        while let Ok(event) = self.runtime.async_event_channel.1.try_recv() {
            match event {
                WasmUiEvent::RomLoad(result) => match result {
                    Ok(load) => {
                        let status_update = load.status_update;
                        self.load_rom_bytes(load.rom_bytes, None, load.imported_save, storage);
                        if self.runtime.loaded_game.is_some() {
                            if let Some(status) = status_update {
                                self.set_status(status);
                            }
                        }
                    }
                    Err(error) => {
                        self.runtime.error_msg =
                            Some(format!("Failed to load ROM via async: {error}"));
                    }
                },
                WasmUiEvent::SaveImport(result) => match result {
                    Ok(bytes) => match self.import_save_bytes(bytes) {
                        Ok(status) => self.set_status(status),
                        Err(error) => self.set_status(StatusUpdate::error(error)),
                    },
                    Err(error) => {
                        self.set_status(StatusUpdate::error(format!(
                            "Failed to load save via async: {error}"
                        )));
                    }
                },
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        self.process_directory_loader(ctx);
    }

    pub(super) fn show_overlay_windows(
        &mut self,
        ctx: &egui::Context,
        storage: Option<&dyn eframe::Storage>,
    ) {
        self.show_error_window(ctx);

        #[cfg(target_arch = "wasm32")]
        self.show_status_banner(ctx);

        if self.show_options_window(ctx) {
            self.reset_loaded_rom(storage);
        }

        self.show_help_windows(ctx);
        self.show_debugger_windows(ctx, storage);
    }

    fn show_error_window(&mut self, ctx: &egui::Context) {
        let mut error_cleared = false;
        if let Some(error) = &self.runtime.error_msg {
            let mut open = true;
            egui::Window::new("Error")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error);
                    if ui.button("OK").clicked() {
                        error_cleared = true;
                    }
                });
            if !open {
                error_cleared = true;
            }
        }

        if error_cleared {
            self.runtime.error_msg = None;
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn show_status_banner(&mut self, ctx: &egui::Context) {
        let Some(status) = self.runtime.status_msg.as_ref() else {
            return;
        };

        let elapsed = status.shown_at.elapsed();
        if elapsed >= ActiveStatusMessage::DISPLAY_FOR {
            self.runtime.status_msg = None;
            return;
        }

        let remaining = ActiveStatusMessage::DISPLAY_FOR - elapsed;
        let fill = match status.level {
            StatusLevel::Success => egui::Color32::from_rgb(28, 78, 52),
            StatusLevel::Error => egui::Color32::from_rgb(120, 34, 34),
        };
        let text = status.text.clone();

        ctx.request_repaint_after(remaining);
        egui::Area::new(egui::Id::new("status_banner"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 40.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).fill(fill).show(ui, |ui| {
                    ui.set_max_width(320.0);
                    ui.label(egui::RichText::new(text).color(egui::Color32::WHITE));
                });
            });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn process_directory_loader(&mut self, ctx: &egui::Context) {
        if !self.catalog.loading_directory {
            return;
        }

        if let Some(rx) = &self.catalog.dir_load_receiver {
            let mut loaded_some = false;
            let disconnected = loop {
                match rx.try_recv() {
                    Ok(entry) => {
                        self.catalog.game_list.push(entry);
                        loaded_some = true;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => break true,
                    Err(std::sync::mpsc::TryRecvError::Empty) => break false,
                }
            };

            if disconnected {
                self.catalog.loading_directory = false;
                self.catalog.dir_load_receiver = None;
                self.catalog
                    .game_list
                    .sort_by(|left, right| left.filename.cmp(&right.filename));
            }

            if loaded_some || self.catalog.loading_directory {
                ctx.request_repaint();
            }
        }
    }
}
