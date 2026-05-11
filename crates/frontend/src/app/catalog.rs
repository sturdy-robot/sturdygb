#[cfg(not(target_arch = "wasm32"))]
use super::config::GameEntry;
use super::EmuApp;

impl EmuApp {
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn load_directory(&mut self, path: std::path::PathBuf) {
        if !self.config.rom_directories.contains(&path) {
            self.config.rom_directories.push(path);
        }
        self.reload_all_directories();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn reload_all_directories(&mut self) {
        self.catalog.game_list.clear();
        if self.config.rom_directories.is_empty() {
            return;
        }
        self.catalog.loading_directory = true;

        let (tx, rx) = std::sync::mpsc::channel();
        self.catalog.dir_load_receiver = Some(rx);
        let recursive = self.catalog.recursive_search;
        let dirs = self.config.rom_directories.clone();

        std::thread::spawn(move || {
            for path in dirs {
                let walker = walkdir::WalkDir::new(path);
                let walker = if recursive {
                    walker
                } else {
                    walker.max_depth(1)
                };

                for entry in walker.into_iter().filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            let ext = ext.to_lowercase();
                            if ext == "gb" || ext == "gbc" || ext == "zip" {
                                let filename = path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                let mut title = "Unknown Title".to_string();
                                let mut company = "Unknown Company".to_string();

                                if ext == "gb" || ext == "gbc" {
                                    if let Ok(mut f) = std::fs::File::open(&path) {
                                        use std::io::Read;
                                        let mut header_bytes = vec![0; 0x150];
                                        if f.read_exact(&mut header_bytes).is_ok() {
                                            if let Ok(header) =
                                                sturdygb_core::cartridge::CartridgeHeader::new(
                                                    &header_bytes,
                                                )
                                            {
                                                title = header.title;
                                                company = header.company;
                                            }
                                        }
                                    }
                                } else if ext == "zip" {
                                    if let Ok(f) = std::fs::File::open(&path) {
                                        if let Ok(mut archive) = zip::ZipArchive::new(f) {
                                            for i in 0..archive.len() {
                                                if let Ok(mut inner) = archive.by_index(i) {
                                                    let inner_name = inner.name().to_lowercase();
                                                    if inner_name.ends_with(".gb")
                                                        || inner_name.ends_with(".gbc")
                                                    {
                                                        use std::io::Read;
                                                        let mut header_bytes = vec![0; 0x150];
                                                        if inner
                                                            .read_exact(&mut header_bytes)
                                                            .is_ok()
                                                        {
                                                            if let Ok(header) = sturdygb_core::cartridge::CartridgeHeader::new(&header_bytes) {
                                                                title = header.title;
                                                                company = header.company;
                                                            }
                                                        }
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if tx
                                    .send(GameEntry {
                                        path: path.to_path_buf(),
                                        filename,
                                        title,
                                        company,
                                    })
                                    .is_err()
                                {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
