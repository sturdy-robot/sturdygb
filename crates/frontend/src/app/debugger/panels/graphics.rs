use super::common::{require_debugger_data, update_texture};
use super::super::types::DebuggerWindowData;
use crate::app::EmuApp;
use crate::debug_views::{build_bg_map_image, build_oam_image, build_vram_image};
use eframe::egui;

impl EmuApp {
    pub(super) fn render_ppu_panel(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<&DebuggerWindowData>,
    ) {
        let Some(data) = require_debugger_data(ui, data, "Load a ROM to inspect PPU state.") else {
            return;
        };

        let ppu = &data.ppu_snapshot;
        egui::Grid::new("debugger_ppu_state")
            .num_columns(2)
            .spacing([16.0, 4.0])
            .show(ui, |ui| {
                ui.label("Mode");
                ui.monospace(format!("{:?}", ppu.mode));
                ui.end_row();

                ui.label("Frame Ready");
                ui.monospace(if ppu.frame_ready { "1" } else { "0" });
                ui.end_row();

                ui.label("LCDC/STAT");
                ui.monospace(format!("{:02X}/{:02X}", ppu.lcdc, ppu.stat));
                ui.end_row();

                ui.label("LY/LYC");
                ui.monospace(format!("{:02X}/{:02X}", ppu.ly, ppu.lyc));
                ui.end_row();

                ui.label("SCX/SCY");
                ui.monospace(format!("{:02X}/{:02X}", ppu.scx, ppu.scy));
                ui.end_row();

                ui.label("WX/WY");
                ui.monospace(format!("{:02X}/{:02X}", ppu.wx, ppu.wy));
                ui.end_row();

                ui.label("BGP");
                ui.monospace(format!("{:02X}", ppu.bgp));
                ui.end_row();

                ui.label("OBP0/OBP1");
                ui.monospace(format!("{:02X}/{:02X}", ppu.obp0, ppu.obp1));
                ui.end_row();

                ui.label("VBK/BCPS");
                ui.monospace(format!("{:02X}/{:02X}", ppu.vbk, ppu.bcps));
                ui.end_row();

                ui.label("OCPS/SVBK");
                ui.monospace(format!("{:02X}/{:02X}", ppu.ocps, ppu.svbk));
                ui.end_row();

                ui.label("Mode Clock");
                ui.monospace(ppu.mode_clock.to_string());
                ui.end_row();

                ui.label("Line Clock");
                ui.monospace(ppu.line_clock.to_string());
                ui.end_row();

                ui.label("Fetch X / Visible X");
                ui.monospace(format!("{} / {}", ppu.fetch_x, ppu.visible_x));
                ui.end_row();

                ui.label("Window Line");
                ui.monospace(ppu.window_line_counter.to_string());
                ui.end_row();

                ui.label("Window");
                ui.monospace(format!(
                    "triggered:{} line:{}",
                    ppu.window_triggered, ppu.window_rendering_this_line
                ));
                ui.end_row();
                
                ui.label("OAM Scan / Sprites");
                ui.monospace(format!("{} / {}", ppu.oam_scan_index, ppu.sprites_on_line));
                ui.end_row();
            });

        ui.separator();
        ui.label(
            egui::RichText::new(format!(
                "DMA active:{} src:{:02X}00 byte:{:02X}",
                ppu.dma_active, ppu.dma_source_high, ppu.dma_byte
            ))
            .monospace(),
        );
        ui.label(
            egui::RichText::new(format!(
                "HDMA active:{} hblank:{} src:{:04X} dst:{:04X}",
                ppu.hdma_active, ppu.hdma_hblank_mode, ppu.hdma_source, ppu.hdma_destination
            ))
            .monospace(),
        );
    }

    pub(super) fn render_vram_panel(&mut self, ui: &mut egui::Ui) {
        let bank_count = self
            .runtime
            .loaded_game
            .as_ref()
            .map(|state| state.gb.vram_bank_count())
            .unwrap_or(0);
        if bank_count > 0 {
            self.debugger.selected_vram_bank = self
                .debugger
                .selected_vram_bank
                .min(bank_count.saturating_sub(1));
        }

        let image_data = self.loaded_game().map(|state| {
            build_vram_image(state.gb.vram_tile_data(self.debugger.selected_vram_bank))
        });

        let texture = image_data.as_ref().map(|(image, _, _)| {
            update_texture(
                &mut self.debugger.vram_texture,
                ui.ctx(),
                "debug_vram",
                image.clone(),
            )
        });

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
                egui::Image::new(texture)
                    .fit_to_exact_size(egui::vec2((*width as f32) * 2.0, (*height as f32) * 2.0)),
            );
        } else {
            ui.label("Load a ROM to inspect VRAM.");
        }
    }

    pub(super) fn render_bg_map_panel(&mut self, ui: &mut egui::Ui) {
        let bank_count = self
            .runtime
            .loaded_game
            .as_ref()
            .map(|state| state.gb.vram_bank_count())
            .unwrap_or(0);

        let image_data = self.loaded_game().map(|state| {
            let tile_bank0 = state.gb.vram_tile_data(0);
            let tile_bank1 = if bank_count > 1 {
                Some(state.gb.vram_tile_data(1))
            } else {
                None
            };
            let tile_map = state
                .gb
                .vram_map_data(0, self.debugger.selected_bg_map.min(1));
            let attr_map = if bank_count > 1 {
                Some(state.gb.vram_map_data(1, self.debugger.selected_bg_map.min(1)))
            } else {
                None
            };
            let signed_mode = state.gb.read_byte(0xFF40) & 0x10 == 0;
            (
                signed_mode,
                build_bg_map_image(tile_bank0, tile_bank1, tile_map, attr_map, signed_mode),
            )
        });

        let texture = image_data.as_ref().map(|(_, (image, _, _))| {
            update_texture(
                &mut self.debugger.bg_map_texture,
                ui.ctx(),
                "debug_bg_map",
                image.clone(),
            )
        });

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
                egui::Image::new(texture)
                    .fit_to_exact_size(egui::vec2((*width as f32) * 2.0, (*height as f32) * 2.0)),
            );
        } else {
            ui.label("Load a ROM to inspect the background map.");
        }
    }

    pub(super) fn render_oam_panel(&mut self, ui: &mut egui::Ui) {
        let bank_count = self
            .runtime
            .loaded_game
            .as_ref()
            .map(|state| state.gb.vram_bank_count())
            .unwrap_or(0);

        let image_data = self.loaded_game().map(|state| {
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
            update_texture(
                &mut self.debugger.oam_texture,
                ui.ctx(),
                "debug_oam",
                image.clone(),
            )
        });

        if let (Some(texture), Some((sprite_height, sprites, (_, width, height)))) =
            (texture.as_ref(), image_data.as_ref())
        {
            ui.label(format!("Sprite height: {}", sprite_height));
            ui.add(
                egui::Image::new(texture)
                    .fit_to_exact_size(egui::vec2((*width as f32) * 2.0, (*height as f32) * 2.0)),
            );
            ui.separator();
            egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
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
    }
}