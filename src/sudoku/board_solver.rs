extern crate rand;

use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn solve(board: &mut Board, callback: fn(&Board)) {
    let mut last_unsolved_count = board.unsolved_count();

    loop {
        // TODO: Consider removing
        solve_board(board);

        let mut unsolved_count = board.unsolved_count();

        if unsolved_count == 0 {
            break;
        }

        if unsolved_count == last_unsolved_count {
            fill_empty_cells(board);

            unsolved_count = board.unsolved_count();

            if unsolved_count == last_unsolved_count {
                panic!(
                    "Failed to improve unsolved cells. Stuck at: {}.",
                    unsolved_count
                );
            }
        }

        last_unsolved_count = unsolved_count;

        if unsolved_count > 0 {
            callback(&board);
        }
    }
}

fn solve_board(board: &mut Board) {
    let unsolved_cells = unsolved_cells(board);

    for cell in unsolved_cells {
        let region_numbers = region_numbers(&cell, board);

        cell.borrow_mut().update_options(&region_numbers);
    }
}

fn region_numbers(cell: &BoardCell, board: &Board) -> Vec<Number> {
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

fn fill_empty_cells(board: &mut Board) {
    let mut columns = board.columns();
    let mut rows = board.rows();
    let mut regions = board.regions();

    fill_region(&mut columns);
    clear_duplicates(&columns, &rows, &regions);
    fill_region(&mut rows);
    clear_duplicates(&columns, &rows, &regions);
    fill_region(&mut regions);
    clear_duplicates(&columns, &rows, &regions);
}

fn fill_region(regions: &mut Vec<Region>) {
    for region in regions.iter_mut() {
        let unsolved_cells: Vec<_> = region
            .cells
            .iter()
            .filter(|x| x.borrow().num == Number::N0)
            .collect();
        let solved_numbers: Vec<_> = region
            .cells
            .iter()
            .filter(|x| x.borrow().num != Number::N0)
            .map(|x| x.borrow().num)
            .collect();

        let mut unused_numbers: Vec<Number> = Number::all()
            .clone()
            .into_iter()
            .filter(|x| !solved_numbers.contains(x))
            .collect();
        unused_numbers.shuffle(&mut thread_rng());

        for x in 0..unsolved_cells.len() {
            let cell = unsolved_cells.get(x).unwrap();
            let mut cell = cell.borrow_mut();

            let num = unused_numbers.get(x);
            if num.is_some() {
                let num = num.unwrap().to_owned();
                cell.update(num);
            }
        }

        if unsolved_cells.len() > 0 {
            &region.recalc_options();
        }
    }
}

fn clear_duplicates(
    columns: &Vec<Region>,
    rows: &Vec<Region>,
    regions: &Vec<Region>,
) {
    clear_duplicates_region(columns);
    clear_duplicates_region(rows);
    clear_duplicates_region(regions);
}

fn clear_duplicates_region(regions: &Vec<Region>) {
    for region in regions {
        let filled_cells: Vec<_> = region
            .cells
            .iter()
            .filter(|x| x.borrow().num != Number::N0)
            .collect();

        for cell in filled_cells.clone().into_iter() {
            let duplicates: Vec<_> = filled_cells
                .iter()
                .filter(|x| x.borrow().num == cell.borrow().num)
                .collect();

            if duplicates.len() >= 2 {
                for duplicate in duplicates {
                    duplicate.borrow_mut().update(Number::N0);
                }
            }
        }
    }
}

fn unsolved_cells(board: &Board) -> Vec<&BoardCell> {
    board.cells.iter().filter(|x| !x.borrow().is_solved()).collect()
}
