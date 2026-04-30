mod audio;
mod common;
mod cpu;
mod graphics;
mod memory;

use super::types::{DebuggerTab, DebuggerWindowData};
use crate::app::EmuApp;
use eframe::egui;

pub(super) struct DebuggerTabViewer<'a> {
    pub(super) app: &'a mut EmuApp,
    pub(super) data: Option<&'a DebuggerWindowData>,
}

impl egui_dock::TabViewer for DebuggerTabViewer<'_> {
    type Tab = DebuggerTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.label().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            DebuggerTab::Cpu => self.app.render_cpu_panel(ui, self.data),
            DebuggerTab::Ppu => self.app.render_ppu_panel(ui, self.data),
            DebuggerTab::Apu => self.app.render_apu_panel(ui, self.data),
            DebuggerTab::Breakpoints => self.app.render_breakpoints_panel(ui, self.data),
            DebuggerTab::Disassembly => self.app.render_disassembly_panel(ui, self.data),
            DebuggerTab::Memory => self.app.render_memory_panel(ui, self.data),
            DebuggerTab::MemoryMap => self.app.render_memory_map_panel(ui, self.data),
            DebuggerTab::Writes => self.app.render_writes_panel(ui, self.data),
            DebuggerTab::Vram => self.app.render_vram_panel(ui),
            DebuggerTab::BgMap => self.app.render_bg_map_panel(ui),
            DebuggerTab::Oam => self.app.render_oam_panel(ui),
        }
    }
}