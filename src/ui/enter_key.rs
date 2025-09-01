use crate::game::*;
use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut enter_key = Element::new(
        0,
        0,
        Look::from(vec![
            vec!["┌", "─", "─", "─", "┐"],
            vec!["│", " ", "↩", " ", "│"],
            vec!["└", "─", "─", "─", "┘"],
        ]),
    );
    enter_key.on_state = Some(Box::new(|el, state: &AppState| {
        el.x.set(state.app_x + 40);
        el.y.set(state.app_y + 19);

        crate::ui::draw_if_fits(el);
    }));
    enter_key.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            mutate_state_on_input(el, state, &"enter".to_string());
        }
    }));

    enter_key
}
