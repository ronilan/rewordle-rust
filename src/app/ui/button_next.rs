use incredible::*;

use crate::{
    game::mutate_state_new_game,
    ui::{clicked, draw_relative, mouse_over},
    AppState, GameStatus,
};

static X: isize = 75;
static Y: isize = 22;

fn underlined(text: &str) -> Look {
    let d = Decor::default();
    d.underline.set(Some(UnderlineKind::Single));
    Look::from(vec![text
        .chars()
        .map(|c| Block::new(c, d.clone()))
        .collect::<Vec<Block>>()])
}

pub fn build() -> Element<AppState> {
    let button: Element<AppState> = Element::new();
    button.on_mouse(|el, state, event| {
        if state.game != GameStatus::InPlay && clicked(event) && mouse_over(el, event.x, event.y) {
            mutate_state_new_game(state);
        }
    });
    button.on_state(|el, state, _event| {
        if state.game != GameStatus::InPlay {
            el.look(underlined("Next"));
            #[cfg(not(target_arch = "wasm32"))]
            crate::storage::save(&state.results, state.streak, state.word_index).ok();
        } else {
            el.look(Look::from("    "));
        }
        draw_relative(el, X, Y, state);
    });
    button
}
