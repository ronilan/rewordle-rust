use incredible::*;

use crate::{ui::draw_relative, AppState};

fn create_bar_element(x: isize, y: isize, what: &'static str, num: usize) -> Element<AppState> {
    let bar: Element<AppState> = Element::new();

    bar.on_state(move |el, state, _event| {
        let index = num - 1;

        // ignore losses (results[6])
        let max = state.results[..6].iter().cloned().max().unwrap_or(0);
        let value = state.results[index];

        let len = if max > 0 {
            ((value as f32 / max as f32) * 10.0).round() as usize
        } else {
            0
        };

        let inverse = {
            let d = Decor::default();
            d.inverse.set(Some(true));
            d
        };
        let plain = Decor::default();

        let mut row: Vec<Block> = format!("{} ", what)
            .chars()
            .map(|c| Block::new(c, plain.clone()))
            .collect();
        row.extend(
            " ".repeat(len)
                .chars()
                .map(|c| Block::new(c, inverse.clone())),
        );
        row.extend(
            value
                .to_string()
                .chars()
                .map(|c| Block::new(c, inverse.clone())),
        );
        row.extend(
            " ".repeat(11usize.saturating_sub(len))
                .chars()
                .map(|c| Block::new(c, plain.clone())),
        );

        el.look(Look::from(vec![row]));
        draw_relative(el, x, y, state);
    });

    bar
}

static X: isize = 57;
static Y: isize = 4;

pub fn build() -> Vec<Element<AppState>> {
    ["1", "2", "3", "4", "5", "6"]
        .iter()
        .enumerate()
        .map(|(index, item)| create_bar_element(X, Y + index as isize, item, index + 1))
        .collect()
}
