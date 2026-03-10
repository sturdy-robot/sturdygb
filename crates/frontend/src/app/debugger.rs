use super::{EmuApp, State};
use crate::debug_views::{
    build_bg_map_image, build_oam_image, build_vram_image, find_watchpoint_hit,
    format_byte_list, format_hex_dump, parse_hex_u16, parse_hex_usize,
};
use eframe::egui;
use std::collections::BTreeSet;
use sturdygb_core::gb::{DebugSnapshot, DisassemblyLine, MemoryWriteEvent};

struct DebuggerWindowData {
    snapshot: DebugSnapshot,
    memory_dump: String,
    disassembly: Vec<DisassemblyLine>,
    last_writes: Vec<MemoryWriteEvent>,
}

pub(super) struct DebuggerUiState {
    pub show_debugger: bool,
    pub show_vram_viewer: bool,
    pub show_bg_map_viewer: bool,
    pub show_oam_viewer: bool,
    pub breakpoints: BTreeSet<u16>,
    pub watchpoints: BTreeSet<u16>,
    pub breakpoint_input: String,
    pub watchpoint_input: String,
    pub memory_start_input: String,
    pub memory_len_input: String,
    pub disassembly_start_input: String,
    pub disassembly_count_input: String,
    pub last_stop_reason: Option<String>,
    step_requested: bool,
    ignore_breakpoint_once: Option<u16>,
    vram_texture: Option<egui::TextureHandle>,
    bg_map_texture: Option<egui::TextureHandle>,
    oam_texture: Option<egui::TextureHandle>,
    pub selected_vram_bank: usize,
    pub selected_bg_map: usize,
}

impl DebuggerUiState {
    pub(super) fn new() -> Self {
        Self {
            show_debugger: false,
            show_vram_viewer: false,
            show_bg_map_viewer: false,
            show_oam_viewer: false,
            breakpoints: BTreeSet::new(),
            watchpoints: BTreeSet::new(),
            breakpoint_input: "0100".to_string(),
            watchpoint_input: "C000".to_string(),
            memory_start_input: "C000".to_string(),
            memory_len_input: "0080".to_string(),
            disassembly_start_input: "0100".to_string(),
            disassembly_count_input: "0020".to_string(),
            last_stop_reason: None,
            step_requested: false,
            ignore_breakpoint_once: None,
            vram_texture: None,
            bg_map_texture: None,
            oam_texture: None,
            selected_vram_bank: 0,
            selected_bg_map: 0,
        }
    }

    pub(super) fn reset_runtime(&mut self) {
        self.last_stop_reason = None;
        self.step_requested = false;
        self.ignore_breakpoint_once = None;
        self.vram_texture = None;
        self.bg_map_texture = None;
        self.oam_texture = None;
        self.selected_vram_bank = 0;
        self.selected_bg_map = 0;
    }

    pub(super) fn prepare_resume(&mut self, state: Option<&State>) {
        self.last_stop_reason = None;
        self.step_requested = false;
        self.ignore_breakpoint_once = state.map(|state| state.gb.current_pc());
    }

    pub(super) fn request_step(&mut self, state: Option<&State>) {
        self.last_stop_reason = None;
        self.step_requested = true;
        self.ignore_breakpoint_once = state.map(|state| state.gb.current_pc());
    }

    fn needs_instruction_loop(&self) -> bool {
        self.step_requested || !self.breakpoints.is_empty() || !self.watchpoints.is_empty()
    }

    pub(super) fn run_until_debug_or_frame(&mut self, state: &mut State) -> bool {
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
                self.step_requested = false;
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
                self.step_requested = false;
                self.ignore_breakpoint_once = None;
                return true;
            }

            if self.step_requested {
                self.last_stop_reason = Some(format!(
                    "Step completed at {:04X}",
                    state.gb.current_pc()
                ));
                self.show_debugger = true;
                self.step_requested = false;
                self.ignore_breakpoint_once = None;
                return true;
            }

            if state.gb.frame_ready() {
                self.ignore_breakpoint_once = None;
                return false;
            }
        }
    }
}

impl EmuApp {
    pub(super) fn show_debugger_windows(&mut self, ctx: &egui::Context) {
        self.show_debugger_window(ctx);
        self.show_vram_window(ctx);
        self.show_bg_map_window(ctx);
        self.show_oam_window(ctx);
    }

    fn show_debugger_window(&mut self, ctx: &egui::Context) {
        let mut open = self.debugger.show_debugger;
        if !open {
            return;
        }

        let data = if let Some(state) = self.state.as_mut() {
            let snapshot = state.gb.debug_snapshot();
            let memory_start = parse_hex_u16(&self.debugger.memory_start_input, snapshot.pc);
            let memory_len = parse_hex_usize(&self.debugger.memory_len_input, 0x80).clamp(0x10, 0x200);
            let disassembly_start = parse_hex_u16(&self.debugger.disassembly_start_input, snapshot.pc);
            let disassembly_count =
                parse_hex_usize(&self.debugger.disassembly_count_input, 0x20).clamp(1, 0x80);
            Some(DebuggerWindowData {
                memory_dump: format_hex_dump(
                    memory_start,
                    &state.gb.read_memory_range(memory_start, memory_len),
                ),
                disassembly: state.gb.disassemble_range(disassembly_start, disassembly_count),
                last_writes: state.gb.last_memory_writes().to_vec(),
                snapshot,
            })
        } else {
            None
        };

        let has_state = self.state.is_some();
        egui::Window::new("Debugger")
            .open(&mut open)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(
                            has_state,
                            egui::Button::new(if self.paused { "Resume" } else { "Pause" }),
                        )
                        .clicked()
                    {
                        if self.paused {
                            self.debugger.prepare_resume(self.state.as_ref());
                            self.paused = false;
                        } else {
                            self.paused = true;
                        }
                    }
                    if ui
                        .add_enabled(has_state, egui::Button::new("Step"))
                        .clicked()
                    {
                        self.debugger.request_step(self.state.as_ref());
                        self.debugger.show_debugger = true;
                        self.paused = false;
                    }
                    if ui.button("VRAM").clicked() {
                        self.debugger.show_vram_viewer = true;
                    }
                    if ui.button("BG Map").clicked() {
                        self.debugger.show_bg_map_viewer = true;
                    }
                    if ui.button("OAM").clicked() {
                        self.debugger.show_oam_viewer = true;
                    }
                });

                let status = self
                    .debugger
                    .last_stop_reason
                    .clone()
                    .unwrap_or_else(|| if self.paused { "Paused" } else { "Running" }.to_string());
                ui.label(status);

                if let Some(data) = data {
                    let flags = data.snapshot.af as u8;
                    ui.separator();
                    egui::Grid::new("debugger_registers")
                        .num_columns(4)
                        .spacing([16.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("AF");
                            ui.monospace(format!("{:04X}", data.snapshot.af));
                            ui.label("BC");
                            ui.monospace(format!("{:04X}", data.snapshot.bc));
                            ui.end_row();
                            ui.label("DE");
                            ui.monospace(format!("{:04X}", data.snapshot.de));
                            ui.label("HL");
                            ui.monospace(format!("{:04X}", data.snapshot.hl));
                            ui.end_row();
                            ui.label("SP");
                            ui.monospace(format!("{:04X}", data.snapshot.sp));
                            ui.label("PC");
                            ui.monospace(format!("{:04X}", data.snapshot.pc));
                            ui.end_row();
                            ui.label("OP");
                            ui.monospace(format!("{:02X}", data.snapshot.opcode));
                            ui.label("LY");
                            ui.monospace(format!("{:02X}", data.snapshot.ly));
                            ui.end_row();
                            ui.label("STAT");
                            ui.monospace(format!("{:02X}", data.snapshot.stat));
                            ui.label("IME");
                            ui.monospace(if data.snapshot.interrupt_master { "1" } else { "0" });
                            ui.end_row();
                            ui.label("Flags");
                            ui.monospace(format!(
                                "Z:{} N:{} H:{} C:{}",
                                (flags >> 7) & 1,
                                (flags >> 6) & 1,
                                (flags >> 5) & 1,
                                (flags >> 4) & 1
                            ));
                            ui.label("State");
                            ui.monospace(format!(
                                "halt:{} stop:{} cyc:{}",
                                data.snapshot.is_halted,
                                data.snapshot.is_stopped,
                                data.snapshot.pending_cycles
                            ));
                            ui.end_row();
                        });

                    ui.separator();
                    ui.label(egui::RichText::new(&data.snapshot.disassembly).monospace());
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

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Breakpoint");
                        ui.text_edit_singleline(&mut self.debugger.breakpoint_input);
                        if ui.button("Add").clicked() {
                            let address =
                                parse_hex_u16(&self.debugger.breakpoint_input, data.snapshot.pc);
                            self.debugger.breakpoints.insert(address);
                            self.debugger.breakpoint_input = format!("{:04X}", address);
                        }
                        if ui.button("PC").clicked() {
                            self.debugger.breakpoints.insert(data.snapshot.pc);
                            self.debugger.breakpoint_input = format!("{:04X}", data.snapshot.pc);
                        }
                    });

                    let breakpoints: Vec<u16> = self.debugger.breakpoints.iter().copied().collect();
                    if !breakpoints.is_empty() {
                        for address in breakpoints {
                            ui.horizontal(|ui| {
                                ui.monospace(format!("{:04X}", address));
                                if ui.button("Remove").clicked() {
                                    self.debugger.breakpoints.remove(&address);
                                }
                            });
                        }
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Watchpoint");
                        ui.text_edit_singleline(&mut self.debugger.watchpoint_input);
                        if ui.button("Add").clicked() {
                            let address =
                                parse_hex_u16(&self.debugger.watchpoint_input, 0xC000);
                            self.debugger.watchpoints.insert(address);
                            self.debugger.watchpoint_input = format!("{:04X}", address);
                        }
                    });

                    let watchpoints: Vec<u16> = self.debugger.watchpoints.iter().copied().collect();
                    if !watchpoints.is_empty() {
                        for address in watchpoints {
                            ui.horizontal(|ui| {
                                ui.monospace(format!("{:04X}", address));
                                if ui.button("Remove").clicked() {
                                    self.debugger.watchpoints.remove(&address);
                                }
                            });
                        }
                    }

                    if !data.last_writes.is_empty() {
                        ui.separator();
                        ui.label("Recent writes");
                        for write in data.last_writes.iter().take(8) {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:04X} = {:02X}",
                                    write.address, write.value
                                ))
                                .monospace(),
                            );
                        }
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Disassembly");
                        ui.text_edit_singleline(&mut self.debugger.disassembly_start_input);
                        ui.label("Count");
                        ui.text_edit_singleline(&mut self.debugger.disassembly_count_input);
                        if ui.button("PC").clicked() {
                            self.debugger.disassembly_start_input = format!("{:04X}", data.snapshot.pc);
                        }
                    });
                    egui::ScrollArea::vertical()
                        .max_height(220.0)
                        .show(ui, |ui| {
                            for line in &data.disassembly {
                                let marker = if line.address == data.snapshot.pc { '▶' } else { ' ' };
                                let breakpoint = if self.debugger.breakpoints.contains(&line.address) {
                                    '●'
                                } else {
                                    ' '
                                };
                                let text = format!(
                                    "{}{} {:04X}: {:<11} {}",
                                    marker,
                                    breakpoint,
                                    line.address,
                                    format_byte_list(&line.bytes),
                                    line.text
                                );
                                let rich = if line.address == data.snapshot.pc {
                                    egui::RichText::new(text).monospace().strong()
                                } else {
                                    egui::RichText::new(text).monospace()
                                };
                                ui.label(rich);
                            }
                        });

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Memory");
                        ui.text_edit_singleline(&mut self.debugger.memory_start_input);
                        ui.label("Len");
                        ui.text_edit_singleline(&mut self.debugger.memory_len_input);
                        if ui.button("PC").clicked() {
                            self.debugger.memory_start_input = format!("{:04X}", data.snapshot.pc);
                        }
                    });
                    egui::ScrollArea::vertical()
                        .max_height(180.0)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(&data.memory_dump).monospace());
                        });
                } else {
                    ui.separator();
                    ui.label("Load a ROM to inspect debugger state.");
                }
            });
        self.debugger.show_debugger = open;
    }

    fn show_vram_window(&mut self, ctx: &egui::Context) {
        let mut open = self.debugger.show_vram_viewer;
        if !open {
            return;
        }

        let bank_count = self
            .state
            .as_ref()
            .map(|state| state.gb.vram_bank_count())
            .unwrap_or(0);
        if bank_count > 0 {
            self.debugger.selected_vram_bank = self
                .debugger
                .selected_vram_bank
                .min(bank_count.saturating_sub(1));
        }

        let image_data = self.state.as_ref().map(|state| {
            build_vram_image(state.gb.vram_tile_data(self.debugger.selected_vram_bank))
        });

        let texture = image_data.as_ref().map(|(image, _, _)| {
            update_texture(&mut self.debugger.vram_texture, ctx, "debug_vram", image.clone())
        });

        egui::Window::new("VRAM Viewer")
            .open(&mut open)
            .vscroll(true)
            .show(ctx, |ui| {
                if let (Some(texture), Some((_, width, height))) = (texture.as_ref(), image_data.as_ref()) {
                    ui.horizontal(|ui| {
                        ui.label("Bank");
                        for bank in 0..bank_count {
                            ui.selectable_value(
                                &mut self.debugger.selected_vram_bank,
                                bank,
                                bank.to_string(),
                            );
                        }
                    });
                    ui.add(
                        egui::Image::new(texture).fit_to_exact_size(egui::vec2(
                            (*width as f32) * 2.0,
                            (*height as f32) * 2.0,
                        )),
                    );
                } else {
                    ui.label("Load a ROM to inspect VRAM.");
                }
            });

        self.debugger.show_vram_viewer = open;
    }

    fn show_bg_map_window(&mut self, ctx: &egui::Context) {
        let mut open = self.debugger.show_bg_map_viewer;
        if !open {
            return;
        }

        let bank_count = self
            .state
            .as_ref()
            .map(|state| state.gb.vram_bank_count())
            .unwrap_or(0);

        let image_data = self.state.as_ref().map(|state| {
            let tile_bank0 = state.gb.vram_tile_data(0);
            let tile_bank1 = if bank_count > 1 {
                Some(state.gb.vram_tile_data(1))
            } else {
                None
            };
            let tile_map = state.gb.vram_map_data(0, self.debugger.selected_bg_map.min(1));
            let attr_map = if bank_count > 1 {
                Some(state.gb.vram_map_data(1, self.debugger.selected_bg_map.min(1)))
            } else {
                None
            };
            let signed_mode = state.gb.read_byte(0xFF40) & 0x10 == 0;
            (
                signed_mode,
                build_bg_map_image(
                    tile_bank0,
                    tile_bank1,
                    tile_map,
                    attr_map,
                    signed_mode,
                ),
            )
        });

        let texture = image_data.as_ref().map(|(_, (image, _, _))| {
            update_texture(&mut self.debugger.bg_map_texture, ctx, "debug_bg_map", image.clone())
        });

        egui::Window::new("Background Map Viewer")
            .open(&mut open)
            .vscroll(true)
            .show(ctx, |ui| {
                if let (Some(texture), Some((signed_mode, (_, width, height)))) =
                    (texture.as_ref(), image_data.as_ref())
                {
                    ui.horizontal(|ui| {
                        ui.label("Map");
                        ui.selectable_value(&mut self.debugger.selected_bg_map, 0, "9800");
                        ui.selectable_value(&mut self.debugger.selected_bg_map, 1, "9C00");
                    });
                    ui.label(if *signed_mode {
                        "Tile data mode: 8800 signed"
                    } else {
                        "Tile data mode: 8000 unsigned"
                    });
                    ui.add(
                        egui::Image::new(texture).fit_to_exact_size(egui::vec2(
                            (*width as f32) * 2.0,
                            (*height as f32) * 2.0,
                        )),
                    );
                } else {
                    ui.label("Load a ROM to inspect the background map.");
                }
            });

        self.debugger.show_bg_map_viewer = open;
    }

    fn show_oam_window(&mut self, ctx: &egui::Context) {
        let mut open = self.debugger.show_oam_viewer;
        if !open {
            return;
        }

        let bank_count = self
            .state
            .as_ref()
            .map(|state| state.gb.vram_bank_count())
            .unwrap_or(0);

        let image_data = self.state.as_ref().map(|state| {
            let tile_bank0 = state.gb.vram_tile_data(0);
            let tile_bank1 = if bank_count > 1 {
                Some(state.gb.vram_tile_data(1))
            } else {
                None
            };
            let sprite_height = state.gb.sprite_height() as usize;
            let sprites = state.gb.oam_sprites();
            let image = build_oam_image(tile_bank0, tile_bank1, &sprites, sprite_height);
            (sprite_height, sprites, image)
        });

        let texture = image_data.as_ref().map(|(_, _, (image, _, _))| {
            update_texture(&mut self.debugger.oam_texture, ctx, "debug_oam", image.clone())
        });

        egui::Window::new("OAM Viewer")
            .open(&mut open)
            .vscroll(true)
            .show(ctx, |ui| {
                if let (Some(texture), Some((sprite_height, sprites, (_, width, height)))) =
                    (texture.as_ref(), image_data.as_ref())
                {
                    ui.label(format!("Sprite height: {}", sprite_height));
                    ui.add(
                        egui::Image::new(texture).fit_to_exact_size(egui::vec2(
                            (*width as f32) * 2.0,
                            (*height as f32) * 2.0,
                        )),
                    );
                    ui.separator();
                    egui::ScrollArea::vertical()
                        .max_height(220.0)
                        .show(ui, |ui| {
                            for sprite in sprites.iter().take(40) {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "#{:02} x:{:03} y:{:03} tile:{:02X} attr:{:02X}",
                                        sprite.index,
                                        sprite.x,
                                        sprite.y,
                                        sprite.tile_number,
                                        sprite.attributes
                                    ))
                                    .monospace(),
                                );
                            }
                        });
                } else {
                    ui.label("Load a ROM to inspect OAM.");
                }
            });

        self.debugger.show_oam_viewer = open;
    }
}

fn update_texture(
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
