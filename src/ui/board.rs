use crate::game::*;
use crate::tui_engine::*;
use crate::AppState;

// ---------------- Board ---------------- //
pub fn build<'a>() -> Element<'a, AppState> {
    let mut board: Element<AppState> = Element::new(0, 0, Look::new());

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

        crate::ui::draw_if_fits(el);
    }));

    board
}
