use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::words::PLAY_WORDS;
use incredible::Platform;

#[derive(Clone, Debug, PartialEq)]
pub enum WordStatus {
    InPlay,
    Valid,
    Invalid,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameStatus {
    InPlay,
    Won,
    Lost,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub app_x: isize,
    pub app_y: isize,
    pub term_cols: usize,
    pub term_rows: usize,
    pub game: GameStatus,
    pub word_index: usize,
    pub answer: &'static str,
    pub status: Vec<Vec<char>>,
    pub in_play: usize,
    pub used: Vec<char>,
    pub results: Vec<u32>,
    pub streak: (u32, u32),
    pub exit_flag: bool,
    pub word_status: WordStatus,
}

fn load_progress() -> (Vec<u32>, (u32, u32), usize) {
    let from_storage = crate::storage::read();

    let results: Vec<u32> = from_storage[0]
        .split(':')
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();

    let streak: (u32, u32) = (
        from_storage[1]
            .split(':')
            .nth(0)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        from_storage[1]
            .split(':')
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
    );

    let word_index: usize = from_storage[2].parse::<usize>().unwrap_or(0);

    (results, streak, word_index)
}

pub fn initial_state() -> AppState {
    let (results, streak, word_index) = load_progress();

    // Centered up front so the first frame never flashes top-left.
    let app_x = (Platform::columns().saturating_sub(APP_WIDTH) / 2) as isize;
    let app_y = (Platform::rows().saturating_sub(APP_HEIGHT) / 2) as isize;

    AppState {
        app_x,
        app_y,
        term_cols: Platform::columns(),
        term_rows: Platform::rows(),
        game: GameStatus::InPlay,
        word_index,
        answer: PLAY_WORDS[word_index],
        status: vec![vec![' '; 5]; 6],
        in_play: 0,
        used: Vec::new(),
        results,
        streak,
        exit_flag: false,
        word_status: WordStatus::InPlay,
    }
}
