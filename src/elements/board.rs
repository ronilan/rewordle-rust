use crate::elements::animators::*;
use crate::game::*;
use crate::tui_engine::*;
use crate::{AppState, WordStatus};

static X: u16 = 9;
static Y: u16 = 4;

// Maps letter status to background color
fn status_to_ansi(status: u8) -> u8 {
    match status {
        2 => 2,
        1 => 3,
        _ => 8,
    }
}

pub fn build<'a>() -> Element<'a, AppState> {
    let mut board: Element<AppState> = Element::new(0, 0, Look::new());

    // the board initiates animations based on state.
    // this has to be done in the loop because the state can not be mutated by the on_state callback
    board.on_loop = Some(Box::new(|el, state, _event| {
        if state.word_status == WordStatus::Invalid {
            invalid_word_animator(el, state.in_play);
            state.word_status = WordStatus::InPlay; // reset status
        }
        if state.word_status == WordStatus::Valid {
            reveal_animator(el, state.in_play, &state.status.clone(), &state.answer);

            state.word_status = WordStatus::InPlay; // reset status
            mutate_state_new_word(state);
        }
    }));
    board.on_keypress = Some(Box::new(|_el, state, event| {
        mutate_state_letter(state, &event.key.clone().unwrap_or_default());
    }));
    board.on_state = Some(Box::new(|el, state| {
        let empty_board = Look::from(vec![
            vec![
                "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌",
                "─", "─", "─", "┐", "┌", "─", "─", "─", "┐",
            ],
            vec![
                "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│",
                " ", " ", " ", "│", "│", " ", " ", " ", "│",
            ],
            vec![
                "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└",
                "─", "─", "─", "┘", "└", "─", "─", "─", "┘",
            ],
            vec![
                "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌",
                "─", "─", "─", "┐", "┌", "─", "─", "─", "┐",
            ],
            vec![
                "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│",
                " ", " ", " ", "│", "│", " ", " ", " ", "│",
            ],
            vec![
                "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└",
                "─", "─", "─", "┘", "└", "─", "─", "─", "┘",
            ],
            vec![
                "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌",
                "─", "─", "─", "┐", "┌", "─", "─", "─", "┐",
            ],
            vec![
                "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│",
                " ", " ", " ", "│", "│", " ", " ", " ", "│",
            ],
            vec![
                "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└",
                "─", "─", "─", "┘", "└", "─", "─", "─", "┘",
            ],
            vec![
                "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌",
                "─", "─", "─", "┐", "┌", "─", "─", "─", "┐",
            ],
            vec![
                "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│",
                " ", " ", " ", "│", "│", " ", " ", " ", "│",
            ],
            vec![
                "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└",
                "─", "─", "─", "┘", "└", "─", "─", "─", "┘",
            ],
            vec![
                "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌",
                "─", "─", "─", "┐", "┌", "─", "─", "─", "┐",
            ],
            vec![
                "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│",
                " ", " ", " ", "│", "│", " ", " ", " ", "│",
            ],
            vec![
                "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└",
                "─", "─", "─", "┘", "└", "─", "─", "─", "┘",
            ],
            vec![
                "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌", "─", "─", "─", "┐", "┌",
                "─", "─", "─", "┐", "┌", "─", "─", "─", "┐",
            ],
            vec![
                "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│", " ", " ", " ", "│", "│",
                " ", " ", " ", "│", "│", " ", " ", " ", "│",
            ],
            vec![
                "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└", "─", "─", "─", "┘", "└",
                "─", "─", "─", "┘", "└", "─", "─", "─", "┘",
            ],
        ]);

        let mapped = empty_board
            .clone()
            .cells()
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_index, item)| {
                        let r = row_index / 3;
                        let c = col_index / 5;
                        let letter = state.status[r][c];
                        let guess_arr = &state.status[r];
                        let answer_arr: Vec<char> = state.answer.chars().collect();

                        if row_index % 3 == 1 && col_index % 5 == 2 {
                            // center cell (letter slot)
                            if state.in_play == r {
                                // current attempt row
                                letter.to_string()
                            } else if letter == ' ' {
                                letter.to_string()
                            } else {
                                // previous guess: apply background
                                let letter_status = get_letter_status(c, guess_arr, &answer_arr);
                                terminal_style::format::background(
                                    status_to_ansi(letter_status),
                                    &letter.to_string(),
                                )
                                .unwrap()
                            }
                        } else {
                            // border / filler
                            if state.in_play == r {
                                item.clone()
                            } else if letter == ' ' {
                                item.clone()
                            } else {
                                let letter_status = get_letter_status(c, guess_arr, &answer_arr);
                                terminal_style::format::background(
                                    status_to_ansi(letter_status),
                                    item,
                                )
                                .unwrap()
                            }
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        el.look.update(mapped);

        crate::elements::draw_relative(el, X, Y, state);
    }));

    board
}
