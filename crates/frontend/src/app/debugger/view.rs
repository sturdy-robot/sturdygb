use super::layout::default_dock_state;
use super::panels::DebuggerTabViewer;
use super::types::{DebuggerStepKind, DebuggerTab, DebuggerWindowData};
use crate::app::EmuApp;
use crate::debug_views::{parse_hex_u16, parse_hex_usize};
use eframe::egui;
use egui_dock::{DockArea, Style};

const DEBUGGER_VIEWPORT_ID: &str = "sturdygb_debugger_viewport";

impl EmuApp {
    pub(crate) fn show_debugger_windows(
        &mut self,
        ctx: &egui::Context,
        storage: Option<&dyn eframe::Storage>,
    ) {
        if !self.debugger.show_debugger {
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        if self.debugger.is_detached() {
            self.show_detached_debugger_viewport(ctx, storage);
            return;
        }

        self.show_debugger_window(ctx, storage);
    }

    fn show_debugger_window(&mut self, ctx: &egui::Context, storage: Option<&dyn eframe::Storage>) {
        let mut open = self.debugger.show_debugger;
        if !open {
            return;
        }

        let data = self.build_debugger_window_data();
        let has_state = self.has_loaded_game();

        egui::Window::new("Debugger")
            .open(&mut open)
            .default_size(egui::vec2(1280.0, 840.0))
            .show(ctx, |ui| {
                self.render_debugger_contents(ui, data.as_ref(), has_state, storage);
            });
        self.debugger.show_debugger = open;
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn show_detached_debugger_viewport(
        &mut self,
        ctx: &egui::Context,
        storage: Option<&dyn eframe::Storage>,
    ) {
        let data = self.build_debugger_window_data();
        let has_state = self.has_loaded_game();
        let viewport_id = egui::ViewportId::from_hash_of(DEBUGGER_VIEWPORT_ID);
        let viewport_builder = egui::ViewportBuilder::default()
            .with_title("Debugger")
            .with_inner_size(egui::vec2(1280.0, 840.0))
            .with_min_inner_size(egui::vec2(960.0, 600.0));
        let mut close_requested = false;

        ctx.show_viewport_immediate(viewport_id, viewport_builder, |viewport_ctx, class| {
            if matches!(class, egui::ViewportClass::EmbeddedWindow) {
                let mut open = true;
                egui::Window::new("Debugger")
                    .open(&mut open)
                    .default_size(egui::vec2(1280.0, 840.0))
                    .show(viewport_ctx, |ui| {
                        self.render_debugger_contents(ui, data.as_ref(), has_state, storage);
                    });
                if !open {
                    close_requested = true;
                }
                return;
            }

            if viewport_ctx.input(|input| input.viewport().close_requested()) {
                close_requested = true;
                return;
            }

            egui::CentralPanel::default().show(viewport_ctx, |ui| {
                self.render_debugger_contents(ui, data.as_ref(), has_state, storage);
            });
        });

        if close_requested {
            self.debugger.close();
            self.debugger.set_detached(false);
        }
    }

    fn build_debugger_window_data(&mut self) -> Option<DebuggerWindowData> {
        let state = self.runtime.loaded_game.as_mut()?;
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
    }

    fn render_debugger_contents(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
        has_state: bool,
        storage: Option<&dyn eframe::Storage>,
    ) {
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

            ui.menu_button("Window", |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let label = if self.debugger.is_detached() {
                        "Attach to Main Window"
                    } else {
                        "Detach to Native Window"
                    };
                    if ui.button(label).clicked() {
                        self.debugger.toggle_detached();
                        ui.close();
                    }
                }

                #[cfg(target_arch = "wasm32")]
                ui.label("Native window detachment is unavailable on web builds.");
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

        ui.horizontal(|ui| {
            ui.button(self.debugger_pause_resume_label())
                .clicked()
                .then(|| self.toggle_debugger_pause());

            for &step in DebuggerStepKind::all() {
                ui.button(step.label())
                    .clicked()
                    .then(|| self.request_debugger_step(step));
            }

            Self::show_reset_game_button(ui, has_state);
        });
        ui.separator();

        let mut dock_state = std::mem::replace(&mut self.debugger.dock_state, default_dock_state());
        let mut viewer = DebuggerTabViewer { app: self, data };
        DockArea::new(&mut dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut viewer);
        self.debugger.dock_state = dock_state;
    }
}
