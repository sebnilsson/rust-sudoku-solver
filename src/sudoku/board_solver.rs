use super::Board;
use super::Region;

pub fn solve(board: &mut Board, callback: fn(&Board)) {
    let columns = board.columns();
    let rows = board.rows();
    let regions = board.regions();

    let mut last_unsolved_count = 0;

    loop {
        solve_columns(&columns);
        solve_rows(&rows);
        solve_regions(&regions);

        let unsolved_count = board.unsolved_count();

        if unsolved_count == 0 {
            break;
        }

        if unsolved_count == last_unsolved_count {
            panic!(
                "Failed to improve unsolved cells. Stuck at: {}.",
                unsolved_count
            );
        }

        last_unsolved_count = unsolved_count;

        callback(&board);
    }
}

fn solve_columns(columns: &Vec<Region>) {
    solve_any_regions(columns);
}

fn solve_regions(regions: &Vec<Region>) {
    solve_any_regions(regions);
}

fn solve_rows(rows: &Vec<Region>) {
    solve_any_regions(rows);
}

fn solve_any_regions(_region: &Vec<Region>) {
    // TODO
}
