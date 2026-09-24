use incredible::*;

use crate::{
    ui::{clicked, mouse_over},
    AppState,
};

pub fn build() -> Element<AppState> {
    let button: Element<AppState> = Element::new();
    button
        .x(0)
        .y(0)
        .look(Look::from("Exit"))
        .on_key(|_el, state, event| {
            if event.key == Key::Char('c') && event.modifiers.contains(&KeyMod::Ctrl) {
                state.exit_flag = true;
            }
        })
        .on_mouse(|el, state, event| {
            if clicked(event) && mouse_over(el, event.x, event.y) {
                state.exit_flag = true;
            }
        })
        .on_state(|el, _state, _event| {
            // Owns the last 4 columns of the title row; the title bar leaves
            // this space empty. Same on_state cycle, adjacent cells, no overlap.
            el.x((Platform::columns().saturating_sub(4)) as isize);
            el.decoration.style.base.decor.inverse.set(Some(true));
            el.decoration
                .style
                .base
                .decor
                .underline
                .set(Some(UnderlineKind::Single));
            el.decorate();
            el.draw();
        });
    button
}
