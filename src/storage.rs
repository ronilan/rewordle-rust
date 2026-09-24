use std::fs::{read_to_string, write};
use std::path::PathBuf;

const DATA_DIR: &str = "rewordle";
const DATA_FILE: &str = ".rewordle";

/// Per-user stats file location, following platform conventions
/// (macOS: ~/Library/Application Support, Linux: $XDG_DATA_HOME,
/// Windows: %APPDATA%). `None` when no base data directory can be
/// determined (e.g. HOME is unset).
fn data_file_path() -> Option<PathBuf> {
    Some(dirs::data_dir()?.join(DATA_DIR).join(DATA_FILE))
}

fn is_number(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn load_rewordle(default_data: Vec<String>) -> Vec<String> {
    // three exact formats: "0", "0:0", "0:0:0:0:0:0:0"
    let valid = |s: &String| {
        let parts: Vec<&str> = s.split(':').collect();
        matches!(parts.len(), 1 | 2 | 7) && parts.iter().all(|p| is_number(p))
    };
    let content = data_file_path().and_then(|path| read_to_string(path).ok());

    match content {
        Some(status) => {
            let lines: Vec<String> = status.lines().map(|s| s.trim().to_string()).collect();

            let all_valid = lines.iter().all(valid);

            if all_valid {
                lines
            } else {
                default_data
            }
        }
        None => default_data,
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
    match data_file_path() {
        Some(path) => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            write(path, content)
        }
        // Nowhere to store: keep playing, just don't persist.
        None => Ok(()),
    }
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
