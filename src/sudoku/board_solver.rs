extern crate rand;

use super::*;

pub fn solve(board: &mut Board, callback: fn(&Board)) {
    let mut last_unsolved_count = board.unsolved_count();

    loop {
        solve_board(board);
        //solve_cell(cell, board);

        let /*mut*/ unsolved_count = board.unsolved_count();

        if unsolved_count == 0 {
            break;
        }

        if unsolved_count == last_unsolved_count {
            // try_fill_empty_cells(&mut columns, &mut rows, &mut regions);

            // unsolved_count = board.unsolved_count();

            // if unsolved_count == last_unsolved_count {
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
    let cells: Vec<&mut Cell> = board.cells.iter_mut().collect();

    solve_cells(cells);
}

fn solve_cells(cells: Vec<&mut Cell>) {
    // let cell_list: Vec<&Cell> = cells.iter().map(|x|&*x).collect();
    // let c = *&cells;
    // for cell in cells {
    //     solve_cell(cell, cells);
    // }
}

fn solve_cell(cell: &mut Cell, cells: Vec<&Cell>) {
    //let cell_info = board_builder::get(cells, cell);
}

// fn try_fill_empty_cells(
//     columns: &mut Vec<Region>,
//     rows: &mut Vec<Region>,
//     regions: &mut Vec<Region>,
// ) {
//     try_fill_empty_cells_region(columns);
//     clear_invalid_cells(columns, rows, regions);
//     try_fill_empty_cells_region(rows);
//     clear_invalid_cells(columns, rows, regions);
//     try_fill_empty_cells_region(regions);
//     clear_invalid_cells(columns, rows, regions);
//     recalc_options(columns);
//     recalc_options(rows);
//     recalc_options(regions);
// }

// fn recalc_options(regions: &mut Vec<Region>) {
//     for region in regions {
//         region.recalc_options();
//     }
// }

// fn try_fill_empty_cells_region(regions: &mut Vec<Region>) {
//     for region in regions {
//         let unsolved_cells: Vec<&BoardCell> = region
//             .cells
//             .iter()
//             .filter(|x| x.borrow().num == Number::N0)
//             .collect();
//         let resolved_numbers: Vec<Number> = region
//             .cells
//             .iter()
//             .filter(|x| x.borrow().num != Number::N0)
//             .map(|x| x.borrow().num)
//             .collect();
//         let mut unused_numbers: Vec<Number> = Number::all()
//             .clone()
//             .into_iter()
//             .filter(|x| !resolved_numbers.contains(x))
//             .collect();
//         unused_numbers.shuffle(&mut thread_rng());

//         for x in 0..unsolved_cells.len() {
//             let cell = unsolved_cells.get(x).unwrap();
//             let mut cell = cell.borrow_mut();

//             let num = unused_numbers.get(x);
//             if num.is_some() {
//                 let num = Number::from_ref(num.unwrap());
//                 cell.update(num);
//             }
//         }
//     }
// }

// fn clear_invalid_cells(
//     columns: &mut Vec<Region>,
//     rows: &mut Vec<Region>,
//     regions: &mut Vec<Region>,
// ) {
//     clear_invalid_cells_region(columns);
//     clear_invalid_cells_region(rows);
//     clear_invalid_cells_region(regions);
// }

// fn clear_invalid_cells_region(regions: &mut Vec<Region>) {
//     for region in regions {
//         for cell in region.cells.iter() {
//             if cell.borrow().num == Number::N0 {
//                 continue;
//             }

//             let duplicates: Vec<&BoardCell> = region
//                 .cells
//                 .iter()
//                 .filter(|x| x.borrow().num == cell.borrow().num)
//                 .collect();
//             if duplicates.len() >= 2 {
//                 for duplicate in duplicates {
//                     duplicate.borrow_mut().update(Number::N0);
//                 }
//             }
//         }
//     }
// }

// fn solve_columns(columns: &mut Vec<Region>) {
//     solve_any_regions(columns);
// }

// fn solve_regions(regions: &mut Vec<Region>) {
//     solve_any_regions(regions);
// }

// fn solve_rows(rows: &mut Vec<Region>) {
//     solve_any_regions(rows);
// }

// fn solve_any_regions(regions: &mut Vec<Region>) {
//     for region in regions.iter_mut() {
//         let solved_cells: Vec<&BoardCell> =
//             region.cells.iter().filter(|x| x.borrow().is_solved()).collect();

//         let mut unsolved_cells: Vec<&BoardCell> =
//             region.cells.iter().filter(|x| !x.borrow().is_solved()).collect();

//         unsolved_cells.sort_by(|a, b| {
//             a.borrow().options.len().cmp(&b.borrow().options.len())
//         });

//         for cell in unsolved_cells.into_iter() {
//             remove_solved_cells(cell, &solved_cells);
//             try_solve_cell(&mut cell.borrow_mut());
//         }
//     }
// }

// fn remove_solved_cells(cell: &BoardCell, solved_cells: &Vec<&BoardCell>) {
//     let mut cell = cell.borrow_mut();
//     for solved in solved_cells {
//         let solved = solved.borrow();
//         cell.try_remove_option(&solved.num);
//     }
// }

// fn try_solve_cell(cell: &mut Cell) {
//     let mut num = Number::N0;
//     if cell.options.len() == 1 {
//         let option = cell.options.first().unwrap_or(&Number::N0);
//         num = Number::from_ref(option);
//     }

//     if num != Number::N0 {
//         cell.num = num;
//     }
// }
