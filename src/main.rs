mod file_path;
mod sudoku;

use std::fs;
use sudoku::Board;

fn main() {
    println!("Solving Sudoku...");
    println!();

    let sudoku_content = sudoku_content();
    if sudoku_content.is_none() {
        return;
    }

    let sudoku_content = sudoku_content.unwrap();

    let board = sudoku::solve(sudoku_content, board_callback);

    println!("Solved:");
    println!("{}", board);
}

fn board_callback(board: &Board) {
    println!("-> Progress:");
    println!("{}", board);
    println!();
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
    println!();

    Some(file_content)
}
