use crate::game::get_letter_status;
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

pub fn reveal_animator(el: &Element<AppState>, in_play: usize, status: &[Vec<char>], answer: &str) {
    // helper: color each char
    fn str_to_colored_vec(s: &str, letter_status: u8) -> Vec<String> {
        s.chars()
            .map(|c| {
                terminal_style::format::background(status_to_ansi(letter_status), &c.to_string())
                    .unwrap()
            })
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
            draw(el);
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
            draw(el);
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }
}

pub fn invalid_word_animator(el: &Element<AppState>, in_play: usize) {
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
        draw(el);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
