use incredible::*;

use crate::{ui::draw_relative, AppState, GameStatus};

static X: isize = 30;
static Y: isize = 2;

fn plain_row(s: &str) -> Vec<Block> {
    s.chars().map(|c| Block::new(c, Decor::default())).collect()
}

fn inverse_row(s: &str) -> Vec<Block> {
    let d = Decor::default();
    d.inverse.set(Some(true));
    s.chars().map(|c| Block::new(c, d.clone())).collect()
}

pub fn build() -> Element<AppState> {
    let answer: Element<AppState> = Element::new();
    answer.on_state(|el, state, _event| {
        let row: Vec<Block> = match state.game {
            GameStatus::Won => {
                let praise = match state.in_play {
                    1 => "Genius",
                    2 => "Magnificent",
                    3 => "Impressive",
                    4 => "Splendid",
                    5 => "Great",
                    6 => "Phew",
                    _ => "???",
                };
                let mut row = plain_row(&format!("Wordle #{} ", state.word_index));
                row.extend(inverse_row(praise));
                row
            }
            GameStatus::Lost => {
                let mut row = plain_row(&format!("Wordle #{} ", state.word_index));
                row.extend(inverse_row(state.answer));
                row
            }
            GameStatus::InPlay => {
                plain_row(&format!("Wordle #{} {}", state.word_index + 1, "?????"))
            }
        };
        el.look(Look::from(vec![row]));
        draw_relative(el, X, Y, state);
    });
    answer
}
