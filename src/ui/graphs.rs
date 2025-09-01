use crate::tui_engine::*;
use crate::AppState;

fn create_bar_element(x: u16, y: u16, what: &str, num: usize) -> Element<AppState> {
    let mut el: Element<AppState> = Element::new(0, 0, Look::from(""));

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
            what,
            terminal_style::format::inverse(&bar),
            terminal_style::format::inverse(&value.to_string()),
            end,
            "" // trailing padding if needed
        );

        el.look.update(vec![vec![look_str]]);

        crate::ui::draw_relative(el, x, y, state);
    }));

    el
}

pub fn build<'a>() -> Elements<'a, AppState> {
    let graphs = Elements::new();

    let stats = vec!["1", "2", "3", "4", "5", "6"];
    for (index, item) in stats.iter().enumerate() {
        let bar_element = create_bar_element(57, 4 + index as u16, item, index + 1);
        graphs.push(bar_element);
    }

    graphs
}
