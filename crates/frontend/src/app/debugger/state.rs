use super::layout::default_dock_state;
use super::types::{DebuggerTab, PendingStep, PersistedDebuggerLayout};
use crate::app::state::LoadedGameState;
use crate::debug_views::find_watchpoint_hit;
use eframe::egui;
use egui_dock::DockState;
use std::collections::BTreeSet;

pub(crate) struct DebuggerUiState {
    pub(super) show_debugger: bool,
    detached: bool,
    pub(super) breakpoints: BTreeSet<u16>,
    pub(super) watchpoints: BTreeSet<u16>,
    pub(super) breakpoint_input: String,
    pub(super) watchpoint_input: String,
    pub(super) memory_jump_input: String,
    pub(super) disassembly_start_input: String,
    pub(super) disassembly_count_input: String,
    pub(super) follow_pc_in_disassembly: bool,
    pub(super) selected_memory_address: Option<u16>,
    pub(super) last_stop_reason: Option<String>,
    pub(super) pending_memory_scroll: Option<u16>,
    pending_step: Option<PendingStep>,
    ignore_breakpoint_once: Option<u16>,
    pub(super) dock_state: DockState<DebuggerTab>,
    pub(super) vram_texture: Option<egui::TextureHandle>,
    pub(super) bg_map_texture: Option<egui::TextureHandle>,
    pub(super) oam_texture: Option<egui::TextureHandle>,
    pub(super) selected_vram_bank: usize,
    pub(super) selected_bg_map: usize,
}

impl DebuggerUiState {
    pub(crate) fn new(layout: Option<PersistedDebuggerLayout>) -> Self {
        Self {
            show_debugger: false,
            detached: false,
            breakpoints: BTreeSet::new(),
            watchpoints: BTreeSet::new(),
            breakpoint_input: "0100".to_string(),
            watchpoint_input: "C000".to_string(),
            memory_jump_input: "C000".to_string(),
            disassembly_start_input: "0100".to_string(),
            disassembly_count_input: "0040".to_string(),
            follow_pc_in_disassembly: true,
            selected_memory_address: None,
            last_stop_reason: None,
            pending_memory_scroll: None,
            pending_step: None,
            ignore_breakpoint_once: None,
            dock_state: layout
                .map(|layout| layout.dock_state)
                .unwrap_or_else(default_dock_state),
            vram_texture: None,
            bg_map_texture: None,
            oam_texture: None,
            selected_vram_bank: 0,
            selected_bg_map: 0,
        }
    }

    pub(crate) fn open(&mut self) {
        self.show_debugger = true;
    }

    pub(crate) fn close(&mut self) {
        self.show_debugger = false;
    }

    pub(crate) fn is_detached(&self) -> bool {
        self.detached
    }

    pub(crate) fn set_detached(&mut self, detached: bool) {
        self.detached = detached;
    }

    pub(crate) fn toggle_detached(&mut self) {
        self.detached = !self.detached;
    }

    pub(crate) fn open_tab(&mut self, tab: DebuggerTab) {
        self.show_debugger = true;
        if self.dock_state.find_tab(&tab).is_none() {
            self.dock_state.push_to_focused_leaf(tab);
        }
        if let Some(location) = self.dock_state.find_tab(&tab) {
            self.dock_state.set_active_tab(location);
            self.dock_state
                .set_focused_node_and_surface((location.0, location.1));
        }
    }

    pub(crate) fn save_layout(&self, storage: &mut dyn eframe::Storage) {
        let persisted = PersistedDebuggerLayout {
            dock_state: self.dock_state.clone(),
        };
        eframe::set_value(storage, "sturdygb_debugger_layout", &persisted);
    }

    pub(crate) fn reset_layout(&mut self) {
        self.dock_state = default_dock_state();
    }

    pub(crate) fn reset_runtime(&mut self) {
        self.last_stop_reason = None;
        self.pending_step = None;
        self.pending_memory_scroll = None;
        self.ignore_breakpoint_once = None;
        self.vram_texture = None;
        self.bg_map_texture = None;
        self.oam_texture = None;
        self.selected_vram_bank = 0;
        self.selected_bg_map = 0;
        self.selected_memory_address = None;
    }

    pub(crate) fn prepare_resume(&mut self, state: Option<&LoadedGameState>) {
        self.last_stop_reason = None;
        self.pending_step = None;
        self.ignore_breakpoint_once = state.map(|state| state.gb.current_pc());
    }

    pub(crate) fn request_step_into(&mut self, state: Option<&LoadedGameState>) {
        self.last_stop_reason = None;
        self.pending_step = Some(PendingStep::Into);
        self.ignore_breakpoint_once = state.map(|state| state.gb.current_pc());
    }

    pub(crate) fn request_step_over(&mut self, state: Option<&LoadedGameState>) {
        self.last_stop_reason = None;
        self.pending_step = state.map_or(Some(PendingStep::Into), |state| {
            let opcode = state
                .gb
                .read_memory_range(state.gb.current_pc(), 1)
                .first()
                .copied()
                .unwrap_or(0);
            if is_step_over_opcode(opcode) {
                Some(PendingStep::Over {
                    target_pc: state
                        .gb
                        .current_pc()
                        .wrapping_add(state.gb.current_opcode_size()),
                    initial_sp: state.gb.cpu.sp,
                })
            } else {
                Some(PendingStep::Into)
            }
        });
        self.ignore_breakpoint_once = state.map(|state| state.gb.current_pc());
    }

    pub(crate) fn request_step_out(&mut self, state: Option<&LoadedGameState>) {
        self.last_stop_reason = None;
        self.pending_step = state.and_then(|state| {
            let sp = state.gb.cpu.sp;
            let stack = state.gb.read_memory_range(sp, 2);
            if stack.len() < 2 {
                return Some(PendingStep::Into);
            }

            Some(PendingStep::Out {
                target_pc: u16::from(stack[0]) | (u16::from(stack[1]) << 8),
                target_sp: sp.wrapping_add(2),
            })
        });
        self.ignore_breakpoint_once = state.map(|state| state.gb.current_pc());
    }

    pub(super) fn focus_memory_address(&mut self, address: u16) {
        self.selected_memory_address = Some(address);
        self.memory_jump_input = format!("{:04X}", address);
        self.pending_memory_scroll = Some(address);
    }

    fn needs_instruction_loop(&self) -> bool {
        self.pending_step.is_some() || !self.breakpoints.is_empty() || !self.watchpoints.is_empty()
    }

    pub(crate) fn run_until_debug_or_frame(&mut self, state: &mut LoadedGameState) -> bool {
        if !self.needs_instruction_loop() {
            state.gb.run_one_frame();
            return false;
        }

        loop {
            let pc = state.gb.current_pc();
            let ignore_breakpoint = self.ignore_breakpoint_once == Some(pc);

            if self.breakpoints.contains(&pc) && !ignore_breakpoint {
                self.last_stop_reason = Some(format!("Breakpoint hit at {:04X}", pc));
                self.show_debugger = true;
                self.pending_step = None;
                return true;
            }

            if ignore_breakpoint {
                self.ignore_breakpoint_once = None;
            }

            state.gb.step_instruction();

            if let Some(hit) = find_watchpoint_hit(state.gb.last_memory_writes(), &self.watchpoints)
            {
                self.last_stop_reason = Some(format!(
                    "Watchpoint hit at {:04X} = {:02X}",
                    hit.address, hit.value
                ));
                self.show_debugger = true;
                self.pending_step = None;
                self.ignore_breakpoint_once = None;
                return true;
            }

            if let Some(reason) = self.finish_pending_step(state) {
                self.last_stop_reason = Some(reason);
                self.show_debugger = true;
                self.ignore_breakpoint_once = None;
                return true;
            }

            if state.gb.frame_ready() {
                return false;
            }
        }
    }

    fn finish_pending_step(&mut self, state: &LoadedGameState) -> Option<String> {
        let pending = self.pending_step?;
        let pc = state.gb.current_pc();
        let sp = state.gb.cpu.sp;
        let completed = match pending {
            PendingStep::Into => true,
            PendingStep::Over {
                target_pc,
                initial_sp,
            } => pc == target_pc && sp == initial_sp,
            PendingStep::Out {
                target_pc,
                target_sp,
            } => pc == target_pc && sp == target_sp,
        };

        if !completed {
            return None;
        }

        self.pending_step = None;
        Some(format!("{} completed at {:04X}", pending.label(), pc))
    }
}

pub(super) fn is_step_over_opcode(opcode: u8) -> bool {
    matches!(
        opcode,
        0xC4 | 0xCC | 0xCD | 0xD4 | 0xDC | 0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF
    )
}
