use super::*;

pub fn solve(
    board: &mut Board,
    callback: fn(&Board),
    complete_callback: fn(&Board),
) {
    let mut last_unsolved_count = board.unsolved_count();

    loop {
        solve_board(board);

        let mut unsolved_count = board.unsolved_count();

        if unsolved_count == 0 {
            validate_solution(board, complete_callback);
            return;
        }

        if unsolved_count == last_unsolved_count {
            println!("--- Failed to solve logically, trying brute force ---");
            println!();

            loop {
                solve_board_fill(board);

                unsolved_count = board.unsolved_count();

                if unsolved_count == 0 {
                    validate_solution(board, complete_callback);
                    return;
                }

                // if unsolved_count == last_unsolved_count {
                //     panic!(
                //         "Failed to improve unsolved cells. Stuck at: {}.",
                //         unsolved_count
                //     );
                // }

                //last_unsolved_count = unsolved_count;
            }
        }

        if unsolved_count > 0 {
            callback(board);
        }

        last_unsolved_count = unsolved_count;
    }
}

fn solve_board_fill(board: &mut Board) {
    board_filler::fill(board);
}

fn validate_solution(board: &Board, complete_callback: fn(&Board)) {
    // TODO: Implement validation

    complete_callback(board);
}

fn solve_board(board: &mut Board) {
    let board_info = BoardInfo::new(board);

    for row in board_info.rows.iter() {
        for cell in row.cells.iter() {
            let x;
            let y;
            {
                let cell = cell.borrow_mut();
                x = cell.x;
                y = cell.y;
            }

            let other_nums = board_info.other_nums(x, y);

            let mut cell = cell.borrow_mut();
            cell.update_options(other_nums);
        }
    }
}
