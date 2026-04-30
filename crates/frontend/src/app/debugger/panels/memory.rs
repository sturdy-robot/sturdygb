use super::common::{jump_to_memory, require_debugger_data};
use super::super::types::{DebuggerWindowData, MemoryRegion};
use crate::app::EmuApp;
use crate::debug_views::{format_byte_list, parse_hex_u16};
use eframe::egui;
use std::collections::BTreeSet;

const MEMORY_ROWS: usize = 0x10000 / 16;

impl EmuApp {
    pub(super) fn render_breakpoints_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let pc = data.map(|data| data.snapshot.pc).unwrap_or(0x0100);
        ui.horizontal(|ui| {
            ui.label("Breakpoint");
            ui.text_edit_singleline(&mut self.debugger.breakpoint_input);
            if ui.button("Add").clicked() {
                let address = parse_hex_u16(&self.debugger.breakpoint_input, pc);
                self.debugger.breakpoints.insert(address);
                self.debugger.breakpoint_input = format!("{:04X}", address);
            }
            if ui.button("PC").clicked() {
                self.debugger.breakpoints.insert(pc);
                self.debugger.breakpoint_input = format!("{:04X}", pc);
            }
        });

        egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
            for address in self.debugger.breakpoints.iter().copied().collect::<Vec<_>>() {
                ui.horizontal(|ui| {
                    if ui.button(format!("{:04X}", address)).clicked() {
                        jump_to_memory(self, address);
                    }
                    if ui.button("Remove").clicked() {
                        self.debugger.breakpoints.remove(&address);
                    }
                });
            }
        });

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Watchpoint");
            ui.text_edit_singleline(&mut self.debugger.watchpoint_input);
            if ui.button("Add").clicked() {
                let address = parse_hex_u16(&self.debugger.watchpoint_input, 0xC000);
                self.debugger.watchpoints.insert(address);
                self.debugger.watchpoint_input = format!("{:04X}", address);
            }
        });

        egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
            for address in self.debugger.watchpoints.iter().copied().collect::<Vec<_>>() {
                ui.horizontal(|ui| {
                    if ui.button(format!("{:04X}", address)).clicked() {
                        jump_to_memory(self, address);
                    }
                    if ui.button("Remove").clicked() {
                        self.debugger.watchpoints.remove(&address);
                    }
                });
            }
        });
    }

    pub(super) fn render_writes_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let Some(data) = require_debugger_data(ui, data, "Load a ROM to inspect write activity.") else {
            return;
        };

        if data.last_writes.is_empty() {
            ui.label("No recent writes captured.");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for write in data.last_writes.iter().rev() {
                ui.horizontal(|ui| {
                    if ui.button(format!("{:04X}", write.address)).clicked() {
                        jump_to_memory(self, write.address);
                    }
                    ui.monospace(format!("= {:02X}", write.value));
                });
            }
        });
    }

    pub(super) fn render_disassembly_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let Some(data) = require_debugger_data(ui, data, "Load a ROM to inspect disassembly.") else {
            return;
        };

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.debugger.follow_pc_in_disassembly, "Follow PC");
            ui.label("Start");
            ui.add_enabled_ui(!self.debugger.follow_pc_in_disassembly, |ui| {
                ui.text_edit_singleline(&mut self.debugger.disassembly_start_input);
            });
            ui.label("Count");
            ui.text_edit_singleline(&mut self.debugger.disassembly_count_input);
            if ui.button("PC").clicked() {
                self.debugger.disassembly_start_input = format!("{:04X}", data.snapshot.pc);
            }
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in &data.disassembly {
                ui.horizontal(|ui| {
                    ui.monospace(if line.address == data.snapshot.pc { "▶" } else { " " });

                    let is_breakpoint = self.debugger.breakpoints.contains(&line.address);
                    let dot = if is_breakpoint { "●" } else { "○" };
                    if ui.button(dot).clicked() {
                        if is_breakpoint {
                            self.debugger.breakpoints.remove(&line.address);
                        } else {
                            self.debugger.breakpoints.insert(line.address);
                        }
                    }

                    if ui.button(format!("{:04X}", line.address)).clicked() {
                        jump_to_memory(self, line.address);
                    }

                    ui.monospace(format!("{:<11}", format_byte_list(&line.bytes)));
                    let text = if line.address == data.snapshot.pc {
                        egui::RichText::new(&line.text).monospace().strong()
                    } else {
                        egui::RichText::new(&line.text).monospace()
                    };
                    ui.label(text);
                });
            }
        });
    }

    pub(super) fn render_memory_map_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let current = data
            .and_then(|data| self.debugger.selected_memory_address.or(Some(data.snapshot.pc)))
            .unwrap_or(0x0000);
        let current_region = MemoryRegion::for_address(current);
        ui.label(format!(
            "Current: {:04X} ({})",
            current,
            current_region.label()
        ));
        ui.separator();

        for &(region, start, end) in MemoryRegion::all() {
            let selected = region == current_region;
            if ui
                .selectable_label(
                    selected,
                    format!("{}  {:04X}-{:04X}", region.label(), start, end),
                )
                .clicked()
            {
                jump_to_memory(self, start);
            }
        }
    }

    pub(super) fn render_memory_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let Some(data) = require_debugger_data(ui, data, "Load a ROM to inspect memory.") else {
            return;
        };

        ui.horizontal(|ui| {
            ui.label("Jump");
            ui.text_edit_singleline(&mut self.debugger.memory_jump_input);
            if ui.button("Go").clicked() {
                let address = parse_hex_u16(&self.debugger.memory_jump_input, data.snapshot.pc);
                self.debugger.focus_memory_address(address);
            }
            if ui.button("PC").clicked() {
                self.debugger.focus_memory_address(data.snapshot.pc);
            }
            if ui.button("SP").clicked() {
                self.debugger.focus_memory_address(data.snapshot.sp);
            }
        });

        let row_height = ui
            .spacing()
            .interact_size
            .y
            .max(ui.text_style_height(&egui::TextStyle::Monospace));
        let write_addresses: BTreeSet<u16> =
            data.last_writes.iter().map(|write| write.address).collect();
        let pending_row = self
            .debugger
            .pending_memory_scroll
            .map(|address| usize::from(address) / 16);

        let mut scroll_area = egui::ScrollArea::vertical()
            .id_salt("debugger_memory_scroll")
            .auto_shrink([false, false]);
        if let Some(target_row) = pending_row {
            scroll_area = scroll_area.vertical_scroll_offset((target_row as f32) * row_height);
        }

        scroll_area.show_rows(ui, row_height, MEMORY_ROWS, |ui, row_range| {
            for row in row_range {
                let base = row * 16;
                ui.horizontal(|ui| {
                    ui.monospace(format!("{:04X}", base));
                    for column in 0..16 {
                        let address = (base + column) as u16;
                        let byte = data.memory[base + column];
                        let mut text = egui::RichText::new(format!("{:02X}", byte)).monospace();
                        if Some(address) == self.debugger.selected_memory_address {
                            text = text.background_color(ui.visuals().selection.bg_fill);
                        } else if address == data.snapshot.pc {
                            text = text.color(ui.visuals().warn_fg_color);
                        } else if address == data.snapshot.sp {
                            text = text.color(ui.visuals().hyperlink_color);
                        } else if write_addresses.contains(&address) {
                            text = text.color(ui.visuals().strong_text_color());
                        } else if self.debugger.watchpoints.contains(&address) {
                            text = text.color(ui.visuals().error_fg_color);
                        }
                        if ui
                            .selectable_label(
                                Some(address) == self.debugger.selected_memory_address,
                                text,
                            )
                            .clicked()
                        {
                            self.debugger.focus_memory_address(address);
                        }
                    }

                    let ascii = (0..16)
                        .map(|column| {
                            let byte = data.memory[base + column];
                            if byte.is_ascii_graphic() {
                                byte as char
                            } else {
                                '.'
                            }
                        })
                        .collect::<String>();
                    ui.monospace(ascii);
                });
            }
        });

        self.debugger.pending_memory_scroll = None;
    }
}