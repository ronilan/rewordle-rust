use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 40;
static Y: u16 = 4;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut results: Element<AppState> = Element::new(0, 0, Look::new());

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

        el.look.update(look);

        crate::elements::draw_relative(el, X, Y, state);
    }));

    results
}
