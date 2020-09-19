mod board;
mod board_parser;
mod board_solver;
mod cell;
mod number;
mod region;

pub fn solve<'a>(sudoku_content: String) -> Board {
    let board = Board::parse(sudoku_content);

    let _last_total_potentials = -1;

    loop {
        board.solve_run();

        let total_potentials = board.total_potentials();

        if total_potentials == 0 {
            break;
        }
    }

    board
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
}

#[derive(Debug)]
pub struct Region<'a> {
    cells: Vec<&'a Cell>,
}

#[derive(Debug)]
pub struct Cell {
    x: u8,
    y: u8,
    num: Number,
    potentials: Vec<Number>,
}

#[derive(Debug)]
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
