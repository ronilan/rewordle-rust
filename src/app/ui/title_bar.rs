use incredible::*;

use crate::AppState;

pub fn build() -> Element<AppState> {
    let title_bar: Element<AppState> = Element::new();
    title_bar.x(0).y(0).on_state(|el, _state, _event| {
        // Leaves the last 4 columns free for button_exit.
        let cols = Platform::columns().saturating_sub(4);

        let mut line = " ".repeat(cols);
        line.replace_range(0.."Rewordle".len().min(cols), "Rewordle");

        el.look(Look::from(line));
        el.decoration.style.base.decor.inverse.set(Some(true));
        el.decorate();
        el.draw();
    });
    title_bar
}
