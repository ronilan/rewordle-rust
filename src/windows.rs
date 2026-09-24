#![cfg_attr(
    all(feature = "windows-native", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[cfg(feature = "windows-native")]
mod app;

#[cfg(feature = "windows-native")]
pub use app::{core, screens, ui};

#[cfg(feature = "windows-native")]
pub use core::model::{AppState, GameStatus, WordStatus};

#[cfg(feature = "windows-native")]
mod game;
#[cfg(feature = "windows-native")]
mod storage;
#[cfg(feature = "windows-native")]
mod words;

#[cfg(feature = "windows-native")]
fn main() {
    incredible_window_windows::set_window_title(
        option_env!("APP_NAME").unwrap_or("An Incredible App"),
    );
    incredible_window_windows::run_app(core::application_flow::run_flow_windows);
}

#[cfg(not(feature = "windows-native"))]
fn main() {
    eprintln!(
        "This binary is only available for Windows targets with the 'windows-native' feature enabled."
    );
}
