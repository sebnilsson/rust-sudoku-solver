use std::cell::RefCell;

mod board;
mod board_filler;
mod board_info;
mod board_parser;
mod board_solver;
mod cell;
mod number;
mod region;

pub const BOARD_WIDTH: u8 = 9;

pub fn solve(
    sudoku_content: String,
    callback: fn(&Board),
    complete_callback: fn(&Board),
) {
    let mut board = Board::parse(sudoku_content);

    board_solver::solve(&mut board, callback, complete_callback);
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
    regions: Vec<Region<'a>>,
}

#[derive(Debug)]
pub struct Region<'a> {
    cells: Vec<&'a BoardCell>,
}

type BoardCell = RefCell<Cell>;

#[derive(Debug)]
pub struct Cell {
    x: u8,
    y: u8,
    num: Number,
    options: Vec<Number>,
    template: bool,
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
