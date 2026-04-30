use super::layout::default_dock_state;
use super::panels::DebuggerTabViewer;
use super::types::{DebuggerStepKind, DebuggerTab, DebuggerWindowData};
use crate::app::EmuApp;
use crate::debug_views::{parse_hex_u16, parse_hex_usize};
use eframe::egui;
use egui_dock::{DockArea, Style};

impl EmuApp {
    pub(crate) fn show_debugger_windows(
        &mut self,
        ctx: &egui::Context,
        storage: Option<&dyn eframe::Storage>,
    ) {
        self.show_debugger_window(ctx, storage);
    }

    fn show_debugger_window(&mut self, ctx: &egui::Context, storage: Option<&dyn eframe::Storage>) {
        let mut open = self.debugger.show_debugger;
        if !open {
            return;
        }

        let data = if let Some(state) = self.runtime.loaded_game.as_mut() {
            let snapshot = state.gb.debug_snapshot();
            let disassembly_start = if self.debugger.follow_pc_in_disassembly {
                snapshot.pc
            } else {
                parse_hex_u16(&self.debugger.disassembly_start_input, snapshot.pc)
            };
            let disassembly_count =
                parse_hex_usize(&self.debugger.disassembly_count_input, 0x40).clamp(1, 0x200);
            Some(DebuggerWindowData {
                snapshot,
                ppu_snapshot: state.gb.ppu_debug_snapshot(),
                apu_snapshot: state.gb.apu_debug_snapshot(),
                memory: state.gb.read_memory_range(0, 0x10000),
                disassembly: state
                    .gb
                    .disassemble_range(disassembly_start, disassembly_count),
                last_writes: state.gb.last_memory_writes().to_vec(),
            })
        } else {
            None
        };

        let has_state = self.has_loaded_game();
        egui::Window::new("Debugger")
            .open(&mut open)
            .default_size(egui::vec2(1280.0, 840.0))
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("Control", |ui| {
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

                        for &step in DebuggerStepKind::all() {
                            if ui
                                .add_enabled(has_state, egui::Button::new(step.label()))
                                .clicked()
                            {
                                self.request_debugger_step(step);
                                ui.close();
                            }
                        }

                        if Self::show_reset_game_button(ui, has_state) {
                            self.reset_loaded_rom(storage);
                            ui.close();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        for &tab in DebuggerTab::all() {
                            if ui.button(tab.label()).clicked() {
                                self.open_debugger_tab(tab);
                                ui.close();
                            }
                        }
                    });

                    ui.menu_button("Layout", |ui| {
                        if ui.button("Reset Layout").clicked() {
                            self.debugger.reset_layout();
                            ui.close();
                        }
                    });
                });

                let status = self.debugger.last_stop_reason.clone().unwrap_or_else(|| {
                    if self.is_paused() {
                        "Paused"
                    } else {
                        "Running"
                    }
                    .to_string()
                });
                ui.label(status);
                ui.separator();

                let toggle_label = self.debugger_pause_resume_label();

                ui.horizontal(|ui| {
                    ui.button(toggle_label)
                        .clicked()
                        .then(|| self.toggle_debugger_pause());
                    for &step in DebuggerStepKind::all() {
                        if ui
                            .add_enabled(has_state, egui::Button::new(step.label()))
                            .clicked()
                        {
                            self.request_debugger_step(step);
                            ui.close();
                        }
                    }
                    if Self::show_reset_game_button(ui, has_state) {
                        self.reset_loaded_rom(storage);
                    }
                });
                ui.separator();

                let mut dock_state =
                    std::mem::replace(&mut self.debugger.dock_state, default_dock_state());
                let mut viewer = DebuggerTabViewer {
                    app: self,
                    data: data.as_ref(),
                };
                DockArea::new(&mut dock_state)
                    .style(Style::from_egui(ui.style().as_ref()))
                    .show_inside(ui, &mut viewer);
                self.debugger.dock_state = dock_state;
            });
        self.debugger.show_debugger = open;
    }
}
