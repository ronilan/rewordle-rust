use incredible::*;

use crate::{ui, AppState};

pub fn build() -> Element<AppState> {
    let root = Element::new();
    root.elements.push(ui::screen::build());
    root.elements.push(ui::centered_modal::build());
    root.elements.push(ui::title_bar::build());
    root.elements.push(ui::button_exit::build());
    root.elements.push(ui::button_next::build());
    root.elements.push(ui::board::build());
    root.elements.push(ui::enter_key::build());
    root.elements.push(ui::delete_key::build());
    root.elements.push(ui::answer::build());
    root.elements.push(ui::results::build());
    for key in ui::keyboard::build() {
        root.elements.push(key);
    }
    for bar in ui::graphs::build() {
        root.elements.push(bar);
    }
    root
}
