use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Elements<'a, AppState> {
    let elements: Elements<AppState> = Elements::new();

    elements.push(crate::ui::screen::build());
    elements.push(crate::ui::centered_modal::build());
    elements.push(crate::ui::title_bar::build());
    elements.push(crate::ui::button_exit::build());
    elements.push(crate::ui::button_next::build());
    elements.push(crate::ui::board::build());
    elements.push(crate::ui::enter_key::build());
    elements.push(crate::ui::delete_key::build());
    elements.push(crate::ui::answer::build());
    elements.push(crate::ui::results::build());

    elements.extend(crate::ui::keyboard::build());
    elements.extend(crate::ui::graphs::build());

    elements
}
