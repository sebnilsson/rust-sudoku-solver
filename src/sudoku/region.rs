use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }
}

impl<'a> Region<'a> {
    pub fn for_board() -> Vec<Region<'a>> {
        let mut regions: Vec<Region> = Vec::new();

        for _ in 0..BOARD_WIDTH {
            regions.push(Region::new());
        }

        regions
    }
}
