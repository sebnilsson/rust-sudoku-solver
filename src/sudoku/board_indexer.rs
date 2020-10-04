use super::*;

pub fn get<'a>(cells: &'a Vec<Cell>, cell: &Cell) -> CellInfo<'a> {
    let column = column(cells, cell);
    let row = row(cells, cell);
    let region = region(cells, cell);

    CellInfo { column, row, region }
}

pub fn index(x: u8, y: u8) -> usize {
    (x + (y * BOARD_WIDTH)) as usize
}

pub fn rows<'a>(board: &'a Board) -> Vec<Region<'a>> {
    let mut rows = Vec::new();

    for x in 0..BOARD_WIDTH {
        let mut region = Region::new();

        for y in 0..BOARD_WIDTH {
            let index = index(x, y);
            let cell = board.cells.get(index);

            match cell {
                Some(cell) => region.cells.push(cell),
                _ => {}
            }
        }

        rows.push(region);
    }

    rows
}

fn column<'a>(cells: &'a Vec<Cell>, cell: &Cell) -> Region<'a> {
    let index = column_index(cell) as u8;
    let mut region = Region::new();

    region.cells = cells.iter().filter(|x| x.y == index).collect();

    region
}

fn row<'a>(cells: &'a Vec<Cell>, cell: &Cell) -> Region<'a> {
    let index = row_index(cell) as u8;
    let mut region = Region::new();

    region.cells = cells.iter().filter(|x| x.x == index).collect();

    region
}

fn region<'a>(cells: &'a Vec<Cell>, cell: &Cell) -> Region<'a> {
    let index = region_index(cell) as u8;
    let mut region = Region::new();

    region.cells =
        cells.iter().filter(|x| region_predicate(x, index)).collect();

    region
}

fn region_predicate(cell: &&Cell, index: u8) -> bool {
    match index {
        0 => cell.x < 3 && cell.y < 3,
        1 => cell.x >= 3 && cell.x < 6 && cell.y < 3,
        2 => cell.x >= 6 && cell.x < 9 && cell.y < 3,
        3 => cell.x < 3 && cell.y >= 3 && cell.y < 6,
        4 => cell.x >= 3 && cell.x < 6 && cell.y >= 3 && cell.y < 6,
        5 => cell.x >= 6 && cell.x < 9 && cell.y >= 3 && cell.y < 6,
        6 => cell.x < 3 && cell.y >= 6 && cell.y < 9,
        7 => cell.x >= 3 && cell.x < 6 && cell.y >= 6 && cell.y < 9,
        8 => cell.x >= 6 && cell.x < 9 && cell.y >= 6 && cell.y < 9,
        _ => false,
    }
}

fn column_index(cell: &Cell) -> usize {
    cell.y as usize
}

fn row_index(cell: &Cell) -> usize {
    cell.x as usize
}

fn region_index(cell: &Cell) -> usize {
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
