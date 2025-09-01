use crate::game::*;
use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut delete_key: Element<AppState> = Element::new(
        0,
        0,
        Look::from(vec![
            vec!["┌", "─", "─", "┐"],
            vec!["│", " ", "⌫", "│"],
            vec!["└", "─", "─", "┘"],
        ]),
    );

    delete_key.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 66);
        el.y.set(state.app_y + 19);

        crate::ui::draw_if_fits(el);
    }));
    delete_key.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            mutate_state_on_input(el, state, &"delete".to_string());
        }
    }));

    delete_key
}
