use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Elements<'a, AppState> {
    let elements: Elements<AppState> = Elements::new();

    elements.push(crate::elements::screen::build());
    elements.push(crate::elements::centered_modal::build());
    elements.push(crate::elements::title_bar::build());
    elements.push(crate::elements::button_exit::build());
    elements.push(crate::elements::button_next::build());
    elements.push(crate::elements::board::build());
    elements.push(crate::elements::enter_key::build());
    elements.push(crate::elements::delete_key::build());
    elements.push(crate::elements::answer::build());
    elements.push(crate::elements::results::build());

    elements.extend(crate::elements::keyboard::build());
    elements.extend(crate::elements::graphs::build());

    elements
}
