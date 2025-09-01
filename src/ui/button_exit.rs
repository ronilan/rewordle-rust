use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_exit: Element<AppState> =
        Element::new(0, 0, terminal_style::format::underline(Look::from("Exit")));
    button_exit.on_keypress = Some(Box::new(|_el, state, event| {
        if event.key == Some("c".to_string()) && event.modifiers.contains(&"ctrl".to_string()) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_state = Some(Box::new(|el, _state| {
        el.x.set(columns() - 4);
        draw(el);
    }));

    button_exit
}
