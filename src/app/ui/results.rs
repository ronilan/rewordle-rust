use incredible::*;

use crate::{ui::draw_relative, AppState};

static X: isize = 40;
static Y: isize = 4;

pub fn build() -> Element<AppState> {
    let results: Element<AppState> = Element::new();
    results.on_state(|el, state, _event| {
        let r = &state.results;
        let p: u32 = r.iter().sum();

        let win_percent = if p > 0 {
            (((p - r[6]) as f32 / p as f32) * 100.0).round() as u32
        } else {
            0
        };

        let rows: Vec<Vec<Block>> = vec![
            format!("Played: {}", p),
            format!("Win %: {}", win_percent),
            format!("Streak: {}  ", state.streak.0),
            format!("Max Streak: {}", state.streak.1),
        ]
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| Block::new(c, Decor::default()))
                .collect()
        })
        .collect();
        el.look(Look::from(rows));
        draw_relative(el, X, Y, state);
    });
    results
}
