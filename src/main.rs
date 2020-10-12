mod file_path;
mod sudoku;

use std::fs;
use sudoku::Board;
use sudoku::SolveContext;

fn main() {
    println!("Solving Sudoku...");
    println!();

    let sudoku_content = sudoku_content();

    let context = SolveContext {
        callback: board_callback,
        complete_callback: board_complete_callback,
    };

    sudoku::solve(sudoku_content, &context);
}

fn board_callback(board: &Board) {
    let unsolved_count = board.unsolved_count();

    println!("=== Progress (Unsolved: {}) ===", unsolved_count);
    println!("{}", board);
    println!();
}

fn board_complete_callback(board: &Board) {
    println!("Solved!");
    println!("{}", board);
    println!();
}

fn sudoku_content() -> String {
    let path = file_path::path();
    let full_path = path.to_str().expect("Failed parsing path");

    if !path.exists() || !path.is_file() {
        panic!("Failed finding file at {:?}", full_path);
    }

    println!("File found: {}", full_path);
    println!();

    let file_content =
        fs::read_to_string(full_path).expect("Failed reading file");

    println!("=== File content ===");
    println!("{}", file_content);
    println!();

    file_content
}

// https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
