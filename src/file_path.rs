use std::env;
use std::path;

pub fn path() -> path::PathBuf {
    let file_path = file_path();

    let path = std::path::PathBuf::from(&file_path);

    if path.is_absolute() {
        return path;
    }

    let current_dir = current_dir();

    let mut path = std::path::PathBuf::from(current_dir);
    path.push(file_path);
    path
}

fn current_dir() -> String {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let current_dir = current_dir.to_str().expect("Failed to get current dir");

    String::from(current_dir)
}

fn file_path() -> String {
    let args: Vec<_> = std::env::args().collect();

    args.get(1).unwrap_or(&String::from("sudoku.txt")).to_owned()
}
