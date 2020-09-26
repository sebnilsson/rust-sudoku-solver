use super::*;

pub fn solve(board: &mut Board, callback: fn(&Board)) {
    let mut columns = board.columns();
    let mut rows = board.rows();
    let mut regions = board.regions();

    let mut last_unsolved_count = board.unsolved_count();

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
        let solved_cells: Vec<&&BoardCell> =
            region.cells.iter().filter(|x| x.borrow().is_solved()).collect();

        let mut unsolved_cells: Vec<&&BoardCell> =
            region.cells.iter().filter(|x| !x.borrow().is_solved()).collect();

        unsolved_cells.sort_by(|a, b| {
            a.borrow().options.len().cmp(&b.borrow().options.len())
        });

        for cell in unsolved_cells.iter() {
            remove_solved_cells(cell, &solved_cells);
            try_solve_cell(&mut cell.borrow_mut());
        }
    }
}

fn remove_solved_cells(cell: &BoardCell, solved_cells: &Vec<&&BoardCell>) {
    let mut cell = cell.borrow_mut();
    for solved in solved_cells {
        let solved = solved.borrow();
        cell.try_remove_option(&solved.num);
    }
}

fn try_solve_cell(cell: &mut Cell) {
    let mut num = Number::N0;
    if cell.options.len() == 1 {
        let option = cell.options.first().unwrap_or(&Number::N0);
        num = Number::from_ref(option);
    }

    if num != Number::N0 {
        cell.num = num;
    }
}
