use regex::Regex;
use std::fs::{read_to_string, write};
use std::path::Path;

fn load_rewordle(default_data: Vec<String>) -> Vec<String> {
    // three exact formats
    let re_one = Regex::new(r"^\d+$").unwrap();
    let re_two = Regex::new(r"^\d+:\d+$").unwrap();
    let re_seven = Regex::new(r"^\d+(:\d+){6}$").unwrap();

    if Path::new(".rewordle").exists() {
        match read_to_string(".rewordle") {
            Ok(status) => {
                let lines: Vec<String> = status.lines().map(|s| s.trim().to_string()).collect();

                let all_valid = lines
                    .iter()
                    .all(|s| re_one.is_match(s) || re_two.is_match(s) || re_seven.is_match(s));

                if all_valid {
                    lines
                } else {
                    default_data
                }
            }
            Err(_) => default_data,
        }
    } else {
        default_data
    }
}

pub fn save(results: &[u32], streak: (u32, u32), word_index: usize) -> std::io::Result<()> {
    let content = format!(
        "{}\n{}:{}\n{}",
        results
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(":"),
        streak.0,
        streak.1,
        word_index
    );
    write(".rewordle", content)
}

pub fn read() -> Vec<String> {
    let default_data: Vec<String> = vec![
        "0:0:0:0:0:0:0".to_string(),
        "0:0".to_string(),
        "0".to_string(),
    ];

    let data = load_rewordle(default_data);

    data
}
