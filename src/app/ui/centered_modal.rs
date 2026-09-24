use incredible::*;

use crate::{
    ui::{APP_HEIGHT, APP_WIDTH},
    AppState,
};

pub fn build() -> Element<AppState> {
    let modal: Element<AppState> = Element::new();
    modal.x(0).y(0).on_state(|el, _state, _event| {
        let too_small = Platform::columns() < APP_WIDTH || Platform::rows() < APP_HEIGHT;
        if too_small {
            let rows = Platform::rows();
            let columns = Platform::columns();
            let msg = "Enlarge Terminal Window";

            let mut look_rows = Vec::with_capacity(rows);
            for row_idx in 0..rows {
                let mut row: Vec<Block> = (0..columns)
                    .map(|_| Block::new(' ', Decor::default()))
                    .collect();
                if row_idx == rows / 2 {
                    let point = (columns / 2).saturating_sub(12);
                    for (i, ch) in msg.chars().enumerate() {
                        if point + i < row.len() {
                            row[point + i] = Block::new(ch, Decor::default());
                        }
                    }
                }
                look_rows.push(row);
            }
            el.look(Look::from(look_rows));
        } else {
            el.look(Look::from(""));
        }
        el.draw();
    });
    modal
}
