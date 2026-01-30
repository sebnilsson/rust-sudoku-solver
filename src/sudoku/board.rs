use std::fmt;

use super::*;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

impl Board {
    pub fn parse(sudoku_content: &str) -> Self {
        let mut board = create_board();
        board_parser::fill(&mut board, sudoku_content);

        board
    }
}

impl Board {
    pub fn index(coordinate: Coordinate) -> usize {
        (coordinate.x + (coordinate.y * BOARD_WIDTH)) as usize
    }
}

impl Board {
    pub fn find_cell_mut(&mut self, coordinate: Coordinate) -> &mut BoardCell {
        find_cell_mut(&mut self.cells, coordinate)
    }

    pub fn find_cell(&self, coordinate: Coordinate) -> &BoardCell {
        find_cell(&self.cells, coordinate)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board_info = BoardInfo::new();
        let row_count = board_info.rows.len();

        for (row_index, row) in board_info.rows.iter().enumerate() {
            for (cell_index, coordinate) in row.cells.iter().enumerate() {
                if cell_index > 0 {
                    write!(f, " ")?;
                }

                let cell = self.find_cell(*coordinate);
                let cell = cell.borrow();
                let s = cell.num.to_str();
                if cell.is_template {
                    write!(f, "\x1b[42;1m{}\x1b[0m", s)?;
                } else {
                    write!(f, "{}", s)?;
                }
            }

            if row_index + 1 < row_count {
                write!(f, "{}", LINE_ENDING)?;
            }
        }

        Ok(())
    }
}

fn create_board() -> Board {
    let mut cells = Vec::new();

    for y in 0..BOARD_WIDTH {
        for x in 0..BOARD_WIDTH {
            let coordinate = Coordinate::new(x as u8, y as u8);
            let cell = Cell::new(coordinate);
            let ref_cell = BoardCell::new(cell);
            cells.push(ref_cell);
        }
    }

    Board { cells }
}

fn find_cell(cells: &[BoardCell], coordinate: Coordinate) -> &BoardCell {
    let index = Board::index(coordinate);
    cells
        .get(index)
        .unwrap_or_else(|| panic!("Failed finding cell: {}", coordinate))
}

fn find_cell_mut(
    cells: &mut [BoardCell],
    coordinate: Coordinate,
) -> &mut BoardCell {
    let index = Board::index(coordinate);
    cells
        .get_mut(index)
        .unwrap_or_else(|| panic!("Failed finding cell: {}", coordinate))
}
