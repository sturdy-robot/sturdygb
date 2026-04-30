mod game;
mod library;

use super::EmuApp;
use eframe::egui;

impl EmuApp {
    pub(super) fn show_main_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.has_loaded_game() {
                self.show_running_game_panel(ui, ctx, frame);
            } else {
                self.show_game_library_panel(ui, frame);
            }
        });
    }
}
