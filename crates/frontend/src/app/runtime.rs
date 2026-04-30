use super::EmuApp;
use eframe::egui;

impl EmuApp {
    pub(super) fn sync_viewport_state(&mut self, ctx: &egui::Context) {
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
        if let Ok(result) = self.runtime.rom_load_channel.1.try_recv() {
            match result {
                Ok(bytes) => self.load_rom_bytes(bytes, None, storage),
                Err(error) => {
                    self.runtime.error_msg = Some(format!("Failed to load ROM via async: {error}"))
                }
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