use super::*;

pub fn solve(board: &mut Board, callback: fn(&Board)) {
    let mut columns = board.columns();
    let mut rows = board.rows();
    let mut regions = board.regions();

    let mut last_unsolved_count = 0;

    loop {
        solve_columns(&mut columns);
        solve_rows(&mut rows);
        solve_regions(&mut regions);

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

fn solve_columns(columns: &mut Vec<Region>) {
    solve_any_regions(columns);
}

fn solve_regions(regions: &mut Vec<Region>) {
    solve_any_regions(regions);
}

fn solve_rows(rows: &mut Vec<Region>) {
    solve_any_regions(rows);
}

fn solve_any_regions(regions: &mut Vec<Region>) {
    for region in regions.iter_mut() {
        for cell in region.cells.iter_mut() {
            let cell = cell.borrow();
            
            if cell.num != Number::N0 {
                println!("({}, {}): {}", cell.x, cell.y, cell.num);
            }
        }
    }
}
