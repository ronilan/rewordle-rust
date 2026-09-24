use incredible::*;

use crate::{
    game::mutate_state_letter,
    ui::{clicked, draw_relative, mouse_over},
    AppState,
};

static X: isize = 66;
static Y: isize = 19;

pub fn build() -> Element<AppState> {
    let key: Element<AppState> = Element::new();
    key.x(X).y(Y).look(Look::from(vec![
        vec!['┌', '─', '─', '┐'],
        vec!['│', ' ', '⌫', '│'],
        vec!['└', '─', '─', '┘'],
    ]));
    key.on_mouse(|el, state, event| {
        if clicked(event) && mouse_over(el, event.x, event.y) {
            mutate_state_letter(state, "delete");
        }
    });
    key.on_state(|el, state, _event| {
        draw_relative(el, X, Y, state);
    });
    key
}
