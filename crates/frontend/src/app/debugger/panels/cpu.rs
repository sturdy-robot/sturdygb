use super::common::require_debugger_data;
use super::super::types::DebuggerWindowData;
use crate::app::EmuApp;
use crate::debug_views::format_byte_list;
use eframe::egui;

impl EmuApp {
    pub(super) fn render_cpu_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let Some(data) = require_debugger_data(ui, data, "Load a ROM to inspect debugger state.") else {
            return;
        };

        let flags = data.snapshot.af as u8;
        egui::Grid::new("debugger_registers")
            .num_columns(2)
            .spacing([16.0, 4.0])
            .show(ui, |ui| {
                ui.label("AF");
                ui.monospace(format!("{:04X}", data.snapshot.af));
                ui.end_row();

                ui.label("BC");
                ui.monospace(format!("{:04X}", data.snapshot.bc));
                ui.end_row();

                ui.label("DE");
                ui.monospace(format!("{:04X}", data.snapshot.de));
                ui.end_row();

                ui.label("HL");
                ui.monospace(format!("{:04X}", data.snapshot.hl));
                ui.end_row();
                
                ui.label("SP");
                ui.monospace(format!("{:04X}", data.snapshot.sp));
                ui.end_row();

                ui.label("PC");
                ui.monospace(format!("{:04X}", data.snapshot.pc));
                ui.end_row();
                
                ui.label("OP");
                ui.monospace(format!("{:02X}", data.snapshot.opcode));
                ui.end_row();

                ui.label("LY");
                ui.monospace(format!("{:02X}", data.snapshot.ly));
                ui.end_row();
                
                ui.label("STAT");
                ui.monospace(format!("{:02X}", data.snapshot.stat));
                ui.end_row();

                ui.label("IME");
                ui.monospace(if data.snapshot.interrupt_master { "1" } else { "0" });
                ui.end_row();
                
                ui.label("IF/IE");
                ui.monospace(format!(
                    "{:02X}/{:02X}",
                    data.snapshot.if_flag, data.snapshot.ie_flag
                ));
                ui.end_row();

                ui.label("Ticks");
                ui.monospace(data.snapshot.ticks.to_string());
                ui.end_row();
                
                ui.label("Flags");
                ui.monospace(format!(
                    "Z:{} N:{} H:{} C:{}",
                    (flags >> 7) & 1,
                    (flags >> 6) & 1,
                    (flags >> 5) & 1,
                    (flags >> 4) & 1
                ));
                ui.end_row();
                
                ui.label("State");
                ui.monospace(format!(
                    "halt:{} stop:{} cyc:{}",
                    data.snapshot.is_halted, data.snapshot.is_stopped, data.snapshot.pending_cycles
                ));
                ui.end_row();
            });

        ui.separator();
        ui.label(
            egui::RichText::new(&data.snapshot.disassembly)
                .monospace()
                .strong(),
        );
        ui.label(
            egui::RichText::new(format!(
                "PC bytes: {}",
                format_byte_list(&data.snapshot.pc_bytes)
            ))
            .monospace(),
        );
        ui.label(
            egui::RichText::new(format!(
                "Stack: {}",
                format_byte_list(&data.snapshot.stack_bytes)
            ))
            .monospace(),
        );
    }
}