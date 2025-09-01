use crate::game::mutate_state_new_game;
use crate::storage::save;
use crate::tui_engine::*;
use crate::{AppState, GameStatus};

static X: u16 = 75;
static Y: u16 = 22;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_next: Element<AppState> = Element::new(0, 0, Look::new());

    button_next.on_click = Some(Box::new(|el, state, event| {
        if state.game != GameStatus::InPlay {
            if mouse_over(el, event) {
                mutate_state_new_game(state);
            }
        }
    }));
    button_next.on_state = Some(Box::new(|el, state| {
        if state.game != GameStatus::InPlay {
            el.look
                .update(terminal_style::format::underline(Look::from("Next")));
            // once we move to next word - save the status of the game
            save(&state.results, state.streak, state.word_index).ok();
        } else {
            el.look.update(Look::from("    "));
        }

        crate::ui::draw_relative(el, X, Y, state);
    }));
    button_next
}
