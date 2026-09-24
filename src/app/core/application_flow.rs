use incredible::{run as tui_run, setup, Globals, Providers};

#[cfg(all(
    not(target_arch = "wasm32"),
    not(all(target_os = "macos", feature = "macos-native")),
    not(all(target_os = "windows", feature = "windows-native"))
))]
use incredible_clipboard_terminal::TerminalClipboard;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(all(target_os = "macos", feature = "macos-native")),
    not(all(target_os = "windows", feature = "windows-native"))
))]
use incredible_event_loop_terminal::run_event_loop as looper;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(all(target_os = "macos", feature = "macos-native")),
    not(all(target_os = "windows", feature = "windows-native"))
))]
use incredible_input_terminal_nix::TerminalNixInput;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(all(target_os = "macos", feature = "macos-native")),
    not(all(target_os = "windows", feature = "windows-native"))
))]
use incredible_output_terminal::TerminalOutput;

#[cfg(target_arch = "wasm32")]
use incredible_clipboard_browser::BrowserClipboard;
#[cfg(target_arch = "wasm32")]
use incredible_event_loop_browser::run_event_loop as looper;
#[cfg(target_arch = "wasm32")]
use incredible_input_browser::BrowserInput;
#[cfg(target_arch = "wasm32")]
use incredible_output_html::HtmlOutput;

#[cfg(all(target_os = "macos", feature = "macos-native"))]
use incredible_clipboard_macos::MacosClipboard;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
use incredible_event_loop_macos::run_event_loop as macos_looper;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
use incredible_input_macos::MacosInput;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
use incredible_output_macos::MacosOutput;

#[cfg(all(target_os = "windows", feature = "windows-native"))]
use incredible_clipboard_windows::WindowsClipboard;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
use incredible_event_loop_windows::run_event_loop as windows_looper;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
use incredible_input_windows::WindowsInput;
#[cfg(all(target_os = "windows", feature = "windows-native"))]
use incredible_output_windows::WindowsOutput;

use crate::{core::model::initial_state, screens::game};

#[cfg(all(
    not(target_arch = "wasm32"),
    not(all(target_os = "macos", feature = "macos-native")),
    not(all(target_os = "windows", feature = "windows-native"))
))]
fn setup_runtime() {
    setup(Providers {
        input: Box::new(TerminalNixInput::new()),
        output: Box::new(TerminalOutput::new()),
        clipboard: Box::new(TerminalClipboard),
    });
}

#[cfg(target_arch = "wasm32")]
fn setup_runtime() {
    setup(Providers {
        input: Box::new(BrowserInput::new()),
        output: Box::new(HtmlOutput::new()),
        clipboard: Box::new(BrowserClipboard),
    });
}

#[cfg(not(any(
    all(target_os = "macos", feature = "macos-native"),
    all(target_os = "windows", feature = "windows-native")
)))]
pub fn run_flow() {
    // Providers first: initial_state() centers via Platform size.
    Globals::set_tick_rate(33.0);
    setup_runtime();

    let state = initial_state();
    let root = game::build();

    // get() blocks until exit: only valid on native, where the loop is synchronous.
    #[cfg(all(
        not(target_arch = "wasm32"),
        not(all(target_os = "macos", feature = "macos-native"))
    ))]
    let _ = tui_run(root, state, looper, |_| {}).get();
    #[cfg(target_arch = "wasm32")]
    let _ = tui_run(root, state, looper, |_| {});
}

/// macOS native entry: same plain-Element tree, driven by the macOS event
/// loop inside the GUI window. Must run inside run_app (window exists).
#[cfg(all(target_os = "macos", feature = "macos-native"))]
pub fn run_flow_macos() {
    setup(Providers {
        input: Box::new(MacosInput::new(incredible_window_macos::window())),
        output: Box::new(MacosOutput::new(incredible_window_macos::window())),
        clipboard: Box::new(MacosClipboard),
    });

    Globals::set_tick_rate(33.0);

    let state = initial_state();
    let root = game::build();

    // Like wasm, the macOS loop is asynchronous: no blocking get().
    let _ = tui_run(root, state, macos_looper, |_| {});
}

/// Windows native entry: same shape as macOS. Must run inside run_app.
#[cfg(all(target_os = "windows", feature = "windows-native"))]
pub fn run_flow_windows() {
    setup(Providers {
        input: Box::new(WindowsInput::new(incredible_window_windows::window())),
        output: Box::new(WindowsOutput::new(incredible_window_windows::window())),
        clipboard: Box::new(WindowsClipboard),
    });

    Globals::set_tick_rate(33.0);

    let state = initial_state();
    let root = game::build();

    let _ = tui_run(root, state, windows_looper, |_| {});
}
