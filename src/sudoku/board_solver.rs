extern crate rand;

use super::*;

pub fn solve(board: &mut Board, callback: fn(&Board)) {
    let mut last_unsolved_count = board.unsolved_count();

    loop {
        solve_board(board);

        let /*mut*/ unsolved_count = board.unsolved_count();

        if unsolved_count == 0 {
            break;
        }

        if unsolved_count == last_unsolved_count {
            //solve_board(board);

            // try_fill_empty_cells(&mut columns, &mut rows, &mut regions);

            //unsolved_count = board.unsolved_count();

            //if unsolved_count == last_unsolved_count {
                panic!(
                    "Failed to improve unsolved cells. Stuck at: {}.",
                    unsolved_count
                );
            //}
        }

        last_unsolved_count = unsolved_count;

        callback(&board);
    }
}

fn solve_board(board: &mut Board) {
    let unsolved_cells: Vec<&BoardCell> =
        board.cells.iter().filter(|x| !x.borrow().is_solved()).collect();

    for cell in unsolved_cells {
        let nums = solve_cell(&cell, board);

        cell.borrow_mut().update_options(nums);
    }
}

fn solve_cell(cell: &BoardCell, board: &Board) -> Vec<Number> {
    let cell = cell.borrow();
    let x = cell.x;
    let y = cell.y;

    let row = board.find_row(x, y);
    let column = board.find_column(x, y);
    let region = board.find_region(x, y);

    let mut cells: Vec<&BoardCell> = row
        .cells
        .into_iter()
        .chain(column.cells.into_iter().chain(region.cells.into_iter()))
        .filter(|x| x.borrow().num != Number::N0)
        .collect();

    cells.sort_by_key(|x| x.borrow().num);
    cells.dedup_by_key(|x| x.borrow().num);

    cells.into_iter().map(|x| x.borrow().num).collect()
}
