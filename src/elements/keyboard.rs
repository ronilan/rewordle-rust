use crate::game::*;
use crate::tui_engine::*;

use crate::AppState;

// Maps letter status to background color
fn status_to_ansi(status: u8) -> u8 {
    match status {
        2 => 2,
        1 => 3,
        _ => 8,
    }
}

/// Creates a single key element (A–Z)
pub fn create_key<'a>(x: u16, y: u16, letter: char) -> Element<'a, AppState> {
    let look = Look::from(vec![
        vec!["┌", "─", "┐"],
        vec!["│", &letter.to_string(), "│"],
        vec!["└", "─", "┘"],
    ]);

    let mut el = Element::new(x, y, look);

    el.on_click = Some(Box::new(move |el, state, event| {
        if mouse_over(el, event) {
            mutate_state_letter(state, &letter.to_string());
        }
    }));
    el.on_state = Some(Box::new(move |el, state: &AppState| {
        if state.used.contains(&letter) {
            let mut final_status = 0;

            for guess_arr in &state.status {
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

            let styled =
                terminal_style::format::background(status_to_ansi(final_status), &el.look).unwrap();

            el.look.update(styled);
        } else {
            // letter not used
            el.look.update(Look::from(vec![
                vec!["┌", "─", "┐"],
                vec!["│", &letter.to_string(), "│"],
                vec!["└", "─", "┘"],
            ]));
        }

        crate::elements::draw_relative(el, x, y, state);
    }));

    el
}

static X: u16 = 40;
static Y: u16 = 13;

pub fn build<'a>() -> Elements<'a, AppState> {
    // Layout rows
    let keys: Vec<Vec<char>> = vec![
        vec!['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
        vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
        vec!['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
    ];

    let kb = Elements::new();

    for (line_index, key_line) in keys.iter().enumerate() {
        for (index, &letter) in key_line.iter().enumerate() {
            let x = X + line_index as u16 * 2 + (line_index as u16 / 2) + (index as u16 * 3);
            let y = Y + line_index as u16 * 3;

            kb.push(create_key(x, y, letter));
        }
    }

    kb
}
