use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<&'a Cell> = Vec::new();
        Self { cells }
    }

    pub fn from(cells: Vec<&'a Cell>) -> Self {
        let mut region = Region::new();
        for x in cells {
            region.cells.push(x);
        }
        region
    }

    pub fn push_cell(&mut self, cell: &'a Cell) {
        self.cells.push(cell);
    }
}
