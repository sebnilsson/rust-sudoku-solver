use super::*;

impl BoardInfo {
    pub fn new() -> Self {
        let mut columns = Region::for_board();
        let mut rows = Region::for_board();
        let mut subgrids = Region::for_board();

        for y in 0..BOARD_WIDTH {
            for x in 0..BOARD_WIDTH {
                let coordinate = Coordinate::new(x, y);
                let row = rows
                    .get_mut(y as usize)
                    .expect("Failed to find row by index");

                let column = columns
                    .get_mut(x as usize)
                    .expect("Failed to find column by index");

                let subgrid_index = subgrid_index(coordinate);
                let subgrid = subgrids
                    .get_mut(subgrid_index)
                    .expect("Failed to find region by index");

                row.cells.push(coordinate);
                column.cells.push(coordinate);
                subgrid.cells.push(coordinate);
            }
        }

        BoardInfo { columns, rows, subgrids }
    }

    pub fn cell_potentials(&self, board: &Board, cell: &BoardCell) -> Vec<Number> {
        cell_potentials(self, board, cell)
    }

    pub fn find_column(&self, coordinate: Coordinate) -> &Region {
        find_region(&self.columns, coordinate)
    }

    pub fn find_row(&self, coordinate: Coordinate) -> &Region {
        find_region(&self.rows, coordinate)
    }

    pub fn find_subgrid(&self, coordinate: Coordinate) -> &Region {
        find_region(&self.subgrids, coordinate)
    }
}

fn cell_potentials(board_info: &BoardInfo, board: &Board, cell: &BoardCell) -> Vec<Number> {
    let region_nums = region_nums(board_info, board, cell.borrow().coordinate);

    Number::all()
        .iter()
        .copied()
        .filter(|x| !region_nums.contains(x))
        .collect()
}

fn find_region(regions: &[Region], coordinate: Coordinate) -> &Region {
    let region = regions.iter().find(|region| {
        region
            .cells
            .iter()
            .any(|cell| *cell == coordinate)
    });

    match region {
        Some(region) => region,
        None => panic!("Failed to find correct region"),
    }
}

fn region_nums(board_info: &BoardInfo, board: &Board, coordinate: Coordinate) -> Vec<Number> {
    let column = board_info.find_column(coordinate);
    let row = board_info.find_row(coordinate);
    let subgrid = board_info.find_subgrid(coordinate);

    let mut present = [false; 10];

    for coord in column
        .cells
        .iter()
        .chain(row.cells.iter())
        .chain(subgrid.cells.iter())
    {
        let num = board.find_cell(*coord).borrow().num;
        let idx = num.to_usize();
        if idx > 0 {
            present[idx] = true;
        }
    }

    Number::all()
        .iter()
        .copied()
        .filter(|num| present[num.to_usize()])
        .collect()
}

fn subgrid_index(coordinate: Coordinate) -> usize {
    let x = coordinate.x;
    let y = coordinate.y;

    ((y / 3) * 3 + (x / 3)) as usize
}
