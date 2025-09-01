use crate::tui_engine::*; // Custom TUI engine for Element, Look, drawing, etc.
use crate::ui::constants::{APP_HEIGHT, APP_WIDTH};

pub(crate) fn draw_if_fits<S>(el: &Element<S>) {
    if columns() >= APP_WIDTH && rows() >= APP_HEIGHT {
        draw(el);
    }
}
