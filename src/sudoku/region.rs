use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }

    pub fn recalc_options(&mut self) {
        let solved_cells: Vec<_> = self
            .cells
            .iter()
            .filter(|x| x.borrow().num != Number::N0)
            .collect();
        let unsolved_cells: Vec<_> = self
            .cells
            .iter()
            .filter(|x| x.borrow().num == Number::N0)
            .collect();

        let mut solved_nums: Vec<Number> =
            solved_cells.iter().map(|x| x.borrow().num).collect();
        solved_nums.sort();
        solved_nums.dedup();

        for cell in unsolved_cells {
            cell.borrow_mut().update_options(&solved_nums);
        }
    }
}
