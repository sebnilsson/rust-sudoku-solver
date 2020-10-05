extern crate rand;

use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn solve(board: &mut Board, callback: fn(&Board)) {
    let mut last_unsolved_count = board.unsolved_count();

    loop {
        solve_board(board);

        let mut unsolved_count = board.unsolved_count();

        if unsolved_count == 0 {
            validate_solution(board);
            break;
        }

        if unsolved_count == last_unsolved_count {
            fill_empty_cells(board);

            unsolved_count = board.unsolved_count();

            if unsolved_count == 0 {
                validate_solution(board);
                break;
            }

            if unsolved_count == last_unsolved_count {
                panic!(
                    "Failed to improve unsolved cells. Stuck at: {}.",
                    unsolved_count
                );
            }
        }

        if unsolved_count > 0 {
            callback(&board);
        }

        last_unsolved_count = unsolved_count;
    }
}

fn validate_solution(_board: &Board) {
    // TODO: Implement
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

    let mut unsolved_cells: Vec<&BoardCell> =
        board.cells.iter().filter(|x| x.borrow().num == Number::N0).collect();
    unsolved_cells.sort_by_key(|x| x.borrow().options.len());

    fill_region(&unsolved_cells, &mut columns);
    fill_region(&unsolved_cells, &mut rows);
    fill_region(&unsolved_cells, &mut regions);

    // println!("=== fill_region ===");
    // println!("{}", board);
    // println!();

    clear_duplicates(&unsolved_cells, &mut columns, &mut rows, &mut regions);

    // println!("=== clear_duplicates ===");
    // println!("{}", board);
    // println!();
}

fn fill_region(unsolved_cells: &Vec<&BoardCell>, regions: &mut Vec<Region>) {
    for region in regions.iter_mut() {
        let solved_numbers: Vec<_> = region
            .cells
            .iter()
            .filter(|x| x.borrow().num != Number::N0)
            .map(|x| x.borrow().num)
            .collect();

        let unsolved_region_cells: Vec<&&BoardCell> = region
            .cells
            .iter()
            .filter(|x| {
                unsolved_cells
                    .iter()
                    .any(|u| u.borrow().index() == x.borrow().index())
            })
            .collect();

        let mut unused_numbers: Vec<Number> = Number::all()
            .clone()
            .into_iter()
            .filter(|x| !solved_numbers.contains(x))
            .collect();
        unused_numbers.shuffle(&mut thread_rng());

        for x in 0..unsolved_region_cells.len() {
            let cell = unsolved_region_cells.get(x).unwrap();
            let mut cell = cell.borrow_mut();

            let index = unused_numbers
                .iter()
                .position(|x| cell.options.iter().any(|c| c == x));
            if index.is_none() {
                continue;
            }

            let index = index.unwrap();
            let num = unused_numbers.remove(index);

            cell.update(num);
        }
    }

    recalc_options(regions);
}

fn clear_duplicates(
    unsolved_cells: &Vec<&BoardCell>,
    columns: &mut Vec<Region>,
    rows: &mut Vec<Region>,
    regions: &mut Vec<Region>,
) {
    clear_duplicates_region(unsolved_cells, columns);
    clear_duplicates_region(unsolved_cells, rows);
    clear_duplicates_region(unsolved_cells, regions);

    recalc_options_all(columns, rows, regions);
}

fn recalc_options_all(
    columns: &mut Vec<Region>,
    rows: &mut Vec<Region>,
    regions: &mut Vec<Region>,
) {
    recalc_options(columns);
    recalc_options(rows);
    recalc_options(regions);
}

fn recalc_options(regions: &mut Vec<Region>) {
    for region in regions.iter_mut() {
        region.recalc_options();
    }
}

fn clear_duplicates_region(
    unsolved_cells: &Vec<&BoardCell>,
    regions: &Vec<Region>,
) {
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
                    let is_unsolved_cell = unsolved_cells.iter().any(|x| {
                        x.borrow().index() == duplicate.borrow().index()
                    });

                    if is_unsolved_cell {
                        let mut duplicate = duplicate.borrow_mut();
                        duplicate.update(Number::N0);
                    }
                }
            }
        }
    }
}

fn unsolved_cells(board: &Board) -> Vec<&BoardCell> {
    board.cells.iter().filter(|x| !x.borrow().is_solved()).collect()
}
