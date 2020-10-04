use std::fmt;

use super::*;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

impl Board {
    pub fn parse(sudoku_content: String) -> Self {
        let mut board = new_board();
        board_parser::fill(&mut board, sudoku_content);

        board
    }
}

impl Board {
    pub fn find_cell_mut(&mut self, x: u8, y: u8) -> &mut Cell {
        find_cell_mut(&mut self.cells, x, y)
    }

    pub fn unsolved_count(&self) -> usize {
        self.cells.iter().filter(|x| !x.is_solved()).count()
    }

    pub fn cell_info<'a>(&'a self, cell: &Cell) -> CellInfo<'a> {
        board_indexer::get(&self.cells, cell)
    }

    pub fn rows<'a>(&'a self) -> Vec<Region<'a>> {
        board_indexer::rows(self)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.rows();
        let row_count = rows.len() - 1;

        let mut s = String::new();
        let mut index = 0;

        for row in rows.iter() {
            let nums = row.cells.iter().map(|x| x.num.to_str());
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

fn new_board() -> Board {
    let mut cells: Vec<Cell> = Vec::new();
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_WIDTH {
            let cell = Cell::new(x, y);
            cells.push(cell);
        }
    }

    Board { cells }
}

fn find_cell_mut(cells: &mut Vec<Cell>, x: u8, y: u8) -> &mut Cell {
    let index = board_indexer::index(x, y);
    let cell = cells.get_mut(index);
    if cell.is_none() {
        panic!("Failed finding cell for x: {}, y: {}", x, y);
    }

    cell.unwrap()
}