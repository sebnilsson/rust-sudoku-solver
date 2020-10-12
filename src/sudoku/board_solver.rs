extern crate rand;

use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn solve(board: &mut Board, context: &SolveContext) {
    let board_info = BoardInfo::new(board);

    if !solve_board(&board_info, context) {
        panic!("Failed to solve board");
    }

    (context.complete_callback)(&board);
}

pub fn unsolved_cells<'a>(board_info: &'a BoardInfo) -> Vec<&'a BoardCell> {
    let mut unsolved: Vec<(&BoardCell, usize)> = board_info
        .board
        .cells
        .iter()
        .filter(|x| {
            let cell = x.borrow();
            !cell.is_template && !cell.is_solved()
        })
        .map(|x| {
            let potentials = board_info.cell_potentials(x);
            (x, potentials.len())
        })
        .collect();

    unsolved.shuffle(&mut thread_rng());

    unsolved.sort_by_key(|x| x.1);
    //unsolved.reverse();

    unsolved.iter().map(|x| x.0).collect()
}

pub fn solve_board(board_info: &BoardInfo, context: &SolveContext) -> bool {
    let unsolved_cells = unsolved_cells(board_info);
    let cell = unsolved_cells.first();
    if cell.is_none() {
        return true;
    }

    let cell = cell.unwrap();

    let mut nums = Number::all().clone();
    nums.shuffle(&mut thread_rng());

    for num in nums {
        if can_set_num(cell, num, board_info) {
            cell.borrow_mut().set_num(&num);

            (context.callback)(board_info.board);

            if solve_board(board_info, context) {
                return true;
            }

            cell.borrow_mut().reset();
        }
    }

    false
}

fn can_set_num(
    cell: &RefCell<Cell>,
    num: Number,
    board_info: &BoardInfo,
) -> bool {
    let potentials = board_info.cell_potentials(cell);
    potentials.contains(&num)
}
