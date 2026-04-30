use super::help::HelpUiState;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
use super::config::{GameEntry, SortMethod};

pub(in crate::app) struct LoadedGameState {
    pub(super) gb: sturdygb_core::gb::Gb,
    pub(super) rgba: Vec<u8>,
    pub(super) leftover_audio: Vec<[f32; 2]>,
    pub(super) title: String,
    pub(super) rom_bytes: Vec<u8>,
    pub(super) save_path: Option<std::path::PathBuf>,
}

pub(super) struct RuntimeState {
    pub(super) loaded_game: Option<LoadedGameState>,
    pub(super) texture: Option<egui::TextureHandle>,
    pub(super) error_msg: Option<String>,
    pub(super) paused: bool,
    pub(super) rom_load_channel: (
        std::sync::mpsc::Sender<Result<Vec<u8>, String>>,
        std::sync::mpsc::Receiver<Result<Vec<u8>, String>>,
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
            paused: false,
            rom_load_channel: std::sync::mpsc::channel(),
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
