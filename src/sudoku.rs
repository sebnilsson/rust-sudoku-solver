use std::cell::RefCell;

mod board;
mod board_parser;
mod board_solver;
mod cell;
mod number;
mod region;

pub const REGION_SIZE: u8 = 9;

pub fn solve(sudoku_content: String, callback: fn(&Board)) -> Board {
    let mut board = Board::parse(sudoku_content);

    board_solver::solve(&mut board, callback);

    board
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<BoardCell>,
}

#[derive(Debug)]
pub struct Region<'a> {
    cells: Vec<&'a BoardCell>,
}

pub type BoardCell = RefCell<Cell>;

#[derive(Debug)]
pub struct Cell {
    x: u8,
    y: u8,
    num: Number,
    options: Vec<Number>,
    template: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
