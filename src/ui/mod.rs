// --- Single and Vectors of Elements
pub(crate) mod answer;
pub(crate) mod board;
pub(crate) mod button_exit;
pub(crate) mod button_next;
pub(crate) mod centered_modal;
pub(crate) mod delete_key;
pub(crate) mod enter_key;
pub(crate) mod graphs;
pub(crate) mod keyboard;
pub(crate) mod results;
pub(crate) mod screen;
pub(crate) mod title_bar;

// --- Helpers
pub(crate) mod animators;

// --- Utility
use crate::{
    tui_engine::{columns, draw, rows, Element},
    AppState,
};

pub(crate) static APP_WIDTH: u16 = 80; // Width of the game window
pub(crate) static APP_HEIGHT: u16 = 24; // Height of the game window

pub(crate) fn draw_relative<S>(el: &Element<S>, x: u16, y: u16, state: &AppState) {
    if columns() >= APP_WIDTH && rows() >= APP_HEIGHT {
        el.x.set(state.app_x + x);
        el.y.set(state.app_y + y);
        draw(el);
    }
}
