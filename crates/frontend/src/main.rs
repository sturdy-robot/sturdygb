#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)] // it's an app

mod app;

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

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([(160.0) * 4.0, (144.0) * 4.0 + 30.0])
            .with_title(format!("SturdyGB")),
        ..Default::default()
    };

    eframe::run_native(
        "SturdyGB",
        options,
        Box::new(|cc| Ok(Box::new(app::EmuApp::new(cc, cli.rom)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
}
