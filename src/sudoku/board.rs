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

    pub fn update_options(&mut self) {
        board_update_options(self);
    }

    pub fn unsolved_count(&self) -> usize {
        self.cells.iter().map(|x| x.borrow()).filter(|x| !x.is_solved()).count()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board_info = BoardInfo::new(self);
        let rows = board_info.rows.iter();
        let row_count = rows.len() - 1;

        let mut s = String::new();
        let mut index = 0;

        for row in rows {
            let nums = row.cells.iter().map(|x| x.borrow()).map(|x| {
                let s = x.num.to_str();
                if x.template {
                    format!("\x1b[42;1m{}\x1b[0m", s)
                } else {
                    s
                }
            });
            let nums: Vec<_> = nums.collect();

            let r = nums.join(" ");
            let r = r.as_str();

            s.push_str(r);
            if index != row_count {
                s.push_str(LINE_ENDING);
            }

            index += 1;
        }

        let line_ending = String::from(LINE_ENDING);
        let s = s.trim_end_matches(&line_ending);

        write!(f, "{}", s)
    }
}

fn create_board() -> Board {
    let mut cells = Vec::new();

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_WIDTH {
            let coordinate = Coordinate::new(x as u8, y as u8);
            let cell = Cell::new(coordinate);
            let ref_cell = BoardCell::new(cell);
            cells.push(ref_cell);
        }
    }

    Board { cells }
}

fn find_cell(cells: &Vec<BoardCell>, coordinate: Coordinate) -> &BoardCell {
    let index = Board::index(coordinate);
    let cell = cells.get(index);

    cell.expect(format!("Failed finding cell: {}", coordinate).as_str())
}

fn find_cell_mut(
    cells: &mut Vec<BoardCell>,
    coordinate: Coordinate,
) -> &mut BoardCell {
    let index = Board::index(coordinate);
    let cell = cells.get_mut(index);

    cell.expect(format!("Failed finding cell: {}", coordinate).as_str())
}

fn board_update_options(board: &mut Board) {
    let board_info = BoardInfo::new(board);

    for cell in board.cells.iter() {
        let coordinate;
        {
            let cell = cell.borrow_mut();
            coordinate = Coordinate::new(cell.coordinate.x, cell.coordinate.y);
        }

        let region_nums = board_info.region_nums(coordinate);

        let mut cell = cell.borrow_mut();
        cell.update_options(region_nums);
    }
}
