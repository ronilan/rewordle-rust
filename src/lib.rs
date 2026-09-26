#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
pub use app::{core, screens, ui};

#[cfg(target_arch = "wasm32")]
pub use core::model::{AppState, GameStatus, WordStatus};

#[cfg(target_arch = "wasm32")]
mod game;
#[cfg(target_arch = "wasm32")]
#[path = "storage_web.rs"]
mod storage;
#[cfg(target_arch = "wasm32")]
mod words;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    core::application_flow::run_flow();
}
