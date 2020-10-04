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

    pub fn index(x: u8, y: u8) -> usize {
        (x + (y * BOARD_WIDTH)) as usize
    }
}

impl Board {
    pub fn find_cell_mut(&mut self, x: u8, y: u8) -> &mut BoardCell {
        find_cell_mut(&mut self.cells, x, y)
    }

    pub fn unsolved_count(&self) -> usize {
        self.cells.iter().map(|x| x.borrow()).filter(|x| !x.is_solved()).count()
    }

    pub fn find_column<'a>(&'a self, x: u8, y: u8) -> Region<'a> {
        find_region(self.columns(), x, y)
    }

    pub fn find_region<'a>(&'a self, x: u8, y: u8) -> Region<'a> {
        find_region(self.regions(), x, y)
    }

    pub fn find_row<'a>(&'a self, x: u8, y: u8) -> Region<'a> {
        find_region(self.rows(), x, y)
    }

    pub fn columns<'a>(&'a self) -> Vec<Region<'a>> {
        region_resolver::columns(&self.cells)
    }

    pub fn regions<'a>(&'a self) -> Vec<Region<'a>> {
        region_resolver::regions(&self.cells)
    }

    pub fn rows<'a>(&'a self) -> Vec<Region<'a>> {
        region_resolver::rows(&self.cells)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.rows();
        let row_count = rows.len() - 1;

        let mut s = String::new();
        let mut index = 0;

        for row in rows.iter() {
            let nums =
                row.cells.iter().map(|x| x.borrow()).map(|x| x.num.to_str());
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
            let cell = Cell::new(x, y);
            let cell = BoardCell::new(cell);
            cells.push(cell);
        }
    }

    Board { cells }
}

fn find_cell_mut(cells: &mut Vec<BoardCell>, x: u8, y: u8) -> &mut BoardCell {
    let index = Board::index(x, y);
    let cell = cells.get_mut(index);
    if cell.is_none() {
        panic!("Failed finding cell for x: {}, y: {}", x, y);
    }

    cell.unwrap()
}

fn find_region<'a>(regions: Vec<Region<'a>>, x: u8, y: u8) -> Region<'a> {
    let region = regions.into_iter().find(|region| {
        region
            .cells
            .iter()
            .map(|x| x.borrow())
            .any(|cell| cell.x == x && cell.y == y)
    });

    match region {
        Some(region) => region,
        None => panic!("Failed to find correct region"),
    }
}
