use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn solve(board: &mut Board, context: &mut SolveContext) {
    let board_info = BoardInfo::new(board);

    if !solve_board(&board_info, context) {
        panic!("Failed to solve board");
    }

    (context.complete_callback)(&board, &context.solve_count);
}

fn can_set_num(cell: &BoardCell, num: Number, board_info: &BoardInfo) -> bool {
    let potentials = board_info.cell_potentials(cell);
    potentials.contains(&num)
}

fn solve_board(board_info: &BoardInfo, context: &mut SolveContext) -> bool {
    let unsolved_cells = unsolved_cells(board_info, context);
    let Some(cell) = unsolved_cells.first() else {
        return true;
    };

    let mut nums = Number::all().to_vec();
    if context.use_random {
        nums.shuffle(&mut thread_rng());
    }

    for num in nums {
        if can_set_num(cell, num, board_info) {
            cell.borrow_mut().set_num(num);

            context.solve_count += 1;

            (context.callback)(board_info.board, &context.solve_count);

            if solve_board(board_info, context) {
                return true;
            }

            cell.borrow_mut().reset();
        }
    }

    false
}

fn unsolved_cells<'a>(
    board_info: &'a BoardInfo,
    context: &SolveContext,
) -> Vec<&'a BoardCell> {
    let mut unsolved: Vec<(&BoardCell, usize)> = board_info
        .board
        .cells
        .iter()
        .filter(|x| {
            let cell = x.borrow();
            !cell.is_template && !cell.is_filled()
        })
        .map(|x| {
            let potentials = board_info.cell_potentials(x);
            (x, potentials.len())
        })
        .collect();

    if context.use_random {
        unsolved.shuffle(&mut thread_rng());
    }
    unsolved.sort_by_key(|x| x.1);
    unsolved.iter().map(|(cell, _)| *cell).collect()
}
