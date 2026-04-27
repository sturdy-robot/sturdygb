use super::EmuApp;
use eframe::egui;
use sturdygb_core::joypad::JoypadButton;
use sturdygb_core::gb::ModelSelection;

impl EmuApp {
    pub(super) fn show_options_window(&mut self, ctx: &egui::Context) -> bool {
        let mut is_open = self.show_options;
        let mut reload_requested = false;
        if is_open {
            egui::Window::new("Emulator Options")
                .collapsible(false)
                .resizable(false)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    egui::Grid::new("options_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Scale Mode:");
                            egui::ComboBox::from_id_salt("scale_combo")
                                .selected_text(format!("{:?}", self.config.scale))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        super::config::ScaleMode::Integer(1.0),
                                        "1x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        super::config::ScaleMode::Integer(2.0),
                                        "2x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        super::config::ScaleMode::Integer(3.0),
                                        "3x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        super::config::ScaleMode::Integer(4.0),
                                        "4x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        super::config::ScaleMode::Integer(5.0),
                                        "5x",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        super::config::ScaleMode::Integer(6.0),
                                        "6x",
                                    );
                                    ui.separator();
                                    ui.selectable_value(
                                        &mut self.config.scale,
                                        super::config::ScaleMode::Stretch,
                                        "Stretch (Fit window)",
                                    );
                                });
                            ui.end_row();

                            ui.label("Game Boy Model:");
                            let previous_model = self.config.model_selection;
                            egui::ComboBox::from_id_salt("model_combo")
                                .selected_text(self.config.model_selection.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.config.model_selection,
                                        ModelSelection::Auto,
                                        "Auto",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.model_selection,
                                        ModelSelection::Dmg,
                                        "DMG",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.model_selection,
                                        ModelSelection::Cgb,
                                        "CGB",
                                    );
                                });
                            reload_requested |= previous_model != self.config.model_selection;
                            ui.end_row();

                            ui.label("Color Palette:");
                            egui::ComboBox::from_id_salt("palette_combo")
                                .selected_text(format!("{:?}", self.config.palette))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.config.palette,
                                        super::config::Palette::Greyscale,
                                        "Greyscale",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.palette,
                                        super::config::Palette::ClassicGreen,
                                        "Classic Green",
                                    );
                                    ui.selectable_value(
                                        &mut self.config.palette,
                                        super::config::Palette::Pocket,
                                        "Pocket (Grey/Green)",
                                    );
                                });
                            ui.end_row();
                        });

                    if let Some(state) = self.state.as_ref() {
                        ui.label(format!(
                            "Current session model: {:?} ({:?})",
                            state.gb.gb_type, state.gb.gb_mode
                        ));
                    }

                    ui.separator();
                    ui.label("Keybindings:");

                    egui::Grid::new("keybinds_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .show(ui, |ui| {
                            let buttons = [
                                JoypadButton::Up,
                                JoypadButton::Down,
                                JoypadButton::Left,
                                JoypadButton::Right,
                                JoypadButton::A,
                                JoypadButton::B,
                                JoypadButton::Start,
                                JoypadButton::Select,
                            ];

                            for btn in buttons {
                                ui.label(format!("{:?}", btn));

                                let current_key = self
                                    .config
                                    .keybinds
                                    .get(&btn)
                                    .copied()
                                    .unwrap_or(egui::Key::Escape);

                                let btn_text = if ctx.memory(|mem| {
                                    mem.data
                                        .get_temp::<JoypadButton>(egui::Id::new("listening_bind"))
                                }) == Some(btn)
                                {
                                    "Press any key...".to_string()
                                } else {
                                    format!("{:?}", current_key)
                                };

                                let response = ui.button(btn_text);

                                if response.clicked() {
                                    ctx.memory_mut(|mem| {
                                        mem.data.insert_temp(egui::Id::new("listening_bind"), btn)
                                    });
                                }

                                ui.end_row();
                            }
                        });

                    if let Some(btn) = ctx.memory(|mem| {
                        mem.data
                            .get_temp::<JoypadButton>(egui::Id::new("listening_bind"))
                    }) {
                        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                            ctx.memory_mut(|mem| {
                                mem.data
                                    .remove::<JoypadButton>(egui::Id::new("listening_bind"))
                            });
                        } else if let Some(key) = ctx.input(|i| {
                            i.events.iter().find_map(|e| {
                                if let egui::Event::Key {
                                    key, pressed: true, ..
                                } = e
                                {
                                    Some(*key)
                                } else {
                                    None
                                }
                            })
                        }) {
                            self.config.keybinds.insert(btn, key);
                            ctx.memory_mut(|mem| {
                                mem.data
                                    .remove::<JoypadButton>(egui::Id::new("listening_bind"))
                            });
                        }
                    }
                });
        }
        self.show_options = is_open;
            reload_requested
    }
}
