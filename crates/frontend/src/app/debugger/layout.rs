use super::types::{DebuggerTab, PersistedDebuggerLayout};
use egui_dock::{DockState, NodeIndex};

const DEBUGGER_LAYOUT_KEY: &str = "sturdygb_debugger_layout";

pub(crate) fn load_debugger_layout(
    storage: &dyn eframe::Storage,
) -> Option<PersistedDebuggerLayout> {
    eframe::get_value(storage, DEBUGGER_LAYOUT_KEY)
}

pub(super) fn default_dock_state() -> DockState<DebuggerTab> {
    let mut dock_state = DockState::new(vec![DebuggerTab::Disassembly]);
    let surface = dock_state.main_surface_mut();
    let [center, left] = surface.split_left(
        NodeIndex::root(),
        0.22,
        vec![DebuggerTab::Cpu, DebuggerTab::Breakpoints],
    );
    let [_left_top, _left_bottom] =
        surface.split_below(left, 0.58, vec![DebuggerTab::Ppu, DebuggerTab::Apu]);
    let [_center_top, _memory_bottom] =
        surface.split_below(center, 0.58, vec![DebuggerTab::Memory]);
    let [_old_root, right] = surface.split_right(
        NodeIndex::root(),
        0.76,
        vec![DebuggerTab::MemoryMap, DebuggerTab::Writes],
    );
    surface.split_below(
        right,
        0.45,
        vec![DebuggerTab::Vram, DebuggerTab::BgMap, DebuggerTab::Oam],
    );
    dock_state
}
