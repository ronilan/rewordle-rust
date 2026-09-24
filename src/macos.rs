#[cfg(feature = "macos-native")]
mod app;

#[cfg(feature = "macos-native")]
pub use app::{core, screens, ui};

#[cfg(feature = "macos-native")]
pub use core::model::{AppState, GameStatus, WordStatus};

#[cfg(feature = "macos-native")]
mod game;
#[cfg(feature = "macos-native")]
mod storage;
#[cfg(feature = "macos-native")]
mod words;

#[cfg(feature = "macos-native")]
fn main() {
    incredible_window_macos::set_window_title(
        option_env!("APP_NAME").unwrap_or("An Incredible App"),
    );
    incredible_window_macos::run_app(core::application_flow::run_flow_macos);
}

#[cfg(not(feature = "macos-native"))]
fn main() {
    eprintln!(
        "This binary is only available for macOS targets with the 'macos-native' feature enabled."
    );
}
