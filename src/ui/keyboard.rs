use crate::game::*;
use crate::tui_engine::*;

use crate::AppState;

/// Creates a single key element (A–Z)
pub fn create_key<'a>(x: u16, y: u16, letter: char) -> Element<'a, AppState> {
    let look = Look::from(vec![
        vec!["┌", "─", "┐"],
        vec!["│", &letter.to_string(), "│"],
        vec!["└", "─", "┘"],
    ]);

    let mut el = Element::new(x, y, look);

    // --- on_state: redraw + background status ---
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

            // replace look with styled one
            let styled = set_background_look(final_status, &el.look);
            el.look.update(styled);
        } else {
            // letter not used
            el.look.update(Look::from(vec![
                vec!["┌", "─", "┐"],
                vec!["│", &letter.to_string(), "│"],
                vec!["└", "─", "┘"],
            ]));
        }

        crate::ui::draw_relative(el,x, y, state);
    }));

    // --- on_click: inject letter into state ---
    el.on_click = Some(Box::new(move |el, state, event| {
        if mouse_over(el, event) {
            mutate_state_on_input(el, state, &letter.to_string());
        }
    }));

    el
}

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
            let x = 40 + line_index as u16 * 2 + (line_index as u16 / 2) + (index as u16 * 3);
            let y = 13 + line_index as u16 * 3;

            kb.push(create_key(x, y, letter));
        }
    }

    kb
}
