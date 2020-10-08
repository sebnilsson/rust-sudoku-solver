extern crate rand;

use super::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn fill(board: &mut Board) {
    let mut board_info = BoardInfo::new(board);

    //fill_region(&board_info, &board_info.columns);
    fill_region(&board_info, &board_info.rows);
    //fill_region(&board_info, &board_info.regions);

    println!("||| fill_region |||");
    println!("{}", board);
    println!();

    clear_duplicates(&mut board_info);

    board.update_options();

    println!("||| clear_duplicates |||");
    println!("{}", board);
    println!();
}

fn fill_region(board_info: &BoardInfo, regions: &Vec<Region>) {
    for region in regions.iter() {
        let solved_numbers: Vec<_> = region
            .cells
            .iter()
            .filter(|x| x.borrow().num != Number::N0)
            .map(|x| x.borrow().num)
            .collect();

        let mut unsolved_cells: Vec<&&BoardCell> =
            region.cells.iter().filter(|x| !x.borrow().is_solved()).collect();
        unsolved_cells.sort_by_key(|x| x.borrow().options.len());

        let mut unused_numbers: Vec<Number> = Number::all()
            .clone()
            .into_iter()
            .filter(|x| !solved_numbers.contains(x))
            .collect();
        unused_numbers.shuffle(&mut thread_rng());

        for cell in unsolved_cells.iter() {
            let num;
            let x;
            let y;
            {
                let cell = cell.borrow();
                x = cell.x;
                y = cell.y;

                let index = unused_numbers
                    .iter()
                    .position(|x| cell.options.iter().any(|c| c == x));

                // num = match index {
                //     Some(index) => unused_numbers.remove(index),
                //     None => unused_numbers.pop()
                // };
                if index.is_some() {
                    let index = index.unwrap();
                    num = unused_numbers.remove(index);
                } else {
                    match unused_numbers.pop() {
                        Some(n) => {
                            num = n;
                        }
                        None => {
                            continue;
                        }
                    }
                }
            }

            let other_nums = board_info.other_nums(x, y);

            let mut cell = cell.borrow_mut();

            cell.set_num(&num);
            cell.update_options(other_nums);
        }
    }
}

fn clear_duplicates(board_info: &mut BoardInfo) {
    clear_duplicates_region(&board_info.columns);
    clear_duplicates_region(&board_info.rows);
    clear_duplicates_region(&board_info.regions);
}

fn clear_duplicates_region(regions: &Vec<Region>) {
    for region in regions {
        let filled_cells: Vec<_> = region
            .cells
            .iter()
            .filter(|x| x.borrow().num != Number::N0)
            .collect();

        for cell in filled_cells.iter() {
            let duplicate_cells: Vec<_> = filled_cells
                .iter()
                .filter(|x| x.borrow().num == cell.borrow().num)
                .collect();

            if duplicate_cells.len() >= 2 {
                for cell in duplicate_cells {
                    let mut cell = cell.borrow_mut();

                    cell.reset();
                }
            }
        }
    }
}
