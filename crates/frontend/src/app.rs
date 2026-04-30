mod audio;
mod catalog;
mod config;
mod debugger;
mod help;
mod menu;
mod options;
mod panel;
mod rom;
mod runtime;
mod state;

use eframe::egui;

use self::config::SturdyConfig;
use self::debugger::DebuggerUiState;
#[cfg(not(target_arch = "wasm32"))]
use self::state::CatalogState;
use self::state::{RuntimeState, UiState};
use sturdygb_core::gb::ModelSelection;

pub const APP_NAME: &str = concat!("SturdyGB v", env!("CARGO_PKG_VERSION"));

const GB_W: usize = 160;
const GB_H: usize = 144;

pub struct EmuApp {
    runtime: RuntimeState,
    debugger: DebuggerUiState,
    #[cfg(not(target_arch = "wasm32"))]
    catalog: CatalogState,
    config: SturdyConfig,
    ui: UiState,
}

impl EmuApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        initial_rom: Option<String>,
        initial_model_selection: Option<ModelSelection>,
    ) -> Self {
        let mut config: SturdyConfig = Default::default();
        if let Some(storage) = cc.storage {
            if let Some(saved) = eframe::get_value::<SturdyConfig>(storage, "sturdygb_config") {
                config = saved;
            }
        }

        if let Some(model_selection) = initial_model_selection {
            config.model_selection = model_selection;
        }

        let debugger_layout = cc.storage.and_then(debugger::load_debugger_layout);

        let mut app = Self {
            runtime: RuntimeState::default(),
            debugger: DebuggerUiState::new(debugger_layout),
            #[cfg(not(target_arch = "wasm32"))]
            catalog: CatalogState::default(),
            config,
            ui: UiState::default(),
        };

        if let Some(rom) = initial_rom {
            app.load_rom_file(&rom, cc.storage);
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            app.reload_all_directories();
        }

        app
    }
}

impl eframe::App for EmuApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "sturdygb_config", &self.config);
        self.debugger.save_layout(storage);

        #[cfg(target_arch = "wasm32")]
        if let Some(state) = &mut self.runtime.loaded_game {
            if let Some(ram) = state.gb.get_battery_ram() {
                eframe::set_value(
                    storage,
                    &format!("sturdygb_sram_{}", state.title),
                    &ram.to_vec(),
                );
            }
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sync_viewport_state(ctx);
        self.process_background_tasks(ctx, _frame.storage());

        self.show_top_menu(ctx, _frame);
        self.show_overlay_windows(ctx, _frame.storage());
        self.show_main_panel(ctx, _frame);
    }
}
