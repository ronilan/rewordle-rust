use incredible::*;

use crate::{
    game::{get_letter_status, mutate_state_new_word},
    ui::{
        board::{board_look, status_to_ansi, X, Y},
        reposition,
    },
    AppState, WordStatus,
};

fn bg(status: u8) -> Decor {
    let d = Decor::default();
    d.background.set(Some(Color::Ansi(status_to_ansi(status))));
    d
}

fn set_span(rows: &mut [Vec<Block>], row: usize, col: usize, cells: &[char], decor: &Decor) {
    for (i, &ch) in cells.iter().enumerate() {
        rows[row][col + i] = Block::new(ch, decor.clone());
    }
}

fn present(el: &Element<AppState>, state: &AppState, rows: Vec<Vec<Block>>) {
    el.look(Look::from(rows));
    reposition(el, X, Y, state);
    el.draw();
}

fn paint_revealed(
    rows: &mut [Vec<Block>],
    in_play: usize,
    index: usize,
    letter: char,
    letter_status: u8,
) {
    let base_row = in_play * 3;
    let base_col = index * 5;
    let colored = bg(letter_status);
    set_span(
        rows,
        base_row,
        base_col,
        &['┌', '─', '─', '─', '┐'],
        &colored,
    );
    set_span(
        rows,
        base_row + 1,
        base_col,
        &['│', ' ', letter, ' ', '│'],
        &colored,
    );
    set_span(
        rows,
        base_row + 2,
        base_col,
        &['└', '─', '─', '─', '┘'],
        &colored,
    );
}

/// Reveal is 10 half-steps of 300ms. Pure function of step: tiles before the
/// current one stay colored, matching the old frame-accumulating behavior.
pub fn render_reveal_step(el: &Element<AppState>, state: &AppState, in_play: usize, step: usize) {
    let step = step.min(9);
    let index = step / 2;
    let second_half = step % 2 == 1;

    let line_in_play = &state.status[in_play];
    let answer_arr: Vec<char> = state.answer.chars().collect();

    let mut rows = board_look(state);
    for i in 0..index {
        let ls = get_letter_status(i, line_in_play, &answer_arr);
        paint_revealed(&mut rows, in_play, i, line_in_play[i], ls);
    }

    let base_row = in_play * 3;
    let base_col = index * 5;
    if second_half {
        let ls = get_letter_status(index, line_in_play, &answer_arr);
        paint_revealed(&mut rows, in_play, index, line_in_play[index], ls);
    } else {
        let plain = Decor::default();
        set_span(
            &mut rows,
            base_row,
            base_col,
            &[' ', ' ', ' ', ' ', ' '],
            &plain,
        );
        set_span(
            &mut rows,
            base_row + 1,
            base_col,
            &['─', '─', '─', '─', '─'],
            &plain,
        );
        set_span(
            &mut rows,
            base_row + 2,
            base_col,
            &[' ', ' ', ' ', ' ', ' '],
            &plain,
        );
    }
    present(el, state, rows);
}

const SHAKE_OFFSETS: [i32; 6] = [1, -2, 2, -2, 1, 0];

/// Shake is 6 frames of 50ms. Pure function of step.
pub fn render_shake_step(el: &Element<AppState>, state: &AppState, in_play: usize, step: usize) {
    let dx = SHAKE_OFFSETS[step.min(5)];
    let row_index = in_play * 3;

    let blank = BlockContent::new(" ");
    let original: Vec<Vec<(BlockContent, Decor)>> = (0..3)
        .map(|k| {
            board_look(state)[row_index + k]
                .iter()
                .map(|b| (b.content.get().unwrap_or(blank), b.decor.clone()))
                .collect()
        })
        .collect();

    fn shake_row(original: &[(BlockContent, Decor)], dx: i32) -> Vec<(BlockContent, Decor)> {
        let len = original.len();
        let mut new_row = original.to_vec();
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

    let mut rows = board_look(state);
    for k in 0..3 {
        let shifted = shake_row(&original[k], dx);
        for (i, (content, decor)) in shifted.into_iter().enumerate() {
            rows[row_index + k][i] = Block::new(content, decor);
        }
    }
    present(el, state, rows);
}

/// Drives the reveal effect across loop ticks. Attaches a 3000ms animation on
/// first call, renders one step per tick, finalizes when complete.
pub fn drive_reveal(el: &Element<AppState>, state: &mut AppState) {
    if el.get_animation().is_none() {
        el.animation(Some(Animation::new(3000.0, 300.0, 1.0)));
        return;
    }
    if let Some(anim) = el.get_animation() {
        if let Some(p) = anim.progress {
            let in_play = state.in_play;
            render_reveal_step(el, state, in_play, (p * 10.0) as usize);
        } else if anim.last_progress == Some(1.0) {
            el.animation(None);
            state.word_status = WordStatus::InPlay;
            mutate_state_new_word(state);
        }
    }
}

/// Drives the shake effect across loop ticks (300ms total).
pub fn drive_shake(el: &Element<AppState>, state: &mut AppState) {
    if el.get_animation().is_none() {
        el.animation(Some(Animation::new(300.0, 50.0, 1.0)));
        return;
    }
    if let Some(anim) = el.get_animation() {
        if let Some(p) = anim.progress {
            let in_play = state.in_play;
            render_shake_step(el, state, in_play, (p * 6.0) as usize);
        } else if anim.last_progress == Some(1.0) {
            el.animation(None);
            state.word_status = WordStatus::InPlay;
        }
    }
}
