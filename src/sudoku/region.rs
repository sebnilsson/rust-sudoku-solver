use super::*;

impl Region {
    pub fn new() -> Self {
        let cells: Vec<Cell> = Vec::new();
        Self { cells }
    }

    pub fn from(cells: &Vec<&Cell>) -> Self {
        let mut region = Region::new();
        for x in cells {
            region.cells.push(*x);
        }
        region
    }

    pub fn push_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }
}
