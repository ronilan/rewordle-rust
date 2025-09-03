mod event_loop;
mod tui_engine;

mod rewordle_screen;
mod game;
mod storage;
mod ui;
mod words;

use crate::storage::read;
use crate::tui_engine::Elements;
use crate::words::PLAY_WORDS;

#[derive(Clone, Debug, PartialEq)]
enum WordStatus {
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
    app_x: u16,
    app_y: u16,
    game: GameStatus,
    word_index: usize,      // the index of the played from the list of words
    answer: &'static str,   // the correct answer
    status: Vec<Vec<char>>, // each line is Vec<char>, " " for empty slots
    in_play: usize,         // current attempt number (0-5)
    used: Vec<char>,        // letters already guessed
    results: Vec<u32>,      // results[0..6], wins per attempt, results[6] for losses
    streak: (u32, u32),     // (current_streak, max_streak)
    exit_flag: bool,
    word_status: WordStatus,
}

fn exit_ui(state: &AppState) -> bool {
    // exit UI loop when exit_flag is true
    state.exit_flag
}

fn main() {
    // read from storage
    let from_storage = read();

    // stats from storage
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

    // last word played
    let word_index: usize = from_storage[2].parse::<usize>().unwrap_or(0);

    let state = AppState {
        app_x: 0,
        app_y: 0,
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
    };
    let elements: Elements<'_, AppState> = crate::rewordle_screen::build();

    tui_engine::run(state, elements, None, Some(&exit_ui));
}
