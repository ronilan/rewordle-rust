use crate::game::*;
use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 66;
static Y: u16 = 19;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut delete_key: Element<AppState> = Element::new(
        X,
        Y,
        Look::from(vec![
            vec!["┌", "─", "─", "┐"],
            vec!["│", " ", "⌫", "│"],
            vec!["└", "─", "─", "┘"],
        ]),
    );

    delete_key.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            mutate_state_letter(state, &"delete".to_string());
        }
    }));
    delete_key.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    delete_key
}
