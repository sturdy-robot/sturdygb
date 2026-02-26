#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)] // it's an app

mod app;

use crate::app::APP_NAME;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(name = "sturdygb")]
    struct Cli {
        #[arg(value_name = "ROM")]
        rom: Option<String>,
    }

    let cli = Cli::parse();

    let icon_data = match image::load_from_memory(include_bytes!(
        "../../../images/sturdygb_symbol_64x64.png"
    )) {
        Ok(img) => {
            let img = img.into_rgba8();
            let (width, height) = img.dimensions();
            let rgba = img.into_raw();
            Some(std::sync::Arc::new(eframe::egui::IconData {
                rgba,
                width,
                height,
            }))
        }
        Err(_) => None,
    };

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([(160.0) * 4.0, (144.0) * 4.0 + 30.0])
        .with_title(APP_NAME.to_string());

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::EmuApp::new(cc, cli.rom)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
}
