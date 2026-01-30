use super::*;

impl Region {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::with_capacity(BOARD_WIDTH as usize);
        Self { cells }
    }
}

impl Region {
    pub fn for_board() -> Vec<Region> {
        let mut regions = Vec::with_capacity(BOARD_WIDTH as usize);
        for _ in 0..BOARD_WIDTH {
            regions.push(Region::new());
        }
        regions
    }
}
