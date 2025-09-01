use std::fs::{read_to_string, write};
use std::path::Path;

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

    if Path::new(".rewordle").exists() {
        match read_to_string(".rewordle") {
            Ok(status) => status.lines().map(|s| s.to_string()).collect(),
            Err(_) => default_data,
        }
    } else {
        default_data
    }
}
