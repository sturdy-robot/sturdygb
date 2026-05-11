#[cfg(target_arch = "wasm32")]
use std::time::Duration;

use super::help::HelpUiState;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
use super::config::{GameEntry, SortMethod};

#[cfg(target_arch = "wasm32")]
pub(super) struct PendingRomLoad {
    pub(super) rom_bytes: Vec<u8>,
    pub(super) imported_save: Option<Vec<u8>>,
    pub(super) status_update: Option<StatusUpdate>,
}

#[cfg(target_arch = "wasm32")]
pub(super) enum WasmUiEvent {
    RomLoad(Result<PendingRomLoad, String>),
    SaveImport(Result<Vec<u8>, String>),
}

pub(in crate::app) struct LoadedGameState {
    pub(super) gb: sturdygb_core::gb::Gb,
    pub(super) rgba: Vec<u8>,
    pub(super) leftover_audio: Vec<[f32; 2]>,
    pub(super) title: String,
    pub(super) rom_bytes: Vec<u8>,
    pub(super) save_path: Option<std::path::PathBuf>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy)]
pub(super) enum StatusLevel {
    Success,
    Error,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub(super) struct StatusUpdate {
    pub(super) level: StatusLevel,
    pub(super) text: String,
}

#[cfg(target_arch = "wasm32")]
pub(super) struct ActiveStatusMessage {
    pub(super) level: StatusLevel,
    pub(super) text: String,
    pub(super) shown_at: instant::Instant,
}

#[cfg(target_arch = "wasm32")]
impl StatusUpdate {
    pub(super) fn success(text: impl Into<String>) -> Self {
        Self {
            level: StatusLevel::Success,
            text: text.into(),
        }
    }

    pub(super) fn error(text: impl Into<String>) -> Self {
        Self {
            level: StatusLevel::Error,
            text: text.into(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ActiveStatusMessage {
    pub(super) const DISPLAY_FOR: Duration = Duration::from_secs(4);

    fn new(update: StatusUpdate) -> Self {
        Self {
            level: update.level,
            text: update.text,
            shown_at: instant::Instant::now(),
        }
    }
}

pub(super) struct RuntimeState {
    pub(super) loaded_game: Option<LoadedGameState>,
    pub(super) texture: Option<egui::TextureHandle>,
    pub(super) error_msg: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub(super) status_msg: Option<ActiveStatusMessage>,
    pub(super) paused: bool,
    #[cfg(target_arch = "wasm32")]
    pub(super) async_event_channel: (
        std::sync::mpsc::Sender<WasmUiEvent>,
        std::sync::mpsc::Receiver<WasmUiEvent>,
    ),
    pub(super) frames_rendered: usize,
    pub(super) last_fps_update: instant::Instant,
    pub(super) current_fps: usize,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            loaded_game: None,
            texture: None,
            error_msg: None,
            #[cfg(target_arch = "wasm32")]
            status_msg: None,
            paused: false,
            #[cfg(target_arch = "wasm32")]
            async_event_channel: std::sync::mpsc::channel(),
            frames_rendered: 0,
            last_fps_update: instant::Instant::now(),
            current_fps: 0,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) struct CatalogState {
    pub(super) game_list: Vec<GameEntry>,
    pub(super) recursive_search: bool,
    pub(super) search_query: String,
    pub(super) sort_by: SortMethod,
    pub(super) sort_ascending: bool,
    pub(super) loading_directory: bool,
    pub(super) dir_load_receiver: Option<std::sync::mpsc::Receiver<GameEntry>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for CatalogState {
    fn default() -> Self {
        Self {
            game_list: Vec::new(),
            recursive_search: false,
            search_query: String::new(),
            sort_by: SortMethod::Filename,
            sort_ascending: true,
            loading_directory: false,
            dir_load_receiver: None,
        }
    }
}

#[derive(Default)]
pub(super) struct UiState {
    pub(super) show_options: bool,
    pub(super) help: HelpUiState,
}

impl super::EmuApp {
    pub(in crate::app) fn has_loaded_game(&self) -> bool {
        self.runtime.loaded_game.is_some()
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::app) fn set_status(&mut self, status: StatusUpdate) {
        self.runtime.status_msg = Some(ActiveStatusMessage::new(status));
    }

    pub(in crate::app) fn loaded_game(&self) -> Option<&LoadedGameState> {
        self.runtime.loaded_game.as_ref()
    }

    pub(in crate::app) fn is_paused(&self) -> bool {
        self.runtime.paused
    }

    pub(in crate::app) fn set_paused(&mut self, paused: bool) {
        self.runtime.paused = paused;
    }
}
