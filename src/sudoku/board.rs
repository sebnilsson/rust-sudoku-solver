use std::fmt;
use std::ops::Range;

use super::*;

impl Board {
    pub fn new() -> Self {
        new_board()
    }

    pub fn parse(sudoku_content: String) -> Self {
        let board = new_board();
        board_parser::fill(&board, sudoku_content);

        board
    }
}

impl Board {
    pub fn find_cell<'a>(&'a self, x: u8, y: u8) -> &'a Cell {
        &find_cell(&self.cells, x, y)
    }

    pub fn total_potentials(&self) -> usize {
        let potentials = self.cells.iter().map(|x| x.potentials.len());
        potentials.sum()
    }

    pub fn solve_run(&self) {
        board_solver::solve(self);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for row in self.rows.iter() {
            let nums = row.cells.iter().map(|x| x.num.to_str());
            let nums: Vec<String> = nums.collect();

            let r = nums.join(" ");
            let r = r.as_str();
            result.push_str(r);
        }

        write!(f, "{}", result)
    }
}

fn find_cell<'a>(cells: &'a Vec<Cell>, x: u8, y: u8) -> &'a Cell {
    let index = index(x, y);
    let cell = cells.get(index);
    if cell.is_none() {
        panic!("Failed finding cell for x: {}, y: {}", x, y);
    }

    cell.unwrap()
}

fn new_board() -> Board {
    let cells = cells();

    let columns = columns(&cells);
    let rows = rows(&cells);
    let regions = regions(&cells);

    Board { cells, columns, regions, rows }
}

fn cells() -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();

    for x in 0..9 {
        for y in 0..9 {
            let cell = Cell::new(x, y);
            cells.push(cell);
        }
    }

    cells
}

fn columns(cells: &Vec<Cell>) -> Vec<Region> {
    let mut columns: Vec<Region> = Vec::new();

    for y in 0..9 {
        let mut column_cells: Vec<Cell> = Vec::new();

        for x in 0..9 {
            let cell = find_cell(&cells, x, y);
            column_cells.push(*cell);
        }

        let column = Region::from(column_cells);
        columns.push(column);
    }

    columns
}

fn index(x: u8, y: u8) -> usize {
    (x + (y * 9)) as usize
}

fn regions(cells: &Vec<Cell>) -> Vec<Region> {
    (0..9).map(|x| region(cells, x)).collect()
}

fn region(cells: &Vec<Cell>, region_index: usize) -> Region {
    // TODO: Make algo much smarter
    let region_indexes = region_indexes(region_index);

    let cells1 = &cells[region_indexes.0];
    let cells2 = &cells[region_indexes.1];
    let cells3 = &cells[region_indexes.2];

    let mut region_cells: Vec<Cell> = Vec::new();
    cells1.iter().for_each(|x|region_cells.push(*x));
    cells2.iter().for_each(|x|region_cells.push(*x));
    cells3.iter().for_each(|x|region_cells.push(*x));

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

fn rows(cells: &Vec<Cell>) -> Vec<Region> {
    let mut rows: Vec<Region> = Vec::new();

    for x in 0..9 {
        let mut row_cells: Vec<&Cell> = Vec::new();

        for y in 0..9 {
            let cell = find_cell(&cells, x, y);
            row_cells.push(cell);
        }

        let row = Region::from(&row_cells);
        rows.push(row);
    }

    rows

    // let sudoku_lines: Vec<&str> = sudoku_content.lines().collect();
    // let mut rows: Vec<Region> = Vec::new();

    // for x in 0..9 {
    //     let row_cells: Vec<Cell> = Vec::new();
    //     let row_text = sudoku_lines.get(x).to_owned().unwrap_or(&"");
    //     let numbers: Vec<&str> = row_text.split_whitespace().collect();

    //     for y in 0..9_usize {
    //         let number = numbers.get(y).unwrap_or_else(|| &"");

    //         let mut cell = find_cell(&cells, x as u8, y as u8);
    //         cell.update(number);
    //     }

    //     let row = Region::from(row_cells);
    //     rows.push(row);
    // }

    // rows
}
