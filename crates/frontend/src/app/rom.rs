use super::audio::setup_audio;
use super::state::LoadedGameState;
use super::{EmuApp, GB_H, GB_W};
use sturdygb_core::prelude::GbInstance;

impl EmuApp {
    pub(super) fn load_rom_file(&mut self, path: &str, storage: Option<&dyn eframe::Storage>) {
        if let Ok(bytes) = std::fs::read(path) {
            self.load_rom_bytes(
                bytes,
                Some(std::path::PathBuf::from(path).with_extension("sav")),
                storage,
            );
        } else {
            self.runtime.error_msg = Some(format!("Could not read file {path}"));
        }
    }

    pub(in crate::app) fn reset_loaded_rom(&mut self, storage: Option<&dyn eframe::Storage>) {
        if let Some((rom_bytes, save_path)) = self
            .runtime
            .loaded_game
            .as_ref()
            .map(|state| (state.rom_bytes.clone(), state.save_path.clone()))
        {
            self.load_rom_bytes(rom_bytes, save_path, storage);
        }
    }

    pub(super) fn load_rom_bytes(
        &mut self,
        mut bytes: Vec<u8>,
        save_path: Option<std::path::PathBuf>,
        _storage: Option<&dyn eframe::Storage>,
    ) {
        if let Some(extracted) = extract_rom_from_bytes(&bytes) {
            bytes = extracted;
        }

        let mut title = "Unknown Title".to_string();
        if let Ok(header) = sturdygb_core::cartridge::CartridgeHeader::new(&bytes) {
            title = header.title;
        }

        match GbInstance::build_from_bytes_with_model(
            bytes.clone(),
            save_path.clone(),
            self.config.model_selection,
        ) {
            Ok(mut gb) => {
                #[cfg(target_arch = "wasm32")]
                if let Some(storage) = _storage {
                    if let Some(saved) =
                        eframe::get_value::<Vec<u8>>(storage, &format!("sturdygb_sram_{title}"))
                    {
                        gb.set_battery_ram(&saved);
                    }
                }

                setup_audio(&mut gb);
                self.runtime.loaded_game = Some(LoadedGameState {
                    gb,
                    rgba: vec![0; GB_W * GB_H * 4],
                    leftover_audio: Vec::new(),
                    title,
                    rom_bytes: bytes,
                    save_path,
                });
                self.runtime.texture = None;
                self.runtime.paused = false;
                self.debugger.reset_runtime();
                self.runtime.error_msg = None;
                self.runtime.frames_rendered = 0;
                self.runtime.last_fps_update = instant::Instant::now();
            }
            Err(e) => {
                self.runtime.error_msg = Some(format!("Failed to load ROM:\n{e}"));
            }
        }
    }
}

fn extract_rom_from_bytes(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() >= 4 && bytes[0..4] == [0x50, 0x4b, 0x03, 0x04] {
        let cursor = std::io::Cursor::new(bytes);
        if let Ok(mut archive) = zip::ZipArchive::new(cursor) {
            for i in 0..archive.len() {
                if let Ok(mut file) = archive.by_index(i) {
                    let name = file.name().to_lowercase();
                    if name.ends_with(".gb") || name.ends_with(".gbc") {
                        use std::io::Read;
                        let mut extracted = Vec::new();
                        if file.read_to_end(&mut extracted).is_ok() {
                            return Some(extracted);
                        }
                    }
                }
            }
        }
    }
    None
}
