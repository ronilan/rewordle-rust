use crate::tui_engine::*;
use crate::ui::animators::{invalid_word_animator, reveal_animator};
use crate::words::{PLAY_WORDS, VALID_WORDS};
use crate::{AppState, GameStatus};

// Determines letter status based on Wordle rules
fn wordle_highlight(guess_arr: &[char], answer_arr: &[char]) -> Vec<u8> {
    const EXACT: u8 = 2;
    const EXISTS: u8 = 1;
    const DOES_NOT_EXIST: u8 = 0;

    // ----- STEP 1: "Punch" correct matches out of answer_map -----
    let mut answer_map: Vec<char> = answer_arr
        .iter()
        .enumerate()
        .map(|(i, &ch)| {
            if guess_arr.get(i) == Some(&ch) {
                '-'
            } else {
                ch
            }
        })
        .collect();

    // ----- STEP 2: Build results -----
    let mut result = Vec::with_capacity(guess_arr.len());

    for (i, &guess_ch) in guess_arr.iter().enumerate() {
        if answer_arr.get(i) == Some(&guess_ch) {
            // Exact match
            result.push(EXACT);
        } else if !answer_arr.contains(&guess_ch) {
            // Letter does not exist in answer at all
            result.push(DOES_NOT_EXIST);
        } else if let Some(pos) = answer_map.iter().position(|&ch| ch == guess_ch) {
            // Letter exists elsewhere, mark it and "punch" it out of answer_map
            result.push(EXISTS);
            answer_map[pos] = '-';
        } else {
            // No remaining occurrences in answer_map
            result.push(DOES_NOT_EXIST);
        }
    }

    result
}

// Determines letter background based on Wordle rules
pub(crate) fn get_letter_status(pos: usize, guess_arr: &[char], answer_arr: &[char]) -> u8 {
    wordle_highlight(guess_arr, answer_arr)
        .get(pos)
        .copied()
        .unwrap_or(0)
}

// Maps letter status to background color
pub(crate) fn set_background_look(letter_status: u8, item: &Look) -> Look {
    match letter_status {
        2 => terminal_style::format::background(2, item).unwrap(),
        1 => terminal_style::format::background(3, item).unwrap(),
        _ => terminal_style::format::background(8, item).unwrap(),
    }
}

// Maps letter status to background color
pub(crate) fn set_background(letter_status: u8, item: &str) -> String {
    match letter_status {
        2 => terminal_style::format::background(2, item).unwrap(),
        1 => terminal_style::format::background(3, item).unwrap(),
        _ => terminal_style::format::background(8, item).unwrap(),
    }
}

fn is_valid_key(key: &str) -> bool {
    key.len() == 1 && key.chars().all(|c| c.is_ascii_alphabetic())
}

// encapsulate anything that can happen in the game
pub(crate) fn mutate_state_on_input(
    el: &Element<'_, AppState>,
    state: &mut AppState,
    keypress: &str,
) {
    match state.game {
        GameStatus::InPlay => {
            let line_in_play = &mut state.status[state.in_play];
            let index = line_in_play.iter().position(|&c| c == ' ');
            let uppercase_keypress = keypress.to_uppercase();

            if is_valid_key(&uppercase_keypress) && index.is_some() {
                // Insert letter
                let idx = index.unwrap();
                line_in_play[idx] = uppercase_keypress.chars().next().unwrap();
            } else if keypress == "enter" && index.is_none() {
                // Enter pressed and line is full
                let word: String = line_in_play.iter().collect();
                if !VALID_WORDS.contains(&word.as_str()) && !PLAY_WORDS.contains(&word.as_str()) {
                    invalid_word_animator(el, state.in_play);
                    return;
                } else {
                    let status_snapshot = state.status.clone(); // avoid borrowing state here

                    let line_in_play = &mut state.status[state.in_play]; // now mutable borrow is fine

                    // TODO hacking around the fact that the Enter key can not animate the board.
                    if el.look.cells().len() > 10 {
                        reveal_animator(el, state.in_play, &status_snapshot, &state.answer);
                    }

                    if &word == state.answer {
                        // Win condition
                        state.word_index += 1;
                        state.results[state.in_play] += 1;
                        state.streak.0 += 1;
                        if state.streak.0 > state.streak.1 {
                            state.streak.1 = state.streak.0;
                        }
                        state.game = GameStatus::Won;
                    } else if state.in_play == 5 {
                        // Last attempt, lost
                        state.word_index += 1;
                        state.results[6] += 1;
                        state.streak.0 = 0;
                        state.game = GameStatus::Lost;
                    }

                    // Update used letters
                    for &ch in line_in_play.iter() {
                        if !state.used.contains(&ch) {
                            state.used.push(ch);
                        }
                    }

                    state.in_play += 1;
                }
            } else if keypress == "delete" && line_in_play[0] != ' ' {
                // Delete last letter
                let idx = index.unwrap_or(5);
                if idx > 0 {
                    line_in_play[idx - 1] = ' ';
                } else {
                    line_in_play[0] = ' ';
                }
            }
            // Any other key is ignored
        }
        _ => {
            if keypress == "enter" {
                mutate_state_new_game(state);
            }
        }
    }
}

pub(crate) fn mutate_state_new_game(state: &mut AppState) {
    state.game = GameStatus::InPlay; // Reset game status
    state.answer = PLAY_WORDS[state.word_index]; // Pick next answer
    state.in_play = 0; // Reset attempt index
    state.status = vec![vec![' '; 5]; 6]; // Empty 6x5 board
    state.used = Vec::new(); // Clear used letters
}
