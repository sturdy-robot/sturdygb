use super::super::types::DebuggerWindowData;
use super::common::require_debugger_data;
use crate::app::EmuApp;
use eframe::egui;
use sturdygb_core::gb::{
    NoiseChannelDebugSnapshot, SquareChannelDebugSnapshot, WaveChannelDebugSnapshot,
};

impl EmuApp {
    pub(super) fn render_apu_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let Some(data) = require_debugger_data(ui, data, "Load a ROM to inspect APU state.") else {
            return;
        };

        let apu = &data.apu_snapshot;
        egui::Grid::new("debugger_apu_state")
            .num_columns(2)
            .spacing([16.0, 4.0])
            .show(ui, |ui| {
                ui.label("Enabled");
                ui.monospace(if apu.enabled { "1" } else { "0" });
                ui.end_row();

                ui.label("Sample Rate");
                ui.monospace(apu.sample_rate.to_string());
                ui.end_row();

                ui.label("Master Vol");
                ui.monospace(format!("L:{} R:{}", apu.vol_left, apu.vol_right));
                ui.end_row();

                ui.label("VIN");
                ui.monospace(format!("L:{} R:{}", apu.vin_left, apu.vin_right));
                ui.end_row();

                ui.label("Pan Left");
                ui.monospace(format_pan_channels(&apu.pan_left));
                ui.end_row();

                ui.label("Pan Right");
                ui.monospace(format_pan_channels(&apu.pan_right));
                ui.end_row();

                ui.label("Frame Seq");
                ui.monospace(format!(
                    "step:{} timer:{}",
                    apu.frame_seq_step, apu.frame_seq_timer
                ));
                ui.end_row();
            });

        ui.separator();
        egui::CollapsingHeader::new("CH1 Pulse + Sweep")
            .default_open(true)
            .show(ui, |ui| render_square_channel(ui, &apu.ch1));
        egui::CollapsingHeader::new("CH2 Pulse")
            .default_open(true)
            .show(ui, |ui| render_square_channel(ui, &apu.ch2));
        egui::CollapsingHeader::new("CH3 Wave")
            .default_open(true)
            .show(ui, |ui| render_wave_channel(ui, &apu.ch3));
        egui::CollapsingHeader::new("CH4 Noise")
            .default_open(true)
            .show(ui, |ui| render_noise_channel(ui, &apu.ch4));
    }
}

fn format_pan_channels(pan: &[bool; 4]) -> String {
    let label = pan
        .iter()
        .enumerate()
        .filter_map(|(index, enabled)| enabled.then_some(format!("CH{}", index + 1)))
        .collect::<Vec<_>>()
        .join(" ");
    if label.is_empty() {
        "none".to_string()
    } else {
        label
    }
}

fn render_square_channel(ui: &mut egui::Ui, channel: &SquareChannelDebugSnapshot) {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(2)
        .spacing([16.0, 4.0])
        .show(ui, |ui| {
            ui.label("Enabled");
            ui.monospace(format!("{} / DAC {}", channel.enabled, channel.dac_enabled));
            ui.end_row();

            ui.label("Duty / Pos");
            ui.monospace(format!("{} / {}", channel.duty, channel.duty_pos));
            ui.end_row();

            ui.label("Freq / Timer");
            ui.monospace(format!("{} / {}", channel.frequency, channel.freq_timer));
            ui.end_row();
            
            ui.label("Length");
            ui.monospace(format!(
                "{} ({})",
                channel.length_timer, channel.length_enabled
            ));
            ui.end_row();

            ui.label("Envelope");
            ui.monospace(format!(
                "vol:{} period:{} dir:{}",
                channel.volume, channel.envelope_period, channel.envelope_direction
            ));
            ui.end_row();
            
            ui.label("Sweep");
            ui.monospace(format!(
                "period:{} dir:{} shift:{}",
                channel.sweep_period.unwrap_or(0),
                channel.sweep_direction.unwrap_or(0),
                channel.sweep_shift.unwrap_or(0)
            ));
            ui.end_row();
        });
}

fn render_wave_channel(ui: &mut egui::Ui, channel: &WaveChannelDebugSnapshot) {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(4)
        .spacing([16.0, 4.0])
        .show(ui, |ui| {
            ui.label("Enabled");
            ui.monospace(format!("{} / DAC {}", channel.enabled, channel.dac_enabled));
            ui.label("Wave Pos");
            ui.monospace(channel.wave_pos.to_string());
            ui.end_row();

            ui.label("Freq / Timer");
            ui.monospace(format!("{} / {}", channel.frequency, channel.freq_timer));
            ui.label("Length");
            ui.monospace(format!(
                "{} ({})",
                channel.length_timer, channel.length_enabled
            ));
            ui.end_row();

            ui.label("Volume Shift");
            ui.monospace(channel.volume_shift.to_string());
            ui.label("Sample Buf");
            ui.monospace(format!("{:02X}", channel.sample_buf));
            ui.end_row();
        });
}

fn render_noise_channel(ui: &mut egui::Ui, channel: &NoiseChannelDebugSnapshot) {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(4)
        .spacing([16.0, 4.0])
        .show(ui, |ui| {
            ui.label("Enabled");
            ui.monospace(format!("{} / DAC {}", channel.enabled, channel.dac_enabled));
            ui.label("LFSR");
            ui.monospace(format!("{:04X}", channel.lfsr));
            ui.end_row();

            ui.label("Timer / Div");
            ui.monospace(format!("{} / {}", channel.freq_timer, channel.divisor_code));
            ui.label("Shift / Width");
            ui.monospace(format!("{} / {}", channel.shift, channel.width_mode));
            ui.end_row();

            ui.label("Length");
            ui.monospace(format!(
                "{} ({})",
                channel.length_timer, channel.length_enabled
            ));
            ui.label("Envelope");
            ui.monospace(format!(
                "vol:{} period:{} dir:{}",
                channel.volume, channel.envelope_period, channel.envelope_direction
            ));
            ui.end_row();
        });
}
