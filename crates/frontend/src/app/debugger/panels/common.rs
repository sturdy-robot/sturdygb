use super::super::types::{DebuggerTab, DebuggerWindowData};
use crate::app::EmuApp;
use eframe::egui;

pub(super) fn require_debugger_data<'a>(
    ui: &mut egui::Ui,
    data: Option<&'a DebuggerWindowData>,
    message: &str,
) -> Option<&'a DebuggerWindowData> {
    if let Some(data) = data {
        Some(data)
    } else {
        ui.label(message);
        None
    }
}

pub(super) fn jump_to_memory(app: &mut EmuApp, address: u16) {
    app.debugger.focus_memory_address(address);
    app.open_debugger_tab(DebuggerTab::Memory);
}

pub(super) fn update_texture(
    handle: &mut Option<egui::TextureHandle>,
    ctx: &egui::Context,
    name: &str,
    image: egui::ColorImage,
) -> egui::TextureHandle {
    if let Some(texture) = handle.as_mut() {
        texture.set(image, egui::TextureOptions::NEAREST);
        texture.clone()
    } else {
        let texture = ctx.load_texture(name, image, egui::TextureOptions::NEAREST);
        *handle = Some(texture.clone());
        texture
    }
}