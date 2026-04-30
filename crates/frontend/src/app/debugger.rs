mod actions;
mod layout;
mod panels;
mod state;
mod types;
mod view;

pub(super) use layout::load_debugger_layout;
pub(super) use state::DebuggerUiState;
pub(crate) use types::{DebuggerStepKind, DebuggerTab};

#[cfg(test)]
mod tests {
    use super::DebuggerTab;
    use super::{
        layout::default_dock_state,
        state::{is_step_over_opcode, DebuggerUiState},
        types::{MemoryRegion, PersistedDebuggerLayout},
    };
    use egui_dock::DockState;

    #[test]
    fn default_layout_contains_all_core_tabs() {
        let dock_state = default_dock_state();
        for &tab in DebuggerTab::all() {
            assert!(dock_state.find_tab(&tab).is_some(), "missing tab: {tab:?}");
        }
    }

    #[test]
    fn memory_region_classifies_boundaries() {
        assert!(matches!(
            MemoryRegion::for_address(0x0000),
            MemoryRegion::Rom0
        ));
        assert!(matches!(
            MemoryRegion::for_address(0x8000),
            MemoryRegion::Vram
        ));
        assert!(matches!(
            MemoryRegion::for_address(0xC000),
            MemoryRegion::Wram0
        ));
        assert!(matches!(
            MemoryRegion::for_address(0xFEA0),
            MemoryRegion::Unusable
        ));
        assert!(matches!(
            MemoryRegion::for_address(0xFFFF),
            MemoryRegion::InterruptEnable
        ));
    }

    #[test]
    fn step_over_matches_call_like_opcodes() {
        for opcode in [
            0xC4, 0xCC, 0xCD, 0xD4, 0xDC, 0xC7, 0xCF, 0xD7, 0xDF, 0xE7, 0xEF, 0xF7, 0xFF,
        ] {
            assert!(
                is_step_over_opcode(opcode),
                "expected {opcode:02X} to step over"
            );
        }
        for opcode in [0x00, 0x18, 0x20, 0xC9] {
            assert!(
                !is_step_over_opcode(opcode),
                "expected {opcode:02X} to step into"
            );
        }
    }

    #[test]
    fn open_tab_inserts_missing_tab_into_persisted_layout() {
        let layout = PersistedDebuggerLayout {
            dock_state: DockState::new(vec![DebuggerTab::Cpu]),
        };
        let mut debugger = DebuggerUiState::new(Some(layout));

        debugger.open_tab(DebuggerTab::Ppu);

        assert!(debugger.dock_state.find_tab(&DebuggerTab::Ppu).is_some());
    }
}
