use eframe::egui;
use std::collections::HashMap;
use sturdygb_core::joypad::JoypadButton;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SturdyConfig {
    pub scale: ScaleMode,
    pub palette: Palette,
    #[cfg(not(target_arch = "wasm32"))]
    pub rom_directories: Vec<std::path::PathBuf>,
    pub keybinds: HashMap<JoypadButton, egui::Key>,
    #[cfg(not(target_arch = "wasm32"))]
    pub fullscreen: bool,
}

impl SturdyConfig {
    fn default_key(btn: &JoypadButton) -> egui::Key {
        match btn {
            JoypadButton::Up => egui::Key::ArrowUp,
            JoypadButton::Down => egui::Key::ArrowDown,
            JoypadButton::Left => egui::Key::ArrowLeft,
            JoypadButton::Right => egui::Key::ArrowRight,
            JoypadButton::A => egui::Key::Z,
            JoypadButton::B => egui::Key::X,
            JoypadButton::Start => egui::Key::Enter,
            JoypadButton::Select => egui::Key::Space,
        }
    }

    pub(super) fn keybind(&self, btn: &JoypadButton) -> egui::Key {
        self.keybinds
            .get(btn)
            .copied()
            .unwrap_or_else(|| Self::default_key(btn))
    }
}

impl Default for SturdyConfig {
    fn default() -> Self {
        let mut keybinds = HashMap::new();
        keybinds.insert(JoypadButton::Up, egui::Key::ArrowUp);
        keybinds.insert(JoypadButton::Down, egui::Key::ArrowDown);
        keybinds.insert(JoypadButton::Left, egui::Key::ArrowLeft);
        keybinds.insert(JoypadButton::Right, egui::Key::ArrowRight);
        keybinds.insert(JoypadButton::A, egui::Key::Z);
        keybinds.insert(JoypadButton::B, egui::Key::X);
        keybinds.insert(JoypadButton::Start, egui::Key::Enter);
        keybinds.insert(JoypadButton::Select, egui::Key::Space);

        Self {
            #[cfg(not(target_arch = "wasm32"))]
            rom_directories: Vec::new(),
            scale: ScaleMode::Integer(4.0),
            palette: Palette::Greyscale,
            keybinds,
            #[cfg(not(target_arch = "wasm32"))]
            fullscreen: false,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, Debug)]
pub enum ScaleMode {
    Integer(f32),
    Stretch,
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy, Debug)]
pub enum Palette {
    Greyscale,
    ClassicGreen,
    Pocket,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SortMethod {
    Filename,
    Title,
    Company,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub(super) struct GameEntry {
    pub path: std::path::PathBuf,
    pub filename: String,
    pub title: String,
    pub company: String,
}
