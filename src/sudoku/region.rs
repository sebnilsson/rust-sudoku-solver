use super::*;

impl Region {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }
}

impl Region {
    pub fn for_board() -> Vec<Region> {
        (0..BOARD_WIDTH).map(|_| Region::new()).collect()
    }
}
