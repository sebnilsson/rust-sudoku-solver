use super::*;

impl<'a> BoardInfo<'a> {
    pub fn new(board: &'a Board) -> BoardInfo {
        let mut columns = create_regions();
        let mut rows = create_regions();
        let mut subgrids = create_regions();

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_WIDTH {
                let coordinate = Coordinate::new(x, y);
                let cell = board.find_cell(coordinate);

                let row = rows
                    .get_mut(x as usize)
                    .expect("Failed to find row by index");

                let column = columns
                    .get_mut(y as usize)
                    .expect("Failed to find column by index");

                let region_index = region_index(cell);
                let subgrid = subgrids
                    .get_mut(region_index)
                    .expect("Failed to find region by index");

                row.cells.push(cell);
                column.cells.push(cell);
                subgrid.cells.push(cell);
            }
        }

        BoardInfo { board, columns, rows, subgrids }
    }

    pub fn cell_potentials(&self, cell: &BoardCell) -> Vec<Number> {
        cell_potentials(self, cell)
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

    pub fn nums(&self) -> Vec<Number> {
        self.board.cells.iter().map(|x| x.borrow().num.clone()).collect()
    }

    pub fn region_nums(&self, coordinate: Coordinate) -> Vec<Number> {
        region_nums(self, coordinate)
    }

    pub fn update_cell_region_potentials(&self, cell: &BoardCell) {
        update_cell_region_potentials(self, cell);
    }

    pub fn update_potentials(&mut self) {
        board_potentials(self);
    }
}

fn board_potentials(board_info: &mut BoardInfo) {
    for cell in board_info.board.cells.iter() {
        update_cell_potentials(board_info, cell);
    }
}

fn cell_potentials(board_info: &BoardInfo, cell: &BoardCell) -> Vec<Number> {
    let region_nums = region_nums(board_info, cell.borrow().coordinate);

    Number::all()
        .clone()
        .iter()
        .filter(|x| !region_nums.contains(x))
        .map(|x| x.clone())
        .collect()
}

fn create_regions<'a>() -> Vec<Region<'a>> {
    let mut regions: Vec<Region> = Vec::new();

    for _ in 0..BOARD_WIDTH {
        regions.push(Region::new());
    }

    regions
}

fn find_region<'a>(
    regions: &'a Vec<Region>,
    coordinate: Coordinate,
) -> &'a Region<'a> {
    let region = regions.into_iter().find(|region| {
        region
            .cells
            .iter()
            .map(|x| x.borrow())
            .any(|cell| cell.coordinate == coordinate)
    });

    match region {
        Some(region) => region,
        None => panic!("Failed to find correct region"),
    }
}

fn regions_cells<'a>(
    board_info: &'a BoardInfo,
    coordinate: Coordinate,
) -> Vec<&'a BoardCell> {
    let column = board_info.find_column(coordinate);
    let row = board_info.find_row(coordinate);
    let subgrid = board_info.find_subgrid(coordinate);

    column
        .cells
        .iter()
        .chain(row.cells.iter().chain(subgrid.cells.iter()))
        .map(|x| *x)
        .collect()
}

fn region_nums(board_info: &BoardInfo, coordinate: Coordinate) -> Vec<Number> {
    let region_cells = regions_cells(board_info, coordinate);

    let mut region_cell_nums: Vec<Number> =
        region_cells.iter().map(|x| x.borrow().num).collect();
    region_cell_nums.sort();
    region_cell_nums.dedup();
    region_cell_nums
}

fn region_index(cell: &BoardCell) -> usize {
    let cell = cell.borrow();
    let coordinate = cell.coordinate;
    let x = coordinate.x;
    let y = coordinate.y;

    if x < 3 {
        if y < 3 {
            0
        } else if y < 6 {
            1
        } else {
            2
        }
    } else if x < 6 {
        if y < 3 {
            3
        } else if y < 6 {
            4
        } else {
            5
        }
    } else {
        if y < 3 {
            6
        } else if y < 6 {
            7
        } else {
            8
        }
    }
}

fn update_cell_region_potentials(board_info: &BoardInfo, cell: &BoardCell) {
    let regions_cells = regions_cells(board_info, cell.borrow().coordinate);
    let regions_cells: Vec<&&BoardCell> =
        regions_cells.iter().filter(|x| !x.borrow().template).collect();

    for cell in regions_cells {
        update_cell_potentials(board_info, cell);
    }
}

fn update_cell_potentials(board_info: &BoardInfo, cell: &BoardCell) {
    let coordinate;
    {
        let cell = cell.borrow_mut();
        coordinate = Coordinate::new(cell.coordinate.x, cell.coordinate.y);
    }

    let region_nums = board_info.region_nums(coordinate);

    let mut cell = cell.borrow_mut();
    cell.update_potentials(region_nums);
}
