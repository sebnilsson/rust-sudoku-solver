use super::*;

impl<'a> BoardInfo<'a> {
    pub fn new(board: &'a Board) -> BoardInfo {
        let mut columns = create_regions();
        let mut rows = create_regions();
        let mut subgrids = create_regions();

        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_WIDTH {
                let cell = board.find_cell(x, y);

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

    pub fn region_nums(&self, x: u8, y: u8) -> Vec<Number> {
        region_nums(self, x, y)
    }

    pub fn find_column(&self, x: u8, y: u8) -> &Region {
        find_region(&self.columns, x, y)
    }

    pub fn find_row(&self, x: u8, y: u8) -> &Region {
        find_region(&self.rows, x, y)
    }

    pub fn find_subgrid(&self, x: u8, y: u8) -> &Region {
        find_region(&self.subgrids, x, y)
    }
}

fn create_regions<'a>() -> Vec<Region<'a>> {
    let mut regions: Vec<Region> = Vec::new();

    for _ in 0..BOARD_WIDTH {
        regions.push(Region::new());
    }

    regions
}

fn find_region<'a>(regions: &'a Vec<Region>, x: u8, y: u8) -> &'a Region<'a> {
    let region = regions.into_iter().find(|region| {
        region
            .cells
            .iter()
            .map(|x| x.borrow())
            .any(|cell| cell.x == x && cell.y == y)
    });

    match region {
        Some(region) => region,
        None => panic!("Failed to find correct region"),
    }
}

fn region_nums(board_info: &BoardInfo, x: u8, y: u8) -> Vec<Number> {
    let column = board_info.find_column(x, y);
    let row = board_info.find_row(x, y);
    let subgrid = board_info.find_subgrid(x, y);

    let region_cells =
        column.cells.iter().chain(row.cells.iter().chain(subgrid.cells.iter()));

    let mut region_cell_nums: Vec<Number> =
        region_cells.map(|x| x.borrow().num).collect();
    region_cell_nums.sort();
    region_cell_nums.dedup();
    region_cell_nums
}

fn region_index(cell: &BoardCell) -> usize {
    let cell = cell.borrow();
    let x = cell.x;
    let y = cell.y;

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
