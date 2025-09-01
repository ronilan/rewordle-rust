use crate::game::*;
use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 40;
static Y: u16 = 19;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut enter_key = Element::new(
        X,
        Y,
        Look::from(vec![
            vec!["┌", "─", "─", "─", "┐"],
            vec!["│", " ", "↩", " ", "│"],
            vec!["└", "─", "─", "─", "┘"],
        ]),
    );
    enter_key.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            mutate_state_letter(state, &"enter".to_string());
        }
    }));
    enter_key.on_state = Some(Box::new(|el, state: &AppState| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    enter_key
}
