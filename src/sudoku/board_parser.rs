use super::*;

pub fn fill(board: &mut Board, sudoku_content: String) {
    let sudoku_lines: Vec<_> = sudoku_content.lines().collect();

    for x in 0..BOARD_WIDTH as usize {
        let row_text = sudoku_lines.get(x).to_owned().unwrap_or(&"");
        let numbers: Vec<_> = row_text.split_whitespace().collect();

        for y in 0..BOARD_WIDTH as usize {
            let cell = board.find_cell_mut(x as u8, y as u8);

            let number = *numbers.get(y).unwrap_or_else(|| &"");

            cell.set_template(number);
        }
    }
}
