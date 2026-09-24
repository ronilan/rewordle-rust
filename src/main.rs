#[cfg(not(target_arch = "wasm32"))]
mod app;

#[cfg(not(target_arch = "wasm32"))]
pub use app::{core, screens, ui};

#[cfg(not(target_arch = "wasm32"))]
pub use core::model::{AppState, GameStatus, WordStatus};

#[cfg(not(target_arch = "wasm32"))]
mod game;
#[cfg(not(target_arch = "wasm32"))]
mod storage;
#[cfg(not(target_arch = "wasm32"))]
mod words;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    core::application_flow::run_flow();
}

#[cfg(target_arch = "wasm32")]
fn main() {}
