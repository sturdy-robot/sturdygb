use super::{EmuApp, APP_NAME};
use eframe::egui;

const WEBSITE_URL: &str = "https://sturdygb.dev";
const REPOSITORY_URL: &str = env!("CARGO_PKG_REPOSITORY");
const LICENSE_NAME: &str = "MIT";
const LICENSE_TEXT: &str = include_str!("../../../../LICENSE.md");

#[derive(Default)]
pub(super) struct HelpUiState {
    show_about: bool,
    show_license: bool,
}

impl EmuApp {
    pub(super) fn show_help_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                self.ui.help.show_about = true;
                ui.close();
            }

            if ui.button("Website").clicked() {
                open_url(ui.ctx(), WEBSITE_URL);
                ui.close();
            }

            if ui.button("GitHub").clicked() {
                open_url(ui.ctx(), REPOSITORY_URL);
                ui.close();
            }

            if ui.button("License Information").clicked() {
                self.ui.help.show_license = true;
                ui.close();
            }
        });
    }

    pub(super) fn show_help_windows(&mut self, ctx: &egui::Context) {
        if self.ui.help.show_about {
            egui::Window::new("About SturdyGB")
                .open(&mut self.ui.help.show_about)
                .collapsible(false)
                .resizable(false)
                .default_width(420.0)
                .show(ctx, |ui| {
                    ui.heading(APP_NAME);
                    ui.label("Author: Sturdy Robot");
                    ui.label("A free and open source Game Boy and Game Boy Color emulator.");
                    ui.label("Distributed under the MIT License.");
                    ui.separator();
                    ui.label(format!("Website: {WEBSITE_URL}"));
                    ui.label(format!("GitHub: {REPOSITORY_URL}"));
                    ui.label(format!("License: {LICENSE_NAME}"));
                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Open Website").clicked() {
                            open_url(ctx, WEBSITE_URL);
                        }

                        if ui.button("Open GitHub").clicked() {
                            open_url(ctx, REPOSITORY_URL);
                        }
                    });
                });
        }

            if self.ui.help.show_license {
            egui::Window::new("License Information")
                .open(&mut self.ui.help.show_license)
                .collapsible(false)
                .default_width(640.0)
                .default_height(520.0)
                .show(ctx, |ui| {
                    ui.heading(format!("{LICENSE_NAME} License"));
                    ui.label("This build of SturdyGB is distributed under the MIT License.");
                    ui.label("The full license text bundled with the project is shown below.");
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.monospace(LICENSE_TEXT);
                        });
                });
        }
    }
}

fn open_url(ctx: &egui::Context, url: &str) {
    ctx.open_url(egui::OpenUrl::new_tab(url));
}
