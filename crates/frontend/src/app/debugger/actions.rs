use super::types::{DebuggerStepKind, DebuggerTab};
use crate::app::EmuApp;
use eframe::egui;

impl EmuApp {
    pub(in crate::app) fn debugger_pause_resume_label(&self) -> &'static str {
        if self.runtime.paused {
            "▶ Resume"
        } else {
            "⏸ Pause"
        }
    }

    pub(in crate::app) fn toggle_debugger_pause(&mut self) {
        if self.runtime.paused {
            self.debugger.prepare_resume(self.runtime.loaded_game.as_ref());
        }
        self.runtime.paused = !self.runtime.paused;
    }

    pub(in crate::app) fn open_debugger(&mut self) {
        self.debugger.open();
        self.runtime.paused = true;
    }

    pub(in crate::app) fn open_debugger_tab(&mut self, tab: DebuggerTab) {
        self.debugger.open_tab(tab);
        self.runtime.paused = true;
    }

    pub(in crate::app) fn request_debugger_step(&mut self, step: DebuggerStepKind) {
        match step {
            DebuggerStepKind::Over => self.debugger.request_step_over(self.runtime.loaded_game.as_ref()),
            DebuggerStepKind::Into => self.debugger.request_step_into(self.runtime.loaded_game.as_ref()),
            DebuggerStepKind::Out => self.debugger.request_step_out(self.runtime.loaded_game.as_ref()),
        }
        self.debugger.open();
        self.runtime.paused = false;
    }

    pub(in crate::app) fn show_reset_game_button(ui: &mut egui::Ui, enabled: bool) -> bool {
        ui.add_enabled(enabled, egui::Button::new("🔄 Reset"))
            .clicked()
    }
}
