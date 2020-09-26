use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<&_> = Vec::new();
        Self { cells }
    }

    pub fn from(cells: Vec<&'a BoardCell>) -> Self {
        let mut region = Region::new();
        for x in cells {
            region.cells.push(x);
        }
        region
    }
}
