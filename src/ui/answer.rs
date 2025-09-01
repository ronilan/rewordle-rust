use crate::tui_engine::*;
use crate::{AppState, GameStatus};

pub fn build<'a>() -> Element<'a, AppState> {
    let mut answer: Element<AppState> = Element::new(0, 0, Look::new());

    answer.on_state = Some(Box::new(|el, state| {
        match state.game {
            GameStatus::Won => {
                let index_in_play = state.in_play;
                let word_index = state.word_index;
                let mapped = match index_in_play {
                    1 => format!(
                        "Wordle #{} {}",
                        word_index,
                        terminal_style::format::inverse("Genius")
                    ),
                    2 => format!(
                        "Wordle #{} {}",
                        word_index,
                        terminal_style::format::inverse("Magnificent")
                    ),
                    3 => format!(
                        "Wordle #{} {}",
                        word_index,
                        terminal_style::format::inverse("Impressive")
                    ),
                    4 => format!(
                        "Wordle #{} {}",
                        word_index,
                        terminal_style::format::inverse("Splendid")
                    ),
                    5 => format!(
                        "Wordle #{} {}",
                        word_index,
                        terminal_style::format::inverse("Great")
                    ),
                    6 => format!(
                        "Wordle #{} {}",
                        word_index,
                        terminal_style::format::inverse("Phew")
                    ),
                    _ => format!(
                        "Wordle #{} {}",
                        word_index,
                        terminal_style::format::inverse("???")
                    ),
                };

                el.look.update(vec![vec![mapped]]);

                // then show the actual answer
                let final_mapped = format!("Wordle #{} {}", state.word_index, state.answer);
                el.look.update(vec![vec![final_mapped]]);
            }
            GameStatus::Lost => {
                let mapped = format!(
                    "Wordle #{} {}",
                    state.word_index,
                    terminal_style::format::inverse(state.answer)
                );
                el.look.update(vec![vec![mapped]]);
            }
            GameStatus::InPlay => {
                let mapped = format!("Wordle #{} {}", state.word_index + 1, "?????");
                el.look.update(vec![vec![mapped]]);
            }
        }

        el.x.set(state.app_x + 30);
        el.y.set(state.app_y + 2);
        crate::ui::draw_if_fits(el);
    }));

    answer
}
