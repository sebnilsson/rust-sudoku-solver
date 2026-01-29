use std::env;
use std::path::PathBuf;

pub fn path() -> PathBuf {
    let file_path = file_path();

    if file_path.is_absolute() {
        return file_path;
    }

    let current_dir = env::current_dir().expect("Failed to get current dir");
    current_dir.join(file_path)
}

fn file_path() -> PathBuf {
    env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("sudoku.txt"))
}
