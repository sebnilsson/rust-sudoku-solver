use std::fs;

mod file_path;
mod sudoku;

fn main() {
    let sudoku_content = sudoku_content();
    if sudoku_content.is_none() {
        return;
    }

    let sudoku_content = sudoku_content.unwrap();

    let board = sudoku::solve(sudoku_content);
    println!("Result:");
    println!("{}", board);
}

fn sudoku_content() -> Option<String> {
    let path = file_path::path();
    let full_path = path.to_str().expect("Failed parsing path");

    if !path.exists() || !path.is_file() {
        println!("Failed finding file at {:?}", full_path);
        return None;
    }

    println!("File found: {}", full_path);

    let file_content =
        fs::read_to_string(full_path).expect("Failed reading file");

    println!("File content:");
    println!("{}", file_content);

    Some(file_content)
}
