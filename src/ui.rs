use terminal_style::format::*; // For styling terminal text (e.g., background, underline, inverse) // Game state and status enum

use crate::storage::save;
use crate::tui_engine::*; // Custom TUI engine for Element, Look, drawing, etc.
use crate::words::{PLAY_WORDS, VALID_WORDS}; // Word lists for game logic
use crate::{AppState, GameStatus};

const APP_WIDTH: u16 = 80; // Width of the game window
const APP_HEIGHT: u16 = 24; // Height of the game window

fn dynamic_x(relative_x: u16) -> u16 {
    let base = columns().saturating_sub(APP_WIDTH) / 2;
    base + relative_x
}

fn dynamic_y(relative_y: u16) -> u16 {
    let base = rows().saturating_sub(APP_HEIGHT) / 2;
    base + relative_y
}

fn draw_if_fits<S>(el: &Element<S>) {
    if columns() >= APP_WIDTH && rows() >= APP_HEIGHT {
        draw(el);
    }
}

// ---------------- Game Rules ---------------- //
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
fn get_letter_status(pos: usize, guess_arr: &[char], answer_arr: &[char]) -> u8 {
    wordle_highlight(guess_arr, answer_arr)
        .get(pos)
        .copied()
        .unwrap_or(0)
}

// Maps letter status to background color
fn set_background_look(letter_status: u8, item: &Look) -> Look {
    match letter_status {
        2 => background(2, item).unwrap(),
        1 => background(3, item).unwrap(),
        _ => background(8, item).unwrap(),
    }
}

// Maps letter status to background color
fn set_background(letter_status: u8, item: &str) -> String {
    match letter_status {
        2 => background(2, item).unwrap(),
        1 => background(3, item).unwrap(),
        _ => background(8, item).unwrap(),
    }
}

fn is_valid_key(key: &str) -> bool {
    key.len() == 1 && key.chars().all(|c| c.is_ascii_alphabetic())
}

// encapsulate anything that can happen in the game
fn mutate_state_on_input(el: &Element<'_, AppState>, state: &mut AppState, keypress: &str) {
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

fn mutate_state_new_game(state: &mut AppState) {
    state.game = GameStatus::InPlay; // Reset game status
    state.answer = PLAY_WORDS[state.word_index]; // Pick next answer
    state.in_play = 0; // Reset attempt index
    state.status = vec![vec![' '; 5]; 6]; // Empty 6x5 board
    state.used = Vec::new(); // Clear used letters
}

// ---------------- Animators ---------------- //
fn reveal_animator(el: &Element<AppState>, in_play: usize, status: &[Vec<char>], answer: &str) {
    // helper: color each char
    fn str_to_colored_vec(s: &str, letter_status: u8) -> Vec<String> {
        s.chars()
            .map(|c| set_background(letter_status, &c.to_string()))
            .collect()
    }

    let line_in_play = &status[in_play];
    let answer_arr: Vec<char> = answer.chars().collect();

    // Clone the full look so we always preserve the grid
    let mut full_look = el.look.clone();

    for index in 0..5 {
        let letter_status = get_letter_status(index, line_in_play, &answer_arr);
        let letter_str = line_in_play[index].to_string();

        // step 1: Flip away (clear)
        {
            let mut rows = full_look.cells().clone();

            // take the three rows out as owned clones
            let mut top = rows[in_play * 3 + 0].clone();
            let mut middle = rows[in_play * 3 + 1].clone();
            let mut bottom = rows[in_play * 3 + 2].clone();

            for i in 0..5 {
                top[index * 5 + i] = " ".to_string();
                middle[index * 5 + i] = "─".to_string();
                bottom[index * 5 + i] = " ".to_string();
            }

            // put them back
            rows[in_play * 3 + 0] = top;
            rows[in_play * 3 + 1] = middle;
            rows[in_play * 3 + 2] = bottom;

            // rebuild Look
            full_look = Look::from(rows);
            el.look.update(full_look.clone());
            draw_if_fits(el);
            std::thread::sleep(std::time::Duration::from_millis(300));
        }

        // step 2: Reveal with color
        {
            let top_pattern = str_to_colored_vec("┌───┐", letter_status);
            let middle_pattern = str_to_colored_vec(&format!("│ {} │", letter_str), letter_status);
            let bottom_pattern = str_to_colored_vec("└───┘", letter_status);

            let mut rows = full_look.cells().clone();

            // take ownership of the row triplet
            let mut top = rows[in_play * 3 + 0].clone();
            let mut middle = rows[in_play * 3 + 1].clone();
            let mut bottom = rows[in_play * 3 + 2].clone();

            for i in 0..5 {
                top[index * 5 + i] = top_pattern[i].clone();
                middle[index * 5 + i] = middle_pattern[i].clone();
                bottom[index * 5 + i] = bottom_pattern[i].clone();
            }

            // put them back
            rows[in_play * 3 + 0] = top;
            rows[in_play * 3 + 1] = middle;
            rows[in_play * 3 + 2] = bottom;

            // rebuild Look
            full_look = Look::from(rows);

            el.look.update(full_look.clone());
            draw_if_fits(el);
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }
}

fn invalid_word_animator(el: &Element<AppState>, in_play: usize) {
    // helper: shake a row
    fn shake_row(original: &[String], dx: i32) -> Vec<String> {
        let len = original.len();
        let mut new_row = original.to_vec(); // start with original

        if dx > 0 {
            for i in (0..len).rev() {
                let target = i + dx as usize;
                if target < len {
                    new_row[target] = original[i].clone();
                }
            }
        } else if dx < 0 {
            let dx = (-dx) as usize;
            for i in 0..len {
                if i >= dx {
                    new_row[i - dx] = original[i].clone();
                }
            }
        }

        new_row
    }

    // Clone the original look so we can restore it after each shake
    let full_look = el.look.clone();
    let row_index = in_play * 3;

    // Extract the three rows for this line
    let original_top = full_look.cells()[row_index + 0].clone();
    let original_middle = full_look.cells()[row_index + 1].clone();
    let original_bottom = full_look.cells()[row_index + 2].clone();

    // Shake offsets sequence
    let shake_offsets = [1, -2, 2, -2, 1, 0];

    for &dx in &shake_offsets {
        let mut temp_rows = full_look.cells().clone();

        // Overlay the shifted row triplet
        temp_rows[row_index + 0] = shake_row(&original_top, dx);
        temp_rows[row_index + 1] = shake_row(&original_middle, dx);
        temp_rows[row_index + 2] = shake_row(&original_bottom, dx);

        el.look.update(Look::from(temp_rows));
        draw_if_fits(el);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

// ---------------- Elements ---------------- //
pub fn build_elements<'a>() -> Elements<'a, AppState> {
    let elements: Elements<AppState> = Elements::new();

    // ---------------- Screen ---------------- //
    // utility element with no visible look.
    // clears screen on resize, "mouse out" of pickers.
    let mut screen: Element<AppState> = Element::new(0, 0, Look::new());

    screen.on_loop = Some(Box::new(|_el, state, _event| {
        let x = columns().saturating_sub(APP_WIDTH) / 2;
        let y = rows().saturating_sub(APP_HEIGHT) / 2;

        if x != state.app_x || y != state.app_y {
            go_to(0, 1);
            clear_below();

            state.app_x = x;
            state.app_y = y;
        }
    }));

    elements.push(screen);

    // ---------------- Size Modal ---------------- //
    let mut centered_modal: Element<AppState> =
        Element::new(0, 0, Look::from(vec![vec!["".to_string()]]));

    centered_modal.on_state = Some(Box::new(|el, _state| {
        let terminal_too_small = columns() < APP_WIDTH || rows() < APP_HEIGHT;
        let mut look_rows = Vec::new();

        if terminal_too_small {
            let rows = rows() as usize;
            let columns = columns() as usize;

            for row_idx in 0..rows {
                let mut row = vec![" ".to_string(); columns];

                if row_idx == rows / 2 {
                    let mut point = columns / 2;
                    let msg_offset = 12;
                    point = point.saturating_sub(msg_offset);

                    let msg = "Enlarge Terminal Window";
                    for (i, ch) in msg.chars().enumerate() {
                        if point + i < row.len() {
                            row[point + i] = ch.to_string();
                        }
                    }
                }

                look_rows.push(row);
            }

            el.look.update(look_rows);
            draw(el);
        } else {
            el.look.update("");
            draw(el);
        }
    }));

    elements.push(centered_modal);

    // ---------------- Title Bar ---------------- //
    let mut title_bar: Element<AppState> = Element::new(0, 0, Look::new());

    title_bar.on_state = Some(Box::new(move |el, _state| {
        let cols = crossterm::terminal::size().unwrap().0 as usize;

        let mut line = " ".repeat(cols);
        let text = "Rusty Wordle";
        line.replace_range(0..text.len().min(cols), &text);

        el.look.update(vec![vec![inverse(&line)]]);
        draw(el);
    }));

    elements.push(title_bar);

    // ---------------- Button: "Exit" ---------------- //
    let mut button_exit: Element<AppState> = Element::new(0, 0, underline(Look::from("Exit")));
    button_exit.on_keypress = Some(Box::new(|_el, state, event| {
        if event.key == Some("c".to_string()) && event.modifiers.contains(&"ctrl".to_string()) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_state = Some(Box::new(|el, _state| {
        el.x.set(columns() - 4);
        draw(el);
    }));

    elements.push(button_exit);

    // ---------------- Button: "Next" ---------------- //
    let mut button_next: Element<AppState> =
        Element::new(dynamic_x(75), dynamic_y(22), Look::from("Next"));

    button_next.on_click = Some(Box::new(|el, state, event| {
        if state.game != GameStatus::InPlay {
            if mouse_over(el, event) {
                mutate_state_new_game(state);
            }
        }
    }));
    button_next.on_state = Some(Box::new(|el, state| {
        if state.game != GameStatus::InPlay {
            el.look.update(underline(Look::from("Next")));
            // once we move to next word - save the status of the game
            save(&state.results, state.streak, state.word_index).ok();
        } else {
            el.look.update(Look::from("    "));
        }

        el.x.set(state.app_x + 75);
        el.y.set(state.app_y + 22);
        draw_if_fits(el);
    }));
    elements.push(button_next);

    // ---------------- Board ---------------- //
    let mut board: Element<AppState> = Element::new(dynamic_x(9), dynamic_y(4), Look::from(""));

    board.on_keypress = Some(Box::new(|el, state, event| {
        mutate_state_on_input(el, state, &event.key.clone().unwrap_or_default());
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
                                set_background(letter_status, &letter.to_string())
                            }
                        } else {
                            // border / filler
                            if state.in_play == r {
                                item.clone()
                            } else if letter == ' ' {
                                item.clone()
                            } else {
                                let letter_status = get_letter_status(c, guess_arr, &answer_arr);
                                set_background(letter_status, item)
                            }
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        el.look.update(mapped);
        el.x.set(state.app_x + 9);
        el.y.set(state.app_y + 4);

        draw_if_fits(el);
    }));

    elements.push(board);

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

            el.x.set(state.app_x + x);
            el.y.set(state.app_y + y);

            draw_if_fits(el);
        }));

        // --- on_click: inject letter into state ---
        el.on_click = Some(Box::new(move |el, state, event| {
            if mouse_over(el, event) {
                mutate_state_on_input(el, state, &letter.to_string());
            }
        }));

        el
    }

    // ---------------- Enter Key ---------------- //
    let mut enter_key = Element::new(
        dynamic_x(40),
        dynamic_y(19),
        Look::from(vec![
            vec!["┌", "─", "─", "─", "┐"],
            vec!["│", " ", "↩", " ", "│"],
            vec!["└", "─", "─", "─", "┘"],
        ]),
    );
    enter_key.on_state = Some(Box::new(|el, state: &AppState| {
        el.x.set(state.app_x + 40);
        el.y.set(state.app_y + 19);

        draw_if_fits(el);
    }));
    enter_key.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            mutate_state_on_input(el, state, &"enter".to_string());
        }
    }));

    elements.push(enter_key);

    // ---------------- Delete Key ---------------- //
    let mut delete_key: Element<AppState> = Element::new(
        dynamic_x(66),
        dynamic_y(19),
        Look::from(vec![
            vec!["┌", "─", "─", "┐"],
            vec!["│", " ", "⌫", "│"],
            vec!["└", "─", "─", "┘"],
        ]),
    );

    delete_key.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 66);
        el.y.set(state.app_y + 19);

        draw_if_fits(el);
    }));
    delete_key.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            mutate_state_on_input(el, state, &"delete".to_string());
        }
    }));

    elements.push(delete_key);

    pub fn build_keyboard<'a>() -> Elements<'a, AppState> {
        // Layout rows
        let keys: Vec<Vec<char>> = vec![
            vec!['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
            vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
            vec!['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
        ];

        let kb = Elements::new();

        for (line_index, key_line) in keys.iter().enumerate() {
            for (index, &letter) in key_line.iter().enumerate() {
                // replicate Crumb's (add (* line_index 2) (/ line_index 2) (* index 3))
                let x = 40 + line_index as u16 * 2 + (line_index as u16 / 2) + (index as u16 * 3);
                let y = 13 + line_index as u16 * 3;

                kb.push(create_key(x, y, letter));
            }
        }

        kb
    }

    elements.extend(build_keyboard());

    let mut answer: Element<AppState> = Element::new(
        dynamic_x(30),
        dynamic_y(2),
        Look::from(vec![vec!["Wordle #1 ?????".to_string()]]),
    );

    answer.on_state = Some(Box::new(|el, state| {
        match state.game {
            GameStatus::Won => {
                let index_in_play = state.in_play;
                let word_index = state.word_index;
                let mapped = match index_in_play {
                    1 => format!("Wordle #{} {}", word_index, inverse("Genius")),
                    2 => format!("Wordle #{} {}", word_index, inverse("Magnificent")),
                    3 => format!("Wordle #{} {}", word_index, inverse("Impressive")),
                    4 => format!("Wordle #{} {}", word_index, inverse("Splendid")),
                    5 => format!("Wordle #{} {}", word_index, inverse("Great")),
                    6 => format!("Wordle #{} {}", word_index, inverse("Phew")),
                    _ => format!("Wordle #{} {}", word_index, inverse("???")),
                };

                el.look.update(vec![vec![mapped]]);

                // then show the actual answer
                let final_mapped = format!("Wordle #{} {}", state.word_index, state.answer);
                el.look.update(vec![vec![final_mapped]]);
            }
            GameStatus::Lost => {
                let mapped = format!("Wordle #{} {}", state.word_index, inverse(state.answer));
                el.look.update(vec![vec![mapped]]);
            }
            GameStatus::InPlay => {
                let mapped = format!("Wordle #{} {}", state.word_index + 1, "?????");
                el.look.update(vec![vec![mapped]]);
            }
        }

        el.x.set(state.app_x + 30);
        el.y.set(state.app_y + 2);
        draw_if_fits(el);
    }));

    elements.push(answer);

    let mut results: Element<AppState> = Element::new(
        dynamic_x(40),
        dynamic_y(4),
        Look::from(vec![
            vec!["Played: 0".to_string()],
            vec!["Win %: 0".to_string()],
            vec!["Streak: 0".to_string()],
            vec!["Max Streak: 0".to_string()],
        ]),
    );

    results.on_state = Some(Box::new(|el, state| {
        let r = &state.results;
        let p: u32 = r.iter().sum();

        let win_percent = if p > 0 {
            (((p - r[6]) as f32 / p as f32) * 100.0).round() as u32
        } else {
            0
        };

        let look = vec![
            vec![format!("Played: {}", p)],
            vec![format!("Win %: {}", win_percent)],
            vec![format!("Streak: {}  ", state.streak.0)],
            vec![format!("Max Streak: {}", state.streak.1)],
        ];

        el.x.set(state.app_x + 40);
        el.y.set(state.app_y + 4);
        el.look.update(look);

        draw_if_fits(el);
    }));

    elements.push(results);

    fn create_bar_element(x: u16, y: u16, what: &str, num: usize) -> Element<AppState> {
        let mut el: Element<AppState> = Element::new(
            dynamic_x(x + 57),
            dynamic_y(y + 4),
            Look::from(vec![vec![format!(
                "{} {}{}",
                what,
                inverse(""),
                inverse(&num.to_string())
            )]]),
        );

        el.on_state = Some(Box::new(move |el, state| {
            let index = num - 1; // now `num` is owned by the closure

            // ignore losses (results[6])
            let max = state.results[..6].iter().cloned().max().unwrap_or(0);
            let value = state.results[index];

            let len = if max > 0 {
                ((value as f32 / max as f32) * 10.0).round() as usize
            } else {
                0
            };

            let bar: String = " ".repeat(len);
            let end: String = " ".repeat(11usize.saturating_sub(len));

            let look_str = format!(
                "{} {}{}{}{}",
                index + 1,
                inverse(&bar),
                inverse(&value.to_string()),
                end,
                "" // trailing padding if needed
            );

            el.x.set(state.app_x + x);
            el.y.set(state.app_y + y);
            el.look.update(vec![vec![look_str]]);
            draw_if_fits(el);
        }));

        el
    }

    pub fn build_graphs<'a>() -> Elements<'a, AppState> {
        let graphs = Elements::new();

        let stats = vec!["1", "2", "3", "4", "5", "6"];
        for (index, item) in stats.iter().enumerate() {
            let bar_element = create_bar_element(57, 4 + index as u16, item, index + 1);
            graphs.push(bar_element);
        }

        graphs
    }

    elements.extend(build_graphs());

    elements
}
