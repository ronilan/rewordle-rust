use incredible::*;

use crate::AppState;

pub mod animators;
pub mod answer;
pub mod board;
pub mod button_exit;
pub mod button_next;
pub mod centered_modal;
pub mod delete_key;
pub mod enter_key;
pub mod graphs;
pub mod keyboard;
pub mod results;
pub mod screen;
pub mod title_bar;

pub static APP_WIDTH: usize = 80;
pub static APP_HEIGHT: usize = 24;

pub fn reposition<S>(el: &Element<S>, x: isize, y: isize, state: &AppState) {
    if Platform::columns() >= APP_WIDTH && Platform::rows() >= APP_HEIGHT {
        el.x(state.app_x + x);
        el.y(state.app_y + y);
    }
}

pub fn draw_relative<S>(el: &Element<S>, x: isize, y: isize, state: &AppState) {
    if Platform::columns() >= APP_WIDTH && Platform::rows() >= APP_HEIGHT {
        el.x(state.app_x + x);
        el.y(state.app_y + y);
        el.draw();
    }
}

pub fn mouse_over<S>(el: &Element<S>, x: isize, y: isize) -> bool {
    let w = el.visual.look.width() as isize;
    let h = el.visual.look.height() as isize;
    if w == 0 || h == 0 {
        return false;
    }
    let ex = el.visual.x.get();
    let ey = el.visual.y.get();
    x >= ex && x < ex + w && y >= ey && y < ey + h
}

pub fn clicked(event: &EventMouse) -> bool {
    matches!(event.mouse, Mouse::Up | Mouse::Drag)
}
