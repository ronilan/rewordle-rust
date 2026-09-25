use incredible::*;

use crate::{ui::APP_HEIGHT, ui::APP_WIDTH, AppState};

// Utility element with no visible look.
// Tracks the live terminal size: recenters the app and clears/redraws on
// ANY size change (even when the centered position itself is unchanged,
// e.g. shrinking 80 -> 79 where app_x/app_y stay 0).
pub fn build() -> Element<AppState> {
    let screen: Element<AppState> = Element::new();
    screen.on_loop(|_el, state, _event| {
        if state.exit_flag {
            Globals::set_exit_flagged(true);
            return;
        }
        let cols = Platform::columns();
        let rows = Platform::rows();
        let x = (cols.saturating_sub(APP_WIDTH) / 2) as isize;
        let y = (rows.saturating_sub(APP_HEIGHT) / 2) as isize;

        if x != state.app_x
            || y != state.app_y
            || cols != state.term_cols
            || rows != state.term_rows
        {
            Platform::clear_screen();

            state.app_x = x;
            state.app_y = y;
            state.term_cols = cols;
            state.term_rows = rows;
        }
    });
    screen
}
