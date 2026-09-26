//! Browser persistence backend: same three-line format as the native
//! stats file, kept in `localStorage` under the `.rewordle` key.

const STORAGE_KEY: &str = ".rewordle";

fn is_number(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn load_rewordle(default_data: Vec<String>) -> Vec<String> {
    // three exact formats: "0", "0:0", "0:0:0:0:0:0:0"
    let valid = |s: &String| {
        let parts: Vec<&str> = s.split(':').collect();
        matches!(parts.len(), 1 | 2 | 7) && parts.iter().all(|p| is_number(p))
    };

    let content = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten());

    match content {
        Some(status) => {
            let lines: Vec<String> = status.lines().map(|s| s.trim().to_string()).collect();

            if lines.iter().all(valid) {
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
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.set_item(STORAGE_KEY, &content).ok())
        .map(|_| ())
        .ok_or_else(|| std::io::Error::other("localStorage unavailable"))
}

pub fn read() -> Vec<String> {
    let default_data: Vec<String> = vec![
        "0:0:0:0:0:0:0".to_string(),
        "0:0".to_string(),
        "0".to_string(),
    ];

    load_rewordle(default_data)
}
