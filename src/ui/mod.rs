pub(crate) mod answer;

pub(crate) mod board;
pub(crate) mod button_exit;
pub(crate) mod button_next;
pub(crate) mod centered_modal;
pub(crate) mod delete_key;
pub(crate) mod enter_key;
pub(crate) mod graphs;
pub(crate) mod keyboard;
pub(crate) mod results;
pub(crate) mod screen;
pub(crate) mod title;

pub(crate) mod animators;
pub(crate) mod constants;
pub(crate) mod utils;

pub(crate) mod elements;

use crate::ui::utils::*;

pub use crate::ui::elements::build;
