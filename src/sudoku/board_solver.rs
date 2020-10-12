extern crate rand;

use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn solve(board: &mut Board, context: &SolveContext) {
    let mut board_info = BoardInfo::new(board);

    //let last_unsolved_count = board.unsolved_count();
    let mut last_board_nums = Vec::new(); //board_info.nums();

    loop {
        solve_board(&board_info);

        let unsolved_count = board.unsolved_count();

        if unsolved_count == 0 {
            break;
        }

        let is_identical = is_board_identical(&board_info, &last_board_nums);
        if is_identical {
            // let is_identical =
            //     is_board_identical(&board_info, &last_board_nums);
            // if is_identical {
            //     let unsolved_count = board.unsolved_count();
            //     panic!(
            //         "Failed to improve unsolved cells. Stuck at: {}.",
            //         unsolved_count
            //     );
            // }

            clear_board(&mut board_info);

            last_board_nums = Vec::new();
            println!("--- Board cleared ---");
            println!();
            continue;

            //fill_empty_cells(&mut board_info);

            //(context.callback)(&board);

            //shift_cell_nums(&board_info);
        }

        reset_duplicates(&board_info);

        last_board_nums = board_info.nums();

        (context.callback)(&board);
    }

    (context.complete_callback)(&board);
}

fn clear_board(board_info: &mut BoardInfo) {
    let solved_cells = solved_cells(board_info);
    for cell in solved_cells {
        cell.borrow_mut().reset();
    }

    board_info.update_potentials();
}

// fn fill_empty_cells(board_info: &BoardInfo) {
//     for subgrid in board_info.subgrids.iter() {
//         let empty_cells: Vec<&&BoardCell> =
//             subgrid.cells.iter().filter(|x| x.borrow().is_empty()).collect();

//         for cell in empty_cells {
//             let region_nums = subgrid.nums();
//             let available_nums = Number::all();
//             let mut available_nums: Vec<&Number> = available_nums
//                 .iter()
//                 .clone()
//                 .filter(|x| !region_nums.contains(x))
//                 .collect();
//             available_nums.shuffle(&mut thread_rng());

//             let num = available_nums.first();

//             match num {
//                 Some(x) => cell.borrow_mut().set_num(x),
//                 _ => {}
//             }
//         }
//     }
// }

fn is_board_identical(
    board_info: &BoardInfo,
    last_board_nums: &Vec<Number>,
) -> bool {
    let board_nums = board_info.nums();
    &board_nums == last_board_nums
}

fn solve_board(board_info: &BoardInfo) {
    let mut unsolved_cells = unsolved_cells(board_info);

    while unsolved_cells.len() > 0 {
        unsolved_cells.sort_by_key(|x| x.borrow().potentials.len());
        unsolved_cells.reverse();

        let cell = unsolved_cells.pop().unwrap();
        solve_cell(&cell, board_info);
    }
}

fn solve_cell(cell: &BoardCell, board_info: &BoardInfo) {
    let mut cell_potentials = board_info.cell_potentials(cell);
    cell_potentials.shuffle(&mut thread_rng());

    let cell_potential = cell_potentials.first();
    if cell_potential.is_none() {
        return;
    }

    let cell_potential = cell_potential.unwrap();
    cell.borrow_mut().set_num(cell_potential);

    board_info.update_cell_region_potentials(cell);
}

fn reset_duplicates(board_info: &BoardInfo) {
    let row_duplicates = duplicate_region_cells(&board_info.rows);
    let column_duplicates = duplicate_region_cells(&board_info.columns);
    let subgrid_duplicates = duplicate_region_cells(&board_info.subgrids);

    let mut duplicates: Vec<&&BoardCell> = row_duplicates
        .iter()
        .chain(column_duplicates.iter().chain(subgrid_duplicates.iter()))
        .collect();
    duplicates.sort_by_key(|x| x.borrow().coordinate);
    duplicates.dedup();

    for cell in duplicates {
        cell.borrow_mut().reset();
    }
}

fn duplicate_region_cells<'a>(regions: &'a Vec<Region>) -> Vec<&'a BoardCell> {
    let mut duplicates = Vec::new();

    for region in regions {
        let filled_cells: Vec<&&BoardCell> = region
            .cells
            .iter()
            .filter(|x| {
                let cell = x.borrow();
                !cell.template && cell.num != Number::N0
            })
            .collect();

        for filled_cell_ref in filled_cells {
            let filled_cell = filled_cell_ref.borrow();

            let is_duplicate = region.cells.iter().any(|x| {
                let cell = x.borrow();
                cell.coordinate != filled_cell.coordinate
                    && cell.num == filled_cell.num
            });
            if is_duplicate {
                duplicates.push(*filled_cell_ref);
            }
        }
    }

    duplicates
}

fn solved_cells<'a>(board_info: &'a BoardInfo) -> Vec<&'a BoardCell> {
    let mut rows: Vec<&Region> = board_info.rows.iter().collect();
    rows.shuffle(&mut thread_rng());

    let mut solved_cells = Vec::new();

    for row in rows {
        for ref_cell in row.cells.iter() {
            let cell = ref_cell.borrow();

            if !cell.template && cell.is_solved() {
                solved_cells.push(*ref_cell);
            }
        }
    }

    solved_cells
}

fn unsolved_cells<'a>(board_info: &'a BoardInfo) -> Vec<&'a BoardCell> {
    let mut rows: Vec<&Region> = board_info.rows.iter().collect();
    rows.shuffle(&mut thread_rng());

    let mut unsolved_cells = Vec::new();

    for row in rows {
        for ref_cell in row.cells.iter() {
            let cell = ref_cell.borrow();

            if !cell.template && !cell.is_solved() {
                unsolved_cells.push(*ref_cell);
            }
        }
    }

    unsolved_cells
}
