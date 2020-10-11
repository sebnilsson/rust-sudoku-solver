extern crate rand;

use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn solve(
    board: &mut Board,
    callback: fn(&Board),
    complete_callback: fn(&Board),
) {
    let board_info = BoardInfo::new(board);

    let board_solved = solve_board(&board_info, callback);

    if !board_solved {
        let unsolved_count = board.unsolved_count();
        panic!(
            "Failed to improve unsolved cells. Stuck at: {}.",
            unsolved_count
        );
    }

    complete_callback(board);
}

fn solve_board<'a>(board_info: &'a BoardInfo, callback: fn(&Board)) -> bool {
    let unsolved_cell =
        board_info.board.cells.iter().find(|x| !x.borrow().is_solved());

    if unsolved_cell.is_none() {
        return true;
    }

    let unsolved_cell = unsolved_cell.unwrap();

    let mut nums = Number::all().clone();
    nums.shuffle(&mut thread_rng());


    for num in nums {
        if is_valid_num(unsolved_cell, &num, board_info) {
            unsolved_cell.borrow_mut().set_num(&num);

            if solve_board(board_info, callback) {
                callback(board_info.board);
                return true;
            }

            unsolved_cell.borrow_mut().reset();
        }
    }

    callback(board_info.board);

    return false;
}

fn is_valid_num(
    cell: &BoardCell,
    num: &Number,
    board_info: &BoardInfo,
) -> bool {
    let cell = cell.borrow();

    let region_nums = board_info.region_nums(cell.x, cell.y);

    !region_nums.contains(num)
}