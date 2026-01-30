use super::*;

impl Region<'_> {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }
}

impl<'a> Region<'a> {
    pub fn for_board() -> Vec<Region<'a>> {
        (0..BOARD_WIDTH).map(|_| Region::new()).collect()
    }
}
