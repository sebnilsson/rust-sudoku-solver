use std::fmt;
use std::ops::Range;

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
    pub fn find_cell_mut(&mut self, x: u8, y: u8) -> &mut BoardCell {
        find_cell_mut(&mut self.cells, x, y)
    }

    pub fn unsolved_count(&self) -> usize {
        self.cells.iter().filter(|x| !x.borrow().is_solved()).count()
    }

    pub fn columns(&self) -> Vec<Region> {
        columns(&self.cells)
    }

    pub fn regions(&self) -> Vec<Region> {
        regions(&self.cells)
    }

    pub fn rows(&self) -> Vec<Region> {
        rows(&self.cells)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.rows();
        let row_count = rows.len() - 1;

        let mut s = String::new();
        let mut index = 0;

        for row in rows.iter() {
            let nums = row.cells.iter().map(|x| x.borrow().num.to_str());
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

fn find_cell<'a>(cells: &'a Vec<BoardCell>, x: u8, y: u8) -> &'a BoardCell {
    let index = index(x, y);
    let cell = cells.get(index);
    if cell.is_none() {
        panic!("Failed finding cell for x: {}, y: {}", x, y);
    }

    cell.unwrap()
}

fn find_cell_mut<'a>(
    cells: &'a mut Vec<BoardCell>,
    x: u8,
    y: u8,
) -> &'a mut BoardCell {
    let index = index(x, y);
    let cell = cells.get_mut(index);
    if cell.is_none() {
        panic!("Failed finding cell for x: {}, y: {}", x, y);
    }

    cell.unwrap()
}

fn new_board() -> Board {
    let cells: Vec<_> = cells();

    Board { cells }
}

fn cells<'a>() -> Vec<BoardCell> {
    let mut cells: Vec<_> = Vec::new();

    for x in 0..REGION_SIZE {
        for y in 0..REGION_SIZE {
            let cell = Cell::new(x, y);
            let cell = BoardCell::new(cell);
            cells.push(cell);
        }
    }

    cells
}

fn columns<'a>(cells: &'a Vec<BoardCell>) -> Vec<Region<'a>> {
    let mut columns: Vec<_> = Vec::new();

    for y in 0..REGION_SIZE {
        let mut column_cells: Vec<_> = Vec::new();

        for x in 0..REGION_SIZE {
            let cell = find_cell(cells, x, y);
            column_cells.push(cell);
        }

        let column = Region::from(column_cells);
        columns.push(column);
    }

    columns
}

fn index(x: u8, y: u8) -> usize {
    (x + (y * REGION_SIZE)) as usize
}

fn regions<'a>(cells: &'a Vec<BoardCell>) -> Vec<Region<'a>> {
    (0..REGION_SIZE as usize).map(|x| region(cells, x)).collect()
}

fn region<'a>(cells: &'a Vec<BoardCell>, region_index: usize) -> Region<'a> {
    // TODO: Make algo much smarter
    let region_indexes = region_indexes(region_index);

    let cells1 = &cells[region_indexes.0];
    let cells2 = &cells[region_indexes.1];
    let cells3 = &cells[region_indexes.2];

    let mut region_cells: Vec<_> = Vec::new();
    cells1.iter().for_each(|x| region_cells.push(x));
    cells2.iter().for_each(|x| region_cells.push(x));
    cells3.iter().for_each(|x| region_cells.push(x));

    Region::from(region_cells)
}

fn region_indexes(
    region_index: usize,
) -> (Range<usize>, Range<usize>, Range<usize>) {
    // TODO: Make this smarter
    if region_index > 8 {
        return (0..0, 0..0, 0..0);
    }

    let start = 3 * region_index;

    let r1 = start..(start + 3);
    let r2 = (start + 9)..(start + 9 + 3);
    let r3 = (start + 18)..(start + 18 + 3);

    (r1, r2, r3)
}

fn rows<'a>(cells: &'a Vec<BoardCell>) -> Vec<Region<'a>> {
    let mut rows: Vec<_> = Vec::new();

    for x in 0..REGION_SIZE {
        let mut row_cells: Vec<_> = Vec::new();

        for y in 0..REGION_SIZE {
            let cell = find_cell(cells, x, y);
            row_cells.push(cell);
        }

        let row: Region<'a> = Region::from(row_cells);
        rows.push(row);
    }

    rows
}
