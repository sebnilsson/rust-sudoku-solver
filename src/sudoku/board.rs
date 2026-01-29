use std::fmt;

use super::*;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

impl Board {
    pub fn parse(sudoku_content: String) -> Self {
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
        let board_info = BoardInfo::new(self);
        let rows = board_info.rows.iter();
        let row_count = board_info.rows.len();

        let mut s = String::new();

        for (index, row) in rows.enumerate() {
            let nums = row.cells.iter().map(|x| x.borrow()).map(|x| {
                let s = x.num.to_str();
                if x.is_template {
                    format!("\x1b[42;1m{}\x1b[0m", s)
                } else {
                    s
                }
            });
            let nums: Vec<_> = nums.collect();

            let r = nums.join(" ");
            let r = r.as_str();

            s.push_str(r);
            if index + 1 < row_count {
                s.push_str(LINE_ENDING);
            }
        }

        let s = s.trim_end_matches(LINE_ENDING);

        write!(f, "{}", s)
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
