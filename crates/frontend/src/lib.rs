#![warn(clippy::all, rust_2018_idioms)]
#![allow(rustdoc::missing_crate_level_docs)]

mod app;
pub mod debug_views;
pub use app::EmuApp;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas_id: String) -> Result<(), eframe::wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast;

    console_error_panic_hook::set_once();

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    let window = eframe::web_sys::window()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("window was not available"))?;
    let document = window
        .document()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("document was not available"))?;
    let canvas = document
        .get_element_by_id(&canvas_id)
        .ok_or_else(|| {
            wasm_bindgen::JsValue::from_str(&format!("failed to find canvas '{canvas_id}'"))
        })?
        .dyn_into::<eframe::web_sys::HtmlCanvasElement>()?;

    eframe::WebRunner::new()
        .start(
            canvas,
            web_options,
            Box::new(|cc| Ok(Box::new(app::EmuApp::new(cc, None, None)))),
        )
        .await
        .map_err(|err| wasm_bindgen::JsValue::from_str(&format!("failed to start eframe: {err:?}")))
}
