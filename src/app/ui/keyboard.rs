use incredible::*;

use crate::{
    game::*,
    ui::{clicked, draw_relative, mouse_over},
    AppState,
};

fn status_to_ansi(status: u8) -> u8 {
    match status {
        2 => 2,
        1 => 3,
        _ => 8,
    }
}

fn key_look(letter: char, status: Option<u8>) -> Look {
    let decor = match status {
        Some(s) => {
            let d = Decor::default();
            d.background.set(Some(Color::Ansi(status_to_ansi(s))));
            d
        }
        None => Decor::default(),
    };
    let rows: Vec<Vec<Block>> = vec![
        vec!['┌', '─', '┐'],
        vec!['│', letter, '│'],
        vec!['└', '─', '┘'],
    ]
    .into_iter()
    .map(|row| {
        row.into_iter()
            .map(|c| Block::new(c, decor.clone()))
            .collect()
    })
    .collect();
    Look::from(rows)
}

/// Creates a single key element (A–Z)
pub fn create_key(x: isize, y: isize, letter: char) -> Element<AppState> {
    let key: Element<AppState> = Element::new();
    key.x(x).y(y).look(key_look(letter, None));
    key.on_mouse(move |el, state, event| {
        if clicked(event) && mouse_over(el, event.x, event.y) {
            mutate_state_letter(state, &letter.to_string());
        }
    });
    key.on_state(move |el, state, _event| {
        // None = never tried (plain). Some(status) = tried, always painted —
        // gray (0) included, not just green/yellow.
        let tried = state.used.contains(&letter);
        let mut final_status = 0;
        if tried {
            // Only submitted rows (below in_play): the current row is unsubmitted
            // and must not leak colors onto the keyboard.
            for (r, guess_arr) in state.status.iter().enumerate() {
                if r >= state.in_play {
                    continue;
                }
                let answer_arr: Vec<char> = state.answer.chars().collect();
                for (i, &item) in guess_arr.iter().enumerate() {
                    if item == letter {
                        let letter_status = get_letter_status(i, guess_arr, &answer_arr);
                        if letter_status > final_status {
                            final_status = letter_status;
                        }
                    }
                }
            }
        }
        if tried {
            el.look(key_look(letter, Some(final_status)));
        } else {
            el.look(key_look(letter, None));
        }
        draw_relative(el, x, y, state);
    });
    key
}

static X: isize = 40;
static Y: isize = 13;

pub fn build() -> Vec<Element<AppState>> {
    // Layout rows
    let keys: Vec<Vec<char>> = vec![
        vec!['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
        vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
        vec!['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
    ];

    let mut kb = Vec::new();

    for (line_index, key_line) in keys.iter().enumerate() {
        for (index, &letter) in key_line.iter().enumerate() {
            let li = line_index as isize;
            let x = X + li * 2 + (li / 2) + (index as isize * 3);
            let y = Y + li * 3;

            kb.push(create_key(x, y, letter));
        }
    }

    kb
}
