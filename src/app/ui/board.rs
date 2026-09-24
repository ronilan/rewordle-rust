use incredible::*;

use crate::{
    game::*,
    ui::{
        animators::{drive_reveal, drive_shake},
        draw_relative,
    },
    AppState, WordStatus,
};

pub(crate) static X: isize = 9;
pub(crate) static Y: isize = 4;

pub(crate) fn status_to_ansi(status: u8) -> u8 {
    match status {
        2 => 2,
        1 => 3,
        _ => 8,
    }
}

fn bg(status: u8) -> Decor {
    let d = Decor::default();
    d.background.set(Some(Color::Ansi(status_to_ansi(status))));
    d
}

pub(crate) fn board_look(state: &AppState) -> Vec<Vec<Block>> {
    let mut rows = Vec::with_capacity(18);
    for r in 0..6 {
        let guess_arr = &state.status[r];
        let answer_arr: Vec<char> = state.answer.chars().collect();
        for sub in 0..3 {
            let mut row = Vec::with_capacity(25);
            for c in 0..5 {
                let letter = state.status[r][c];
                let decor = if state.in_play == r || letter == ' ' {
                    Decor::default()
                } else {
                    bg(get_letter_status(c, guess_arr, &answer_arr))
                };
                let cells: [char; 5] = match sub {
                    0 => ['┌', '─', '─', '─', '┐'],
                    1 => ['│', ' ', letter, ' ', '│'],
                    _ => ['└', '─', '─', '─', '┘'],
                };
                for ch in cells {
                    row.push(Block::new(ch, decor.clone()));
                }
            }
            rows.push(row);
        }
    }
    rows
}

fn key_to_str(key: &Key) -> String {
    match key {
        Key::Char(c) => c.to_string(),
        Key::Enter => "enter".to_string(),
        Key::Delete => "delete".to_string(),
        _ => String::new(),
    }
}

pub fn build() -> Element<AppState> {
    let board: Element<AppState> = Element::new();
    board
        .x(X)
        .y(Y)
        .on_loop(|el, state, _event| {
            if state.word_status == WordStatus::Invalid {
                drive_shake(el, state);
            } else if state.word_status == WordStatus::Valid {
                drive_reveal(el, state);
            }
        })
        .on_key(|_el, state, event| {
            mutate_state_letter(state, &key_to_str(&event.key));
        })
        .on_state(|el, state, _event| {
            el.look(Look::from(board_look(state)));
            draw_relative(el, X, Y, state);
        });
    board
}
