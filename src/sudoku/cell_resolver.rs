use super::*;

pub fn columns<'a>(cells: &'a Vec<Cell>) -> Vec<Region<'a>> {
    let mut rows = Vec::new();

    for x in 0..BOARD_WIDTH {
        let mut region = Region::new();

        for y in 0..BOARD_WIDTH {
            let index = Board::index(x, y);
            let cell = cells.get(index);

            match cell {
                Some(cell) => region.cells.push(cell),
                _ => {}
            }
        }

        rows.push(region);
    }

    rows
}

pub fn regions<'a>(cells: &'a Vec<Cell>) -> Vec<Region<'a>> {
    let mut map: Vec<(&Cell, usize)> =
        cells.iter().map(|x| (x, region_index(x))).collect();
    map.sort_by_key(|x| x.1);

    let mut indexes: Vec<usize> = map.iter().map(|x| x.1).collect();
    indexes.dedup();

    let mut regions = Vec::new();
    for index in indexes {
        let mut region = Region::new();

        let cells: Vec<&Cell> =
            map.iter().filter(|x| x.1 == index).map(|x| x.0).collect();

        for cell in cells {
            region.cells.push(cell);
        }

        regions.push(region);
    }
    regions
}

pub fn rows<'a>(cells: &'a Vec<Cell>) -> Vec<Region<'a>> {
    let mut rows = Vec::new();

    for y in 0..BOARD_WIDTH {
        let mut region = Region::new();

        for x in 0..BOARD_WIDTH {
            let index = Board::index(x, y);
            let cell = cells.get(index);

            match cell {
                Some(cell) => region.cells.push(cell),
                _ => {}
            }
        }

        rows.push(region);
    }

    rows
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
