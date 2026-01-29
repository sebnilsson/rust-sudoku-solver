use std::cell::RefCell;

mod board;
mod board_info;
mod board_parser;
mod board_solver;
mod cell;
mod coordinate;
mod number;
mod region;

pub const BOARD_WIDTH: u8 = 9;

pub fn solve(sudoku_content: &str, context: &mut SolveContext) {
    let mut board = Board::parse(sudoku_content);

    board_solver::solve(&mut board, context);
}

pub struct SolveContext {
    pub callback: fn(&Board, &usize),
    pub complete_callback: fn(&Board, &usize),
    pub solve_count: usize,
    pub use_random: bool,
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<BoardCell>,
}

#[derive(Debug)]
pub struct BoardInfo<'a> {
    board: &'a Board,
    columns: Vec<Region<'a>>,
    rows: Vec<Region<'a>>,
    subgrids: Vec<Region<'a>>,
}

#[derive(Debug)]
pub struct Region<'a> {
    cells: Vec<&'a BoardCell>,
}

type BoardCell = RefCell<Cell>;

#[derive(Debug)]
pub struct Cell {
    coordinate: Coordinate,
    num: Number,
    is_template: bool,
}

#[derive(Copy, Clone, Debug, Ord, Eq)]
pub struct Coordinate {
    x: u8,
    y: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Ord, Eq)]
pub enum Number {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}
