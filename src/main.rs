mod file_path;
mod sudoku;

use std::fs;
use std::time::Instant;
use sudoku::Board;
use sudoku::SolveContext;

fn main() {
    let now = Instant::now();

    println!("Solving Sudoku...");
    println!();

    let sudoku_content = sudoku_content();

    let mut callbacks = SolveContext {
        callback: board_callback,
        complete_callback: board_complete_callback,
        solve_count: 0,
    };

    sudoku::solve(sudoku_content, &mut callbacks);

    let elapsed = now.elapsed();
    println!("Time elapsed: {:.2} seconds", elapsed.as_secs_f32());
    println!();
}

fn board_callback(board: &Board, attempt: &usize) {
    println!("--- Progress (Attempt: {}) ---", attempt);
    println!("{}", board);
    println!();
}

fn board_complete_callback(board: &Board, attempt: &usize) {
    println!("=== Solved! (Attempts: {}) ===", attempt);
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

    println!("--- File content ---");
    println!("{}", file_content);
    println!();

    file_content
}

// https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
