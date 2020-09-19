use super::*;

pub fn fill(board: &mut Board, sudoku_content: String) {
    let sudoku_lines: Vec<&str> = sudoku_content.lines().collect();

    for x in 0..9 {
        let row_text = sudoku_lines.get(x).to_owned().unwrap_or(&"");
        let numbers: Vec<&str> = row_text.split_whitespace().collect();

        for y in 0..9_usize {
            let cell = board.find_cell_mut(x as u8, y as u8);

            let number = *numbers.get(y).unwrap_or_else(|| &"");

            cell.update(number);
        }
    }
}
